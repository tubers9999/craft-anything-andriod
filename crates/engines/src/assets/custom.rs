//! Custom / OpenAI-compatible image generation API (DALL-E, Stable Diffusion, ComfyUI, LocalAI, Ollama, etc.)

use super::{AssetProvider, AssetRequest, Generated, Shape};
use craft_anything_evidence::{ErrorCode, ToolError};
use serde_json::{Value, json};
use std::io::Read as _;
use std::time::Duration;

pub const ID: &str = "custom";
const MAX_BYTES: u64 = 50 * 1024 * 1024;

/// Custom OpenAI-compatible image generation provider.
#[derive(Debug, Clone)]
pub struct CustomImage {
    pub default_base: String,
}

impl Default for CustomImage {
    fn default() -> CustomImage {
        CustomImage {
            default_base: "https://api.openai.com/v1".to_string(),
        }
    }
}

impl AssetProvider for CustomImage {
    fn id(&self) -> &'static str {
        ID
    }

    fn name(&self) -> &'static str {
        "Custom Image API (OpenAI / Local / SD / ComfyUI)"
    }

    fn key_env(&self) -> Option<&'static str> {
        Some("IMAGE_API_KEY")
    }

    fn keys_page(&self) -> Option<&'static str> {
        Some("https://platform.openai.com/api-keys")
    }

    fn generate(&self, key: &str, req: &AssetRequest) -> Result<Generated, ToolError> {
        let key = key.trim();
        // Support:
        // 1. "API_KEY" -> uses default_base
        // 2. "http://endpoint" -> custom base, no key
        // 3. "API_KEY@http://endpoint" -> custom key and custom base
        let (api_key, base_url) = if let Some((k, url)) = key.split_once('@') {
            (k.trim(), url.trim())
        } else if key.starts_with("http://") || key.starts_with("https://") {
            ("", key)
        } else {
            (key, self.default_base.as_str())
        };

        match *req {
            AssetRequest::Image { prompt, shape, .. } => {
                let size_str = match shape {
                    Shape::Square => "1024x1024",
                    Shape::Wide => "1792x1024",
                    Shape::Tall => "1024x1792",
                };
                let body = json!({
                    "prompt": prompt,
                    "n": 1,
                    "size": size_str,
                    "response_format": "b64_json"
                });

                let endpoint = if base_url.ends_with("/images/generations") {
                    base_url.to_string()
                } else {
                    format!("{}/images/generations", base_url.trim_end_matches('/'))
                };

                let mut req_builder = agent(Duration::from_secs(180))
                    .post(&endpoint)
                    .header("Content-Type", "application/json");

                if !api_key.is_empty() {
                    req_builder = req_builder.header("Authorization", &format!("Bearer {api_key}"));
                }

                let resp = req_builder
                    .send(body.to_string())
                    .map_err(|e| {
                        ToolError::new(ErrorCode::EngineMissing, format!("Couldn't reach image API ({endpoint}): {e}"))
                            .with_remediation("Check your custom image API endpoint address and network connection.")
                    })?;

                let status = resp.status().as_u16();
                let text = resp.into_body().read_to_string().unwrap_or_default();
                if !(200..=299).contains(&status) {
                    return Err(ToolError::new(ErrorCode::ProgramFailed, format!("Image API answered {status}: {text}"))
                        .with_remediation("Check the API key or endpoint address. Format can be: API_KEY or API_KEY@http://endpoint"));
                }

                let val: Value = serde_json::from_str(&text)
                    .map_err(|e| ToolError::new(ErrorCode::GarbledOutput, format!("Image API sent invalid JSON: {e}")))?;

                // Look for b64_json first in data[0]
                if let Some(b64) = val.pointer("/data/0/b64_json").and_then(Value::as_str) {
                    let bytes = base64_decode(b64)?;
                    return Ok(Generated { url: "data:image/png;base64".to_string(), bytes, cost_usd: req.listed_cost() });
                }

                // Otherwise look for url in data[0]
                if let Some(img_url) = val.pointer("/data/0/url").and_then(Value::as_str) {
                    let bytes = download(img_url)?;
                    return Ok(Generated { url: img_url.to_string(), bytes, cost_usd: req.listed_cost() });
                }

                Err(ToolError::new(ErrorCode::GarbledOutput, "Image API returned neither b64_json nor url in data[0].")
                    .with_remediation("Check that the custom endpoint implements OpenAI /images/generations."))
            }
            AssetRequest::Model3d { .. } => {
                Err(ToolError::new(ErrorCode::Unsupported, "3D model generation currently requires Fal.ai (Tripo3D).").with_remediation("Use fal.ai key for 3D models."))
            }
        }
    }
}

fn agent(timeout: Duration) -> ureq::Agent {
    ureq::Agent::config_builder()
        .http_status_as_error(false)
        .user_agent("CraftAnything")
        .timeout_connect(Some(Duration::from_secs(20)))
        .timeout_recv_response(Some(timeout))
        .timeout_recv_body(Some(Duration::from_secs(10 * 60)))
        .build()
        .new_agent()
}

fn download(url: &str) -> Result<Vec<u8>, ToolError> {
    if let Some(data) = url.strip_prefix("data:") {
        let b64 = data.split_once(";base64,").map(|(_, b)| b)
            .ok_or_else(|| ToolError::new(ErrorCode::GarbledOutput, "Invalid data URI."))?;
        return base64_decode(b64);
    }
    let resp = agent(Duration::from_secs(120)).get(url).call()
        .map_err(|e| ToolError::new(ErrorCode::EngineMissing, format!("Couldn't download generated image: {e}")))?;
    if !resp.status().is_success() {
        return Err(ToolError::new(ErrorCode::EngineMissing, format!("Downloading generated image failed ({}).", resp.status().as_u16())));
    }
    let mut bytes = Vec::new();
    resp.into_body().into_reader().take(MAX_BYTES + 1).read_to_end(&mut bytes)
        .map_err(|e| ToolError::new(ErrorCode::EngineMissing, format!("Image download interrupted: {e}")))?;
    if bytes.len() as u64 > MAX_BYTES {
        return Err(ToolError::new(ErrorCode::OverLimit, "Generated image is too large."));
    }
    Ok(bytes)
}

fn base64_decode(s: &str) -> Result<Vec<u8>, ToolError> {
    use base64::Engine as _;
    base64::engine::general_purpose::STANDARD
        .decode(s.trim())
        .map_err(|e| ToolError::new(ErrorCode::GarbledOutput, format!("Couldn't decode base64 image data: {e}")))
}

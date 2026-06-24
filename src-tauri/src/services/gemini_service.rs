use std::fs;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri::Manager;

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: GeminiContent,
}

#[derive(Deserialize)]
struct GeminiContent {
    parts: Vec<Part>,
}

#[derive(Deserialize)]
struct Part {
    text: String,
}

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<RequestContent>,
}

#[derive(Serialize)]
struct RequestContent {
    parts: Vec<RequestPart>,
}

#[derive(Serialize)]
struct RequestPart {
    text: String,
}

fn config_path(app: &AppHandle) -> Result<std::path::PathBuf, String> {
    let app_data = app.path().app_data_dir().map_err(|e| format!("Gagal akses app data: {}", e))?;
    fs::create_dir_all(&app_data).map_err(|e| format!("Gagal buat direktori config: {}", e))?;
    Ok(app_data.join("kualitas_air_config.json"))
}

pub fn get_api_key(app: &AppHandle) -> Result<String, String> {
    let path = config_path(app)?;
    if !path.exists() {
        return Err("API Key Gemini belum diatur.".to_string());
    }
    let content = fs::read_to_string(&path).map_err(|e| format!("Gagal baca config: {}", e))?;
    let json: serde_json::Value = serde_json::from_str(&content).map_err(|e| format!("Config format salah: {}", e))?;
    json.get("gemini_api_key")
        .and_then(|k| k.as_str())
        .filter(|k| !k.is_empty())
        .map(|k| k.to_string())
        .ok_or_else(|| "API Key belum diatur. Silakan masukkan API Key di menu Pengaturan.".to_string())
}

pub fn is_key_configured(app: &AppHandle) -> bool {
    let path = match config_path(app) {
        Ok(p) => p,
        Err(_) => return false,
    };
    if !path.exists() {
        return false;
    }
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return false,
    };
    let json: serde_json::Value = match serde_json::from_str(&content) {
        Ok(j) => j,
        Err(_) => return false,
    };
    json.get("gemini_api_key")
        .and_then(|k| k.as_str())
        .is_some_and(|k| !k.is_empty())
}

pub fn save_api_key(app: &AppHandle, key: &str) -> Result<(), String> {
    let path = config_path(app)?;
    let config = serde_json::json!({ "gemini_api_key": key });
    fs::write(&path, serde_json::to_string_pretty(&config).unwrap())
        .map_err(|e| format!("Gagal menyimpan API key: {}", e))?;
    println!("✅ API Key tersimpan di: {:?}", path);
    Ok(())
}

pub async fn generate(app: &AppHandle, prompt: &str) -> Result<String, String> {
    let api_key = get_api_key(app)?;

    let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-flash-latest:generateContent";

    let client = reqwest::Client::new();
    let request_body = GeminiRequest {
        contents: vec![RequestContent {
            parts: vec![RequestPart {
                text: prompt.to_string(),
            }],
        }],
    };

    let response = client
        .post(url)
        .header("X-goog-api-key", &api_key)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Gagal menghubungi Gemini API: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Gemini API error ({}): {}", status, body));
    }

    let gemini: GeminiResponse = response
        .json()
        .await
        .map_err(|e| format!("Gagal parse response Gemini: {}", e))?;

    let text = gemini.candidates
        .first()
        .and_then(|c| c.content.parts.first())
        .map(|p| p.text.clone())
        .ok_or_else(|| "Gemini tidak mengembalikan teks".to_string())?;

    Ok(text)
}

use crate::config::LicenseOptions;
use crate::error::{LicenseError, Result};
use crate::types::{CheckoutResult, LicenseResult, ProductTier};
use reqwest::blocking::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

pub struct Transport {
    base_url: String,
    public_key: String,
    product_slug: String,
    debug: bool,
    http_client: HttpClient,
    machine_id: String,
}

#[derive(Serialize)]
struct ValidateRequest {
    #[serde(rename = "licenseKey")]
    license_key: String,
    #[serde(rename = "machineId")]
    machine_id: String,
}

#[derive(Serialize)]
struct ActivateRequest {
    #[serde(rename = "licenseKey")]
    license_key: String,
    #[serde(rename = "machineId")]
    machine_id: String,
    #[serde(rename = "machineName")]
    machine_name: String,
    platform: String,
}

#[derive(Serialize)]
struct DeactivateRequest {
    #[serde(rename = "licenseKey")]
    license_key: String,
    #[serde(rename = "machineId")]
    machine_id: String,
}

#[derive(Serialize)]
struct TrialRequest {
    email: String,
    #[serde(rename = "machineId")]
    machine_id: String,
}

#[derive(Serialize)]
struct CheckoutRequest {
    #[serde(rename = "tierId")]
    tier_id: String,
    email: String,
}

#[derive(Deserialize)]
struct TiersResponse {
    tiers: Vec<ProductTier>,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}

impl Transport {
    pub fn new(options: &LicenseOptions) -> Self {
        let http_client = HttpClient::builder()
            .timeout(options.http_timeout)
            .build()
            .unwrap_or_default();

        let machine_id = Self::get_or_create_machine_id();

        Self {
            base_url: options.api_base_url.clone(),
            public_key: options.public_key.clone(),
            product_slug: options.product_slug.clone(),
            debug: options.debug,
            http_client,
            machine_id,
        }
    }

    fn log(&self, msg: &str) {
        if self.debug {
            println!("[IronLicensing] {}", msg);
        }
    }

    fn get_machine_id_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".ironlicensing")
            .join("machine_id")
    }

    fn get_or_create_machine_id() -> String {
        let id_path = Self::get_machine_id_path();

        if let Ok(id) = fs::read_to_string(&id_path) {
            return id.trim().to_string();
        }

        let id = Uuid::new_v4().to_string();

        if let Some(parent) = id_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(&id_path, &id);

        id
    }

    pub fn machine_id(&self) -> &str {
        &self.machine_id
    }

    fn get_hostname() -> String {
        hostname::get()
            .map(|h| h.to_string_lossy().to_string())
            .unwrap_or_else(|_| "unknown".to_string())
    }

    fn get_platform() -> &'static str {
        if cfg!(target_os = "windows") {
            "windows"
        } else if cfg!(target_os = "macos") {
            "macos"
        } else if cfg!(target_os = "linux") {
            "linux"
        } else {
            "unknown"
        }
    }

    pub fn validate(&self, license_key: &str) -> LicenseResult {
        let preview = &license_key[..license_key.len().min(10)];
        self.log(&format!("Validating: {}...", preview));

        let request = ValidateRequest {
            license_key: license_key.to_string(),
            machine_id: self.machine_id.clone(),
        };

        self.post("/api/v1/validate", &request)
    }

    pub fn activate(&self, license_key: &str, machine_name: Option<&str>) -> LicenseResult {
        let preview = &license_key[..license_key.len().min(10)];
        self.log(&format!("Activating: {}...", preview));

        let machine_name = machine_name
            .map(String::from)
            .unwrap_or_else(Self::get_hostname);

        let request = ActivateRequest {
            license_key: license_key.to_string(),
            machine_id: self.machine_id.clone(),
            machine_name,
            platform: Self::get_platform().to_string(),
        };

        self.post("/api/v1/activate", &request)
    }

    pub fn deactivate(&self, license_key: &str) -> bool {
        self.log("Deactivating license");

        let request = DeactivateRequest {
            license_key: license_key.to_string(),
            machine_id: self.machine_id.clone(),
        };

        match self
            .http_client
            .post(format!("{}/api/v1/deactivate", self.base_url))
            .header("Content-Type", "application/json")
            .header("X-Public-Key", &self.public_key)
            .header("X-Product-Slug", &self.product_slug)
            .json(&request)
            .send()
        {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }

    pub fn start_trial(&self, email: &str) -> LicenseResult {
        self.log(&format!("Starting trial for: {}", email));

        let request = TrialRequest {
            email: email.to_string(),
            machine_id: self.machine_id.clone(),
        };

        self.post("/api/v1/trial", &request)
    }

    pub fn get_tiers(&self) -> Vec<ProductTier> {
        self.log("Fetching product tiers");

        match self
            .http_client
            .get(format!("{}/api/v1/tiers", self.base_url))
            .header("Content-Type", "application/json")
            .header("X-Public-Key", &self.public_key)
            .header("X-Product-Slug", &self.product_slug)
            .send()
        {
            Ok(resp) if resp.status().is_success() => resp
                .json::<TiersResponse>()
                .map(|r| r.tiers)
                .unwrap_or_default(),
            _ => vec![],
        }
    }

    pub fn start_checkout(&self, tier_id: &str, email: &str) -> CheckoutResult {
        self.log(&format!("Starting checkout for tier: {}", tier_id));

        let request = CheckoutRequest {
            tier_id: tier_id.to_string(),
            email: email.to_string(),
        };

        match self
            .http_client
            .post(format!("{}/api/v1/checkout", self.base_url))
            .header("Content-Type", "application/json")
            .header("X-Public-Key", &self.public_key)
            .header("X-Product-Slug", &self.product_slug)
            .json(&request)
            .send()
        {
            Ok(resp) => {
                let status = resp.status();
                let body = resp.text().unwrap_or_default();

                if status.is_success() {
                    match serde_json::from_str::<CheckoutResult>(&body) {
                        Ok(mut result) => {
                            result.success = true;
                            result
                        }
                        Err(e) => CheckoutResult::failure(e.to_string()),
                    }
                } else {
                    let error = serde_json::from_str::<ErrorResponse>(&body)
                        .map(|e| e.error)
                        .unwrap_or_else(|_| "Checkout failed".to_string());
                    CheckoutResult::failure(error)
                }
            }
            Err(e) => CheckoutResult::failure(e.to_string()),
        }
    }

    fn post<T: Serialize>(&self, path: &str, body: &T) -> LicenseResult {
        match self
            .http_client
            .post(format!("{}{}", self.base_url, path))
            .header("Content-Type", "application/json")
            .header("X-Public-Key", &self.public_key)
            .header("X-Product-Slug", &self.product_slug)
            .json(body)
            .send()
        {
            Ok(resp) => {
                let status = resp.status();
                let body = resp.text().unwrap_or_default();

                if status.is_success() {
                    serde_json::from_str(&body).unwrap_or_else(|e| LicenseResult::failure(e.to_string()))
                } else {
                    let error = serde_json::from_str::<ErrorResponse>(&body)
                        .map(|e| e.error)
                        .unwrap_or_else(|_| "Request failed".to_string());
                    LicenseResult::failure(error)
                }
            }
            Err(e) => LicenseResult::failure(e.to_string()),
        }
    }
}

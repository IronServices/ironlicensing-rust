//! IronLicensing SDK for Rust
//!
//! Official Rust SDK for [IronLicensing](https://ironlicensing.com) - Software licensing
//! and activation for your applications.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use ironlicensing::{LicenseClient, LicenseOptions};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a client
//!     let client = LicenseClient::with_credentials(
//!         "pk_live_your_public_key",
//!         "your-product-slug"
//!     )?;
//!
//!     // Validate a license
//!     let result = client.validate("IRON-XXXX-XXXX-XXXX-XXXX");
//!     if result.valid {
//!         println!("License is valid!");
//!     }
//!
//!     // Check for features
//!     if client.has_feature("premium") {
//!         println!("Premium features enabled!");
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! # Global Client
//!
//! For convenience, you can use a global client:
//!
//! ```rust,no_run
//! use ironlicensing::{init, validate, has_feature};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize the global client
//!     init("pk_live_your_public_key", "your-product-slug")?;
//!
//!     // Use global functions
//!     let result = validate("IRON-XXXX-XXXX-XXXX-XXXX")?;
//!     if result.valid {
//!         println!("Valid!");
//!     }
//!
//!     if has_feature("premium")? {
//!         println!("Premium enabled!");
//!     }
//!
//!     Ok(())
//! }
//! ```

mod client;
mod config;
mod error;
mod transport;
mod types;

pub use client::LicenseClient;
pub use config::LicenseOptions;
pub use error::{LicenseError, Result};
pub use types::*;

use once_cell::sync::OnceCell;
use std::sync::Arc;

static GLOBAL_CLIENT: OnceCell<Arc<LicenseClient>> = OnceCell::new();

/// Initialize the global IronLicensing client.
pub fn init(public_key: impl Into<String>, product_slug: impl Into<String>) -> Result<()> {
    init_with_options(LicenseOptions::new(public_key, product_slug))
}

/// Initialize the global client with custom options.
pub fn init_with_options(options: LicenseOptions) -> Result<()> {
    let client = LicenseClient::new(options)?;
    GLOBAL_CLIENT
        .set(Arc::new(client))
        .map_err(|_| LicenseError::Api("Client already initialized".to_string()))
}

/// Get the global client.
pub fn get_client() -> Result<&'static Arc<LicenseClient>> {
    GLOBAL_CLIENT.get().ok_or(LicenseError::NotInitialized)
}

/// Validate a license key using the global client.
pub fn validate(license_key: &str) -> Result<LicenseResult> {
    Ok(get_client()?.validate(license_key))
}

/// Activate a license key using the global client.
pub fn activate(license_key: &str) -> Result<LicenseResult> {
    Ok(get_client()?.activate(license_key))
}

/// Activate a license key with a machine name using the global client.
pub fn activate_with_name(license_key: &str, machine_name: Option<&str>) -> Result<LicenseResult> {
    Ok(get_client()?.activate_with_name(license_key, machine_name))
}

/// Deactivate the current license using the global client.
pub fn deactivate() -> Result<bool> {
    Ok(get_client()?.deactivate())
}

/// Start a trial using the global client.
pub fn start_trial(email: &str) -> Result<LicenseResult> {
    Ok(get_client()?.start_trial(email))
}

/// Check if a feature is available using the global client.
pub fn has_feature(feature_key: &str) -> Result<bool> {
    Ok(get_client()?.has_feature(feature_key))
}

/// Require a feature using the global client.
pub fn require_feature(feature_key: &str) -> Result<()> {
    get_client()?.require_feature(feature_key)
}

/// Get a feature using the global client.
pub fn get_feature(feature_key: &str) -> Result<Option<Feature>> {
    Ok(get_client()?.get_feature(feature_key))
}

/// Get the current license using the global client.
pub fn license() -> Result<Option<License>> {
    Ok(get_client()?.license())
}

/// Get the license status using the global client.
pub fn status() -> Result<LicenseStatus> {
    Ok(get_client()?.status())
}

/// Check if licensed using the global client.
pub fn is_licensed() -> Result<bool> {
    Ok(get_client()?.is_licensed())
}

/// Check if in trial mode using the global client.
pub fn is_trial() -> Result<bool> {
    Ok(get_client()?.is_trial())
}

/// Get available tiers using the global client.
pub fn get_tiers() -> Result<Vec<ProductTier>> {
    Ok(get_client()?.get_tiers())
}

/// Start a purchase using the global client.
pub fn start_purchase(tier_id: &str, email: &str) -> Result<CheckoutResult> {
    Ok(get_client()?.start_purchase(tier_id, email))
}

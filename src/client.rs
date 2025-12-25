use crate::config::LicenseOptions;
use crate::error::{LicenseError, Result};
use crate::transport::Transport;
use crate::types::{CheckoutResult, Feature, License, LicenseResult, LicenseStatus, LicenseType, ProductTier};
use parking_lot::RwLock;

/// The main IronLicensing client.
/// Thread-safe and can be shared across threads.
pub struct LicenseClient {
    options: LicenseOptions,
    transport: Transport,
    current_license: RwLock<Option<License>>,
    license_key: RwLock<Option<String>>,
}

impl LicenseClient {
    /// Create a new LicenseClient with the given options.
    pub fn new(options: LicenseOptions) -> Result<Self> {
        if options.public_key.is_empty() {
            return Err(LicenseError::PublicKeyRequired);
        }
        if options.product_slug.is_empty() {
            return Err(LicenseError::ProductSlugRequired);
        }

        let transport = Transport::new(&options);

        if options.debug {
            println!("[IronLicensing] Client initialized");
        }

        Ok(Self {
            options,
            transport,
            current_license: RwLock::new(None),
            license_key: RwLock::new(None),
        })
    }

    /// Create a new client with public key and product slug.
    pub fn with_credentials(public_key: impl Into<String>, product_slug: impl Into<String>) -> Result<Self> {
        Self::new(LicenseOptions::new(public_key, product_slug))
    }

    /// Validate a license key.
    pub fn validate(&self, license_key: &str) -> LicenseResult {
        let result = self.transport.validate(license_key);
        if result.valid {
            if let Some(license) = &result.license {
                *self.current_license.write() = Some(license.clone());
                *self.license_key.write() = Some(license_key.to_string());
            }
        }
        result
    }

    /// Activate a license key on this machine.
    pub fn activate(&self, license_key: &str) -> LicenseResult {
        self.activate_with_name(license_key, None)
    }

    /// Activate a license key with a custom machine name.
    pub fn activate_with_name(&self, license_key: &str, machine_name: Option<&str>) -> LicenseResult {
        let result = self.transport.activate(license_key, machine_name);
        if result.valid {
            if let Some(license) = &result.license {
                *self.current_license.write() = Some(license.clone());
                *self.license_key.write() = Some(license_key.to_string());
            }
        }
        result
    }

    /// Deactivate the current license from this machine.
    pub fn deactivate(&self) -> bool {
        let key = self.license_key.read().clone();
        if let Some(key) = key {
            if self.transport.deactivate(&key) {
                *self.current_license.write() = None;
                *self.license_key.write() = None;
                return true;
            }
        }
        false
    }

    /// Start a trial for the given email.
    pub fn start_trial(&self, email: &str) -> LicenseResult {
        let result = self.transport.start_trial(email);
        if result.valid {
            if let Some(license) = &result.license {
                *self.current_license.write() = Some(license.clone());
                *self.license_key.write() = Some(license.key.clone());
            }
        }
        result
    }

    /// Check if a feature is available in the current license.
    pub fn has_feature(&self, feature_key: &str) -> bool {
        self.current_license
            .read()
            .as_ref()
            .map(|l| l.has_feature(feature_key))
            .unwrap_or(false)
    }

    /// Require a feature to be available.
    /// Returns an error if the feature is not available.
    pub fn require_feature(&self, feature_key: &str) -> Result<()> {
        if !self.has_feature(feature_key) {
            return Err(LicenseError::FeatureRequired(feature_key.to_string()));
        }
        Ok(())
    }

    /// Get a feature from the current license.
    pub fn get_feature(&self, feature_key: &str) -> Option<Feature> {
        self.current_license
            .read()
            .as_ref()
            .and_then(|l| l.get_feature(feature_key).cloned())
    }

    /// Get the current license.
    pub fn license(&self) -> Option<License> {
        self.current_license.read().clone()
    }

    /// Get the current license status.
    pub fn status(&self) -> LicenseStatus {
        self.current_license
            .read()
            .as_ref()
            .map(|l| l.status)
            .unwrap_or(LicenseStatus::NotActivated)
    }

    /// Check if the application is licensed (valid or trial).
    pub fn is_licensed(&self) -> bool {
        self.current_license
            .read()
            .as_ref()
            .map(|l| matches!(l.status, LicenseStatus::Valid | LicenseStatus::Trial))
            .unwrap_or(false)
    }

    /// Check if running in trial mode.
    pub fn is_trial(&self) -> bool {
        self.current_license
            .read()
            .as_ref()
            .map(|l| l.status == LicenseStatus::Trial || l.license_type == LicenseType::Trial)
            .unwrap_or(false)
    }

    /// Get available product tiers for purchase.
    pub fn get_tiers(&self) -> Vec<ProductTier> {
        self.transport.get_tiers()
    }

    /// Start a checkout session for the specified tier.
    pub fn start_purchase(&self, tier_id: &str, email: &str) -> CheckoutResult {
        self.transport.start_checkout(tier_id, email)
    }

    /// Get the machine ID used for activations.
    pub fn machine_id(&self) -> &str {
        self.transport.machine_id()
    }
}


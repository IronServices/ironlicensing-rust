use std::time::Duration;

/// Configuration options for the LicenseClient.
#[derive(Debug, Clone)]
pub struct LicenseOptions {
    /// Public key for your product (required).
    pub public_key: String,
    /// Product slug identifier (required).
    pub product_slug: String,
    /// API base URL.
    pub api_base_url: String,
    /// Enable debug logging.
    pub debug: bool,
    /// Enable offline license caching.
    pub enable_offline_cache: bool,
    /// Cache validation interval in minutes.
    pub cache_validation_minutes: u32,
    /// Offline grace period in days.
    pub offline_grace_days: u32,
    /// HTTP request timeout.
    pub http_timeout: Duration,
}

impl LicenseOptions {
    /// Create new options with required public key and product slug.
    pub fn new(public_key: impl Into<String>, product_slug: impl Into<String>) -> Self {
        Self {
            public_key: public_key.into(),
            product_slug: product_slug.into(),
            ..Default::default()
        }
    }

    /// Set the API base URL.
    pub fn api_base_url(mut self, url: impl Into<String>) -> Self {
        self.api_base_url = url.into();
        self
    }

    /// Enable or disable debug mode.
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Enable or disable offline caching.
    pub fn enable_offline_cache(mut self, enable: bool) -> Self {
        self.enable_offline_cache = enable;
        self
    }

    /// Set cache validation interval in minutes.
    pub fn cache_validation_minutes(mut self, minutes: u32) -> Self {
        self.cache_validation_minutes = minutes;
        self
    }

    /// Set offline grace period in days.
    pub fn offline_grace_days(mut self, days: u32) -> Self {
        self.offline_grace_days = days;
        self
    }

    /// Set HTTP request timeout.
    pub fn http_timeout(mut self, timeout: Duration) -> Self {
        self.http_timeout = timeout;
        self
    }
}

impl Default for LicenseOptions {
    fn default() -> Self {
        Self {
            public_key: String::new(),
            product_slug: String::new(),
            api_base_url: "https://api.ironlicensing.com".to_string(),
            debug: false,
            enable_offline_cache: true,
            cache_validation_minutes: 60,
            offline_grace_days: 7,
            http_timeout: Duration::from_secs(30),
        }
    }
}

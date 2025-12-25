use thiserror::Error;

/// Errors that can occur in the IronLicensing SDK.
#[derive(Debug, Error)]
pub enum LicenseError {
    /// Client is not initialized.
    #[error("IronLicensing client not initialized")]
    NotInitialized,

    /// Public key is required.
    #[error("Public key is required")]
    PublicKeyRequired,

    /// Product slug is required.
    #[error("Product slug is required")]
    ProductSlugRequired,

    /// A required feature is not available.
    #[error("Feature '{0}' requires a valid license")]
    FeatureRequired(String),

    /// HTTP request error.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// API error returned from the server.
    #[error("API error: {0}")]
    Api(String),
}

pub type Result<T> = std::result::Result<T, LicenseError>;

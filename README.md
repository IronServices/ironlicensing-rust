# IronLicensing Rust SDK

Official Rust SDK for [IronLicensing](https://ironlicensing.com) - Software licensing and activation for your applications.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ironlicensing = "1.0"
```

## Quick Start

### Using Client Instance

```rust
use ironlicensing::{LicenseClient, LicenseOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = LicenseClient::with_credentials(
        "pk_live_your_public_key",
        "your-product-slug"
    )?;

    // Validate a license
    let result = client.validate("IRON-XXXX-XXXX-XXXX-XXXX");
    if result.valid {
        println!("License is valid!");
        if let Some(license) = &result.license {
            println!("Status: {:?}", license.status);
        }
    } else {
        println!("Validation failed: {:?}", result.error);
    }

    // Check for features
    if client.has_feature("premium") {
        println!("Premium features enabled!");
    }

    Ok(())
}
```

### Using Global Client

```rust
use ironlicensing::{init, validate, has_feature};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the global client
    init("pk_live_your_public_key", "your-product-slug")?;

    // Use global functions
    let result = validate("IRON-XXXX-XXXX-XXXX-XXXX")?;
    if result.valid {
        println!("License is valid!");
    }

    if has_feature("premium")? {
        println!("Premium features enabled!");
    }

    Ok(())
}
```

## Configuration

```rust
use ironlicensing::{LicenseClient, LicenseOptions};
use std::time::Duration;

let options = LicenseOptions::new("pk_live_xxx", "your-product")
    .api_base_url("https://api.ironlicensing.com")
    .debug(true)
    .enable_offline_cache(true)
    .cache_validation_minutes(60)
    .offline_grace_days(7)
    .http_timeout(Duration::from_secs(30));

let client = LicenseClient::new(options)?;
```

## License Validation

```rust
let result = client.validate("IRON-XXXX-XXXX-XXXX-XXXX");

if result.valid {
    if let Some(license) = &result.license {
        println!("License: {}", license.key);
        println!("Status: {:?}", license.status);
        println!("Type: {:?}", license.license_type);
        println!("Activations: {}/{}",
            license.current_activations,
            license.max_activations);
    }
}
```

## License Activation

```rust
// Simple activation (uses hostname as machine name)
let result = client.activate("IRON-XXXX-XXXX-XXXX-XXXX");

// With custom machine name
let result = client.activate_with_name(
    "IRON-XXXX-XXXX-XXXX-XXXX",
    Some("Production Server")
);

if result.valid {
    println!("License activated successfully!");

    // View activations
    if let Some(activations) = &result.activations {
        for activation in activations {
            println!("- {} ({:?})",
                activation.machine_name.as_deref().unwrap_or("Unknown"),
                activation.platform);
        }
    }
}
```

## License Deactivation

```rust
if client.deactivate() {
    println!("License deactivated from this machine");
}
```

## Feature Checking

```rust
// Check if feature is available
if client.has_feature("advanced-analytics") {
    // Enable advanced analytics
}

// Require feature (returns error if not available)
client.require_feature("export-pdf")?;
// Feature is available, continue with export

// Get feature details
if let Some(feature) = client.get_feature("max-users") {
    println!("Feature: {} - {:?}", feature.name, feature.description);
}
```

## Trial Management

```rust
let result = client.start_trial("user@example.com");

if result.valid {
    println!("Trial started!");
    if let Some(license) = &result.license {
        println!("Trial key: {}", license.key);
        if let Some(expires) = &license.expires_at {
            println!("Expires: {}", expires);
        }
    }
}
```

## In-App Purchase

```rust
// Get available tiers
let tiers = client.get_tiers();
for tier in &tiers {
    println!("{} - ${:.2} {}", tier.name, tier.price, tier.currency);
}

// Start checkout
let checkout = client.start_purchase("tier-id", "user@example.com");
if checkout.success {
    if let Some(url) = &checkout.checkout_url {
        println!("Checkout URL: {}", url);
        // Open URL in browser for user to complete purchase
    }
}
```

## License Status

```rust
use ironlicensing::LicenseStatus;

// Get current license
if let Some(license) = client.license() {
    println!("Licensed to: {:?}", license.email);
}

// Check status
match client.status() {
    LicenseStatus::Valid => println!("License is valid"),
    LicenseStatus::Expired => println!("License has expired"),
    LicenseStatus::Trial => println!("Running in trial mode"),
    LicenseStatus::NotActivated => println!("No license activated"),
    status => println!("Status: {:?}", status),
}

// Quick checks
if client.is_licensed() {
    println!("Application is licensed");
}

if client.is_trial() {
    println!("Running in trial mode");
}
```

## License Types

| Type | Description |
|------|-------------|
| `Perpetual` | One-time purchase, never expires |
| `Subscription` | Recurring payment, expires if not renewed |
| `Trial` | Time-limited trial license |

## License Statuses

| Status | Description |
|--------|-------------|
| `Valid` | License is valid and active |
| `Expired` | License has expired |
| `Suspended` | License temporarily suspended |
| `Revoked` | License permanently revoked |
| `Trial` | Active trial license |
| `TrialExpired` | Trial period ended |
| `NotActivated` | No license activated |

## Thread Safety

The client uses `parking_lot::RwLock` and is safe to share across threads:

```rust
use std::sync::Arc;
use std::thread;

let client = Arc::new(LicenseClient::with_credentials("pk_live_xxx", "product")?);

let handles: Vec<_> = (0..10).map(|_| {
    let client = Arc::clone(&client);
    thread::spawn(move || {
        if client.has_feature("concurrent-feature") {
            // Safe to call from multiple threads
        }
    })
}).collect();

for handle in handles {
    handle.join().unwrap();
}
```

## Error Handling

```rust
use ironlicensing::{LicenseError, LicenseResult};

// Validation errors
let result = client.validate(license_key);
if !result.valid {
    if let Some(error) = &result.error {
        match error.as_str() {
            "license_not_found" => println!("Invalid license key"),
            "license_expired" => println!("Your license has expired"),
            "max_activations_reached" => println!("No more activations available"),
            _ => println!("Error: {}", error),
        }
    }
}

// Feature requirement errors
match client.require_feature("premium") {
    Ok(()) => println!("Feature available"),
    Err(LicenseError::FeatureRequired(feature)) => {
        println!("Feature '{}' requires a valid license", feature);
    }
    Err(e) => println!("Error: {}", e),
}
```

## Machine ID

The SDK automatically generates and persists a unique machine ID at `~/.ironlicensing/machine_id`. This ID is used for:
- Tracking activations per machine
- Preventing license sharing
- Offline validation

```rust
let machine_id = client.machine_id();
```

## License

MIT License - see LICENSE file for details.

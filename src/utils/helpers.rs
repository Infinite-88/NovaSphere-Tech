//! Helper utilities and common functions
//!
//! This module provides various helper functions and utilities
//! used throughout the NovaForge system.

use chrono::{DateTime, Utc};
use uuid::Uuid;
use sha2::{Sha256, Digest};

/// Generate a unique identifier
pub fn generate_id() -> Uuid {
    Uuid::new_v4()
}

/// Get current timestamp
pub fn current_timestamp() -> DateTime<Utc> {
    Utc::now()
}

/// Calculate hash of a string
pub fn calculate_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Validate hash format
pub fn is_valid_hash(hash: &str) -> bool {
    hash.len() == 64 && hash.chars().all(|c| c.is_ascii_hexdigit())
}

/// Format duration in human-readable format
pub fn format_duration(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if days > 0 {
        format!("{}d {}h {}m {}s", days, hours, minutes, secs)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}

/// Clamp a value between min and max
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Calculate percentage
pub fn percentage(part: f64, total: f64) -> f64 {
    if total == 0.0 {
        0.0
    } else {
        (part / total) * 100.0
    }
}

/// Round to specified decimal places
pub fn round_to(value: f64, decimal_places: u32) -> f64 {
    let multiplier = 10.0_f64.powi(decimal_places as i32);
    (value * multiplier).round() / multiplier
}

/// Format bytes in human-readable format
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}

/// Validate Ethereum address format
pub fn is_valid_eth_address(address: &str) -> bool {
    address.starts_with("0x") && address.len() == 42 && address[2..].chars().all(|c| c.is_ascii_hexdigit())
}

/// Generate random string of specified length
pub fn random_string(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Calculate exponential backoff delay
pub fn exponential_backoff(attempt: u32, base_delay: u64, max_delay: u64) -> u64 {
    let delay = base_delay * 2_u64.pow(attempt);
    std::cmp::min(delay, max_delay)
}

/// Retry operation with exponential backoff
pub async fn retry_with_backoff<F, T, E>(
    mut operation: F,
    max_attempts: u32,
    base_delay: u64,
    max_delay: u64,
) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    let mut attempts = 0;
    
    loop {
        match operation() {
            Ok(result) => return Ok(result),
            Err(error) => {
                attempts += 1;
                if attempts >= max_attempts {
                    return Err(error);
                }
                
                let delay = exponential_backoff(attempts - 1, base_delay, max_delay);
                tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
            }
        }
    }
}

/// Validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
        self.is_valid = false;
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    pub fn combine(&mut self, other: ValidationResult) {
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
        self.is_valid = self.is_valid && other.is_valid;
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Metrics collector for performance monitoring
#[derive(Debug, Clone, Default)]
pub struct MetricsCollector {
    pub counters: std::collections::HashMap<String, u64>,
    pub gauges: std::collections::HashMap<String, f64>,
    pub histograms: std::collections::HashMap<String, Vec<f64>>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn increment_counter(&mut self, name: &str) {
        *self.counters.entry(name.to_string()).or_insert(0) += 1;
    }

    pub fn add_to_counter(&mut self, name: &str, value: u64) {
        *self.counters.entry(name.to_string()).or_insert(0) += value;
    }

    pub fn set_gauge(&mut self, name: &str, value: f64) {
        self.gauges.insert(name.to_string(), value);
    }

    pub fn record_histogram(&mut self, name: &str, value: f64) {
        self.histograms.entry(name.to_string()).or_default().push(value);
    }

    pub fn get_counter(&self, name: &str) -> u64 {
        self.counters.get(name).copied().unwrap_or(0)
    }

    pub fn get_gauge(&self, name: &str) -> f64 {
        self.gauges.get(name).copied().unwrap_or(0.0)
    }

    pub fn get_histogram_stats(&self, name: &str) -> Option<HistogramStats> {
        self.histograms.get(name).map(|values| {
            if values.is_empty() {
                return HistogramStats::default();
            }

            let mut sorted = values.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let count = sorted.len();
            let sum: f64 = sorted.iter().sum();
            let mean = sum / count as f64;
            
            let min = sorted[0];
            let max = sorted[count - 1];
            let median = if count % 2 == 0 {
                (sorted[count / 2 - 1] + sorted[count / 2]) / 2.0
            } else {
                sorted[count / 2]
            };

            HistogramStats {
                count,
                sum,
                mean,
                min,
                max,
                median,
            }
        })
    }

    pub fn reset(&mut self) {
        self.counters.clear();
        self.gauges.clear();
        self.histograms.clear();
    }
}

#[derive(Debug, Clone, Default)]
pub struct HistogramStats {
    pub count: usize,
    pub sum: f64,
    pub mean: f64,
    pub min: f64,
    pub max: f64,
    pub median: f64,
}

/// Configuration file handling
pub fn load_config_from_env_or_file<T>(
    env_var: &str,
    default_file: &str,
) -> Result<T, Box<dyn std::error::Error + Send + Sync>>
where
    T: serde::de::DeserializeOwned + Default,
{
    let config_path = std::env::var(env_var).unwrap_or_else(|_| default_file.to_string());
    
    if std::path::Path::new(&config_path).exists() {
        let content = std::fs::read_to_string(&config_path)?;
        let config: T = serde_json::from_str(&content)?;
        Ok(config)
    } else {
        tracing::warn!("Config file {} not found, using default configuration", config_path);
        Ok(T::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_id() {
        let id1 = generate_id();
        let id2 = generate_id();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_calculate_hash() {
        let hash1 = calculate_hash("test");
        let hash2 = calculate_hash("test");
        let hash3 = calculate_hash("different");
        
        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
        assert_eq!(hash1.len(), 64);
    }

    #[test]
    fn test_is_valid_hash() {
        assert!(is_valid_hash("a1b2c3d4e5f6789012345678901234567890123456789012345678901234abcd"));
        assert!(!is_valid_hash("invalid"));
        assert!(!is_valid_hash("g1b2c3d4e5f6789012345678901234567890123456789012345678901234abcd"));
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(30), "30s");
        assert_eq!(format_duration(90), "1m 30s");
        assert_eq!(format_duration(3661), "1h 1m 1s");
        assert_eq!(format_duration(90061), "1d 1h 1m 1s");
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5, 1, 10), 5);
        assert_eq!(clamp(0, 1, 10), 1);
        assert_eq!(clamp(15, 1, 10), 10);
    }

    #[test]
    fn test_percentage() {
        assert_eq!(percentage(25.0, 100.0), 25.0);
        assert_eq!(percentage(1.0, 0.0), 0.0);
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(512), "512.00 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1536), "1.50 KB");
    }

    #[test]
    fn test_is_valid_eth_address() {
        assert!(is_valid_eth_address("0x742D35Cc6670C8c1F6DA7D9EC92aE7d0df0E7df4"));
        assert!(!is_valid_eth_address("742D35Cc6670C8c1F6DA7D9EC92aE7d0df0E7df4"));
        assert!(!is_valid_eth_address("0x742D35Cc6670C8c1F6DA7D9EC92aE7d0df0E7df"));
    }

    #[test]
    fn test_exponential_backoff() {
        assert_eq!(exponential_backoff(0, 100, 10000), 100);
        assert_eq!(exponential_backoff(1, 100, 10000), 200);
        assert_eq!(exponential_backoff(2, 100, 10000), 400);
        assert_eq!(exponential_backoff(10, 100, 1000), 1000); // Capped at max
    }

    #[test]
    fn test_validation_result() {
        let mut result = ValidationResult::new();
        assert!(result.is_valid);
        assert!(!result.has_errors());

        result.add_error("Test error".to_string());
        assert!(!result.is_valid);
        assert!(result.has_errors());

        result.add_warning("Test warning".to_string());
        assert!(result.has_warnings());
    }

    #[test]
    fn test_metrics_collector() {
        let mut metrics = MetricsCollector::new();
        
        metrics.increment_counter("test_counter");
        metrics.increment_counter("test_counter");
        assert_eq!(metrics.get_counter("test_counter"), 2);

        metrics.set_gauge("test_gauge", 42.0);
        assert_eq!(metrics.get_gauge("test_gauge"), 42.0);

        metrics.record_histogram("test_histogram", 1.0);
        metrics.record_histogram("test_histogram", 2.0);
        metrics.record_histogram("test_histogram", 3.0);
        
        let stats = metrics.get_histogram_stats("test_histogram").unwrap();
        assert_eq!(stats.count, 3);
        assert_eq!(stats.mean, 2.0);
        assert_eq!(stats.median, 2.0);
    }
}
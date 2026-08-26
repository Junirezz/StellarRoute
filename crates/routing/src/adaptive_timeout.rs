//! Adaptive timeout controller with EMA-based clamping
//!
//! Provides dynamic timeout adjustment based on observed latency patterns.
//! Uses Exponential Moving Average (EMA) with min/max clamping.

use std::time::Duration;

/// Configuration for adaptive timeout
#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    /// Minimum timeout (floor)
    pub min_timeout: Duration,
    /// Maximum timeout (ceiling)
    pub max_timeout: Duration,
    /// Initial timeout value
    pub initial_timeout: Duration,
    /// EMA smoothing factor (0.0 - 1.0, higher = more responsive)
    pub ema_factor: f64,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            min_timeout: Duration::from_millis(100),
            max_timeout: Duration::from_secs(30),
            initial_timeout: Duration::from_secs(5),
            ema_factor: 0.3,
        }
    }
}

/// Adaptive timeout controller
#[derive(Debug)]
pub struct AdaptiveTimeoutController {
    config: TimeoutConfig,
    current_timeout: Duration,
    ema_latency: f64,
    initialized: bool,
}

impl AdaptiveTimeoutController {
    /// Create a new controller with default config
    pub fn new() -> Self {
        Self::with_config(TimeoutConfig::default())
    }

    /// Create a new controller with custom config
    pub fn with_config(config: TimeoutConfig) -> Self {
        Self {
            current_timeout: config.initial_timeout,
            ema_latency: config.initial_timeout.as_millis() as f64,
            config,
            initialized: false,
        }
    }

    /// Record a latency observation and update the timeout
    pub fn record_latency(&mut self, latency: Duration) {
        let latency_ms = latency.as_millis() as f64;

        if !self.initialized {
            self.ema_latency = latency_ms;
            self.initialized = true;
        } else {
            // Update EMA: new_ema = factor * sample + (1 - factor) * old_ema
            self.ema_latency = self.config.ema_factor * latency_ms
                + (1.0 - self.config.ema_factor) * self.ema_latency;
        }

        // Clamp to min/max bounds
        let clamped_ms = self.ema_latency
            .max(self.config.min_timeout.as_millis() as f64)
            .min(self.config.max_timeout.as_millis() as f64);

        self.current_timeout = Duration::from_millis(clamped_ms as u64);
    }

    /// Get the current adaptive timeout
    pub fn current_timeout(&self) -> Duration {
        self.current_timeout
    }

    /// Get the current EMA latency value
    pub fn ema_latency(&self) -> f64 {
        self.ema_latency
    }

    /// Get the controller config
    pub fn config(&self) -> &TimeoutConfig {
        &self.config
    }

    /// Reset the controller to initial state
    pub fn reset(&mut self) {
        self.current_timeout = self.config.initial_timeout;
        self.ema_latency = self.config.initial_timeout.as_millis() as f64;
        self.initialized = false;
    }
}

impl Default for AdaptiveTimeoutController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = TimeoutConfig::default();
        assert_eq!(config.min_timeout, Duration::from_millis(100));
        assert_eq!(config.max_timeout, Duration::from_secs(30));
        assert_eq!(config.initial_timeout, Duration::from_secs(5));
        assert!((config.ema_factor - 0.3).abs() < f64::EPSILON);
    }

    #[test]
    fn test_initial_timeout() {
        let controller = AdaptiveTimeoutController::new();
        assert_eq!(controller.current_timeout(), Duration::from_secs(5));
    }

    #[test]
    fn test_record_latency_ema_update() {
        let mut controller = AdaptiveTimeoutController::new();
        
        // First observation initializes EMA
        controller.record_latency(Duration::from_millis(1000));
        assert_eq!(controller.ema_latency(), 1000.0);
        
        // Second observation updates EMA
        controller.record_latency(Duration::from_millis(2000));
        // EMA = 0.3 * 2000 + 0.7 * 1000 = 600 + 700 = 1300
        assert!((controller.ema_latency() - 1300.0).abs() < 1.0);
    }

    #[test]
    fn test_min_clamp() {
        let config = TimeoutConfig {
            min_timeout: Duration::from_millis(500),
            max_timeout: Duration::from_secs(30),
            initial_timeout: Duration::from_secs(5),
            ema_factor: 1.0, // No smoothing for exact test
        };
        let mut controller = AdaptiveTimeoutController::with_config(config);
        
        // Very low latency should be clamped to min
        controller.record_latency(Duration::from_millis(100));
        assert_eq!(controller.current_timeout(), Duration::from_millis(500));
    }

    #[test]
    fn test_max_clamp() {
        let config = TimeoutConfig {
            min_timeout: Duration::from_millis(100),
            max_timeout: Duration::from_secs(10),
            initial_timeout: Duration::from_secs(5),
            ema_factor: 1.0, // No smoothing for exact test
        };
        let mut controller = AdaptiveTimeoutController::with_config(config);
        
        // Very high latency should be clamped to max
        controller.record_latency(Duration::from_secs(60));
        assert_eq!(controller.current_timeout(), Duration::from_secs(10));
    }

    #[test]
    fn test_ema_smoothing() {
        let config = TimeoutConfig {
            min_timeout: Duration::from_millis(100),
            max_timeout: Duration::from_secs(30),
            initial_timeout: Duration::from_secs(5),
            ema_factor: 0.5,
        };
        let mut controller = AdaptiveTimeoutController::with_config(config);
        
        // First observation
        controller.record_latency(Duration::from_millis(1000));
        assert_eq!(controller.ema_latency(), 1000.0);
        
        // Second observation with EMA factor 0.5
        controller.record_latency(Duration::from_millis(2000));
        // EMA = 0.5 * 2000 + 0.5 * 1000 = 1500
        assert_eq!(controller.ema_latency(), 1500.0);
    }

    #[test]
    fn test_reset() {
        let mut controller = AdaptiveTimeoutController::new();
        
        // Record some latency
        controller.record_latency(Duration::from_millis(5000));
        assert_ne!(controller.current_timeout(), Duration::from_secs(5));
        
        // Reset
        controller.reset();
        assert_eq!(controller.current_timeout(), Duration::from_secs(5));
        assert!(!controller.initialized);
    }

    #[test]
    fn test_multiple_observations_converge() {
        let mut controller = AdaptiveTimeoutController::new();
        
        // Record many observations at 2000ms
        for _ in 0..20 {
            controller.record_latency(Duration::from_millis(2000));
        }
        
        // EMA should converge close to 2000ms
        assert!((controller.ema_latency() - 2000.0).abs() < 100.0);
    }
}

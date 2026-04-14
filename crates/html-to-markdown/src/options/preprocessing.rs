#![allow(clippy::cast_precision_loss, clippy::cast_sign_loss, clippy::unused_self)]

//! HTML preprocessing configuration options.
//!
//! This module provides configuration for document cleanup before conversion,
//! including preset levels and granular control over element removal.

use crate::options::validation::normalize_token;

/// HTML preprocessing aggressiveness level.
///
/// Controls the extent of cleanup performed before conversion. Higher levels remove more elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PreprocessingPreset {
    /// Minimal cleanup. Remove only essential noise (scripts, styles).
    Minimal,
    /// Standard cleanup. Default. Removes navigation, forms, and other auxiliary content.
    #[default]
    Standard,
    /// Aggressive cleanup. Remove extensive non-content elements and structure.
    Aggressive,
}

impl PreprocessingPreset {
    /// Parse a preprocessing preset from a string.
    ///
    /// Accepts "minimal", "aggressive", or defaults to Standard.
    /// Input is normalized (lowercased, alphanumeric only).
    #[must_use]
    pub fn parse(value: &str) -> Self {
        match normalize_token(value).as_str() {
            "minimal" => Self::Minimal,
            "aggressive" => Self::Aggressive,
            _ => Self::Standard,
        }
    }
}

/// HTML preprocessing options for document cleanup before conversion.
#[derive(Debug, Clone)]
#[cfg_attr(
    any(feature = "serde", feature = "metadata"),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", feature = "metadata"), serde(default, deny_unknown_fields))]
pub struct PreprocessingOptions {
    /// Enable HTML preprocessing globally
    pub enabled: bool,

    /// Preprocessing preset level (Minimal, Standard, Aggressive)
    pub preset: PreprocessingPreset,

    /// Remove navigation elements (nav, breadcrumbs, menus, sidebars)
    pub remove_navigation: bool,

    /// Remove form elements (forms, inputs, buttons, etc.)
    pub remove_forms: bool,
}

/// Partial update for `PreprocessingOptions`.
///
/// This struct uses `Option<T>` to represent optional fields that can be selectively updated.
/// Only specified fields (Some values) will override existing options; None values leave the
/// corresponding fields unchanged when applied via [`PreprocessingOptions::apply_update`].
#[derive(Debug, Clone, Default)]
#[cfg_attr(
    any(feature = "serde", feature = "metadata"),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", feature = "metadata"), serde(deny_unknown_fields))]
pub struct PreprocessingOptionsUpdate {
    /// Optional global preprocessing enablement override
    pub enabled: Option<bool>,

    /// Optional preprocessing preset level override (Minimal, Standard, Aggressive)
    pub preset: Option<PreprocessingPreset>,

    /// Optional navigation element removal override (nav, breadcrumbs, menus, sidebars)
    pub remove_navigation: Option<bool>,

    /// Optional form element removal override (forms, inputs, buttons, etc.)
    pub remove_forms: Option<bool>,
}

impl Default for PreprocessingOptions {
    fn default() -> Self {
        Self {
            enabled: true,
            preset: PreprocessingPreset::default(),
            remove_navigation: true,
            remove_forms: true,
        }
    }
}

impl PreprocessingOptions {
    /// Apply a partial update to these preprocessing options.
    ///
    /// Any specified fields in the update will override the current values.
    /// Unspecified fields (None) are left unchanged.
    ///
    /// # Arguments
    ///
    /// * `update` - Partial preprocessing options update
    #[allow(clippy::needless_pass_by_value)]
    pub const fn apply_update(&mut self, update: PreprocessingOptionsUpdate) {
        if let Some(enabled) = update.enabled {
            self.enabled = enabled;
        }
        if let Some(preset) = update.preset {
            self.preset = preset;
        }
        if let Some(remove_navigation) = update.remove_navigation {
            self.remove_navigation = remove_navigation;
        }
        if let Some(remove_forms) = update.remove_forms {
            self.remove_forms = remove_forms;
        }
    }

    /// Create new preprocessing options from a partial update.
    ///
    /// Creates a new `PreprocessingOptions` struct with defaults, then applies the update.
    /// Fields not specified in the update keep their default values.
    ///
    /// # Arguments
    ///
    /// * `update` - Partial preprocessing options update
    ///
    /// # Returns
    ///
    /// New `PreprocessingOptions` with specified updates applied to defaults
    #[must_use]
    pub fn from_update(update: PreprocessingOptionsUpdate) -> Self {
        let mut options = Self::default();
        options.apply_update(update);
        options
    }
}

impl From<PreprocessingOptionsUpdate> for PreprocessingOptions {
    fn from(update: PreprocessingOptionsUpdate) -> Self {
        Self::from_update(update)
    }
}

#[cfg(any(feature = "serde", feature = "metadata"))]
mod serde_impls {
    use super::PreprocessingPreset;
    use serde::{Deserialize, Serializer};

    impl<'de> Deserialize<'de> for PreprocessingPreset {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let value = String::deserialize(deserializer)?;
            Ok(Self::parse(&value))
        }
    }

    impl serde::Serialize for PreprocessingPreset {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let s = match self {
                Self::Minimal => "minimal",
                Self::Standard => "standard",
                Self::Aggressive => "aggressive",
            };
            serializer.serialize_str(s)
        }
    }
}

#[cfg(all(test, any(feature = "serde", feature = "metadata")))]
mod tests {
    use super::*;

    #[test]
    fn test_preprocessing_options_serde() {
        let options = PreprocessingOptions {
            enabled: true,
            preset: PreprocessingPreset::Aggressive,
            remove_navigation: false,
            ..Default::default()
        };

        // Serialize to JSON
        let json = serde_json::to_string(&options).expect("Failed to serialize");

        // Deserialize back
        let deserialized: PreprocessingOptions = serde_json::from_str(&json).expect("Failed to deserialize");

        // Verify values
        assert!(deserialized.enabled);
        assert_eq!(deserialized.preset, PreprocessingPreset::Aggressive);
        assert!(!deserialized.remove_navigation);
    }
}

// Design Token Parser - Phase 8 Sprint 2 Task 2.1
//
// This module provides support for loading design tokens from JSON and YAML files.
// Design tokens allow teams to centralize design decisions (colors, spacing, typography)
// and share them across tools and platforms.
//
// Supported token types:
// - Colors (palettes with shades, semantic colors)
// - Spacing (scale values)
// - Typography (font families, sizes, weights, line heights)
// - Shadows (box shadows)
// - Border radius (rounding values)
// - Breakpoints (responsive design)
//
// Example JSON:
// {
//   "colors": {
//     "brand": {
//       "primary": "#4f46e5",
//       "secondary": "#06b6d4"
//     }
//   },
//   "spacing": {
//     "xs": "4px",
//     "sm": "8px"
//   }
// }

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Design token file format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignTokens {
    #[serde(default)]
    pub colors: HashMap<String, ColorToken>,

    #[serde(default)]
    pub spacing: HashMap<String, String>,

    #[serde(default)]
    pub typography: TypographyTokens,

    #[serde(default)]
    pub shadows: HashMap<String, String>,

    #[serde(default)]
    pub radii: HashMap<String, String>,

    #[serde(default)]
    pub breakpoints: HashMap<String, String>,
}

/// Color token - can be a single value or a palette with shades
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColorToken {
    /// Single color value (e.g., "primary": "#4f46e5")
    Single(String),

    /// Palette with shades (e.g., "blue": { "50": "#eff6ff", "100": "#dbeafe", ... })
    Palette(HashMap<String, String>),
}

/// Typography tokens
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TypographyTokens {
    #[serde(default)]
    pub font_families: HashMap<String, String>,

    #[serde(default)]
    pub font_sizes: HashMap<String, String>,

    #[serde(default)]
    pub font_weights: HashMap<String, String>,

    #[serde(default)]
    pub line_heights: HashMap<String, String>,

    #[serde(default)]
    pub letter_spacings: HashMap<String, String>,
}

impl Default for DesignTokens {
    fn default() -> Self {
        Self {
            colors: HashMap::new(),
            spacing: HashMap::new(),
            typography: TypographyTokens::default(),
            shadows: HashMap::new(),
            radii: HashMap::new(),
            breakpoints: HashMap::new(),
        }
    }
}

impl DesignTokens {
    /// Load design tokens from a JSON file
    pub fn from_json_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(format!("Token file not found: {}", path.display()));
        }

        let contents = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read token file: {}", e))?;

        serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse JSON tokens: {}", e))
    }

    /// Load design tokens from a YAML file
    pub fn from_yaml_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(format!("Token file not found: {}", path.display()));
        }

        let contents = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read token file: {}", e))?;

        serde_yaml::from_str(&contents)
            .map_err(|e| format!("Failed to parse YAML tokens: {}", e))
    }

    /// Load design tokens from a file (auto-detect format by extension)
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path = path.as_ref();

        match path.extension().and_then(|s| s.to_str()) {
            Some("json") => Self::from_json_file(path),
            Some("yaml") | Some("yml") => Self::from_yaml_file(path),
            Some(ext) => Err(format!("Unsupported token file format: .{}", ext)),
            None => Err("Token file has no extension".to_string()),
        }
    }

    /// Merge another token set into this one
    /// Values from `other` will override values in `self`
    pub fn merge(&mut self, other: DesignTokens) {
        // Merge colors
        for (key, value) in other.colors {
            self.colors.insert(key, value);
        }

        // Merge spacing
        for (key, value) in other.spacing {
            self.spacing.insert(key, value);
        }

        // Merge typography
        for (key, value) in other.typography.font_families {
            self.typography.font_families.insert(key, value);
        }
        for (key, value) in other.typography.font_sizes {
            self.typography.font_sizes.insert(key, value);
        }
        for (key, value) in other.typography.font_weights {
            self.typography.font_weights.insert(key, value);
        }
        for (key, value) in other.typography.line_heights {
            self.typography.line_heights.insert(key, value);
        }
        for (key, value) in other.typography.letter_spacings {
            self.typography.letter_spacings.insert(key, value);
        }

        // Merge shadows
        for (key, value) in other.shadows {
            self.shadows.insert(key, value);
        }

        // Merge radii
        for (key, value) in other.radii {
            self.radii.insert(key, value);
        }

        // Merge breakpoints
        for (key, value) in other.breakpoints {
            self.breakpoints.insert(key, value);
        }
    }

    /// Convert color tokens to the format expected by UtilityConfig
    /// Returns a map of color_name → (shade → hex_value)
    pub fn to_color_palettes(&self) -> HashMap<String, HashMap<u32, String>> {
        let mut palettes = HashMap::new();

        for (color_name, color_token) in &self.colors {
            match color_token {
                ColorToken::Single(hex) => {
                    // Single color becomes a palette with only shade 500
                    let mut shades = HashMap::new();
                    shades.insert(500, hex.clone());
                    palettes.insert(color_name.clone(), shades);
                }
                ColorToken::Palette(shades) => {
                    // Convert string keys to u32
                    let mut numeric_shades = HashMap::new();
                    for (shade_str, hex) in shades {
                        if let Ok(shade_num) = shade_str.parse::<u32>() {
                            numeric_shades.insert(shade_num, hex.clone());
                        }
                    }
                    palettes.insert(color_name.clone(), numeric_shades);
                }
            }
        }

        palettes
    }

    /// Convert spacing tokens to the format expected by UtilityConfig
    /// Parses "4px", "0.5rem", etc. into pixel values
    pub fn to_spacing_values(&self) -> Vec<u32> {
        let mut values = Vec::new();

        for value_str in self.spacing.values() {
            if let Some(pixels) = parse_size_to_pixels(value_str) {
                values.push(pixels);
            }
        }

        // Sort and deduplicate
        values.sort();
        values.dedup();

        values
    }
}

/// Parse a CSS size value to pixels
/// Supports: "16px", "1rem" (assumes 16px/rem), "0.5em" (assumes 16px/em)
fn parse_size_to_pixels(value: &str) -> Option<u32> {
    let value = value.trim();

    // Try pixels
    if let Some(num_str) = value.strip_suffix("px") {
        return num_str.trim().parse::<u32>().ok();
    }

    // Try rem (assume 16px base)
    if let Some(num_str) = value.strip_suffix("rem") {
        if let Ok(rem) = num_str.trim().parse::<f64>() {
            return Some((rem * 16.0) as u32);
        }
    }

    // Try em (assume 16px base)
    if let Some(num_str) = value.strip_suffix("em") {
        if let Ok(em) = num_str.trim().parse::<f64>() {
            return Some((em * 16.0) as u32);
        }
    }

    // Try plain number (assume pixels)
    value.parse::<u32>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size_to_pixels() {
        assert_eq!(parse_size_to_pixels("16px"), Some(16));
        assert_eq!(parse_size_to_pixels("1rem"), Some(16));
        assert_eq!(parse_size_to_pixels("0.5rem"), Some(8));
        assert_eq!(parse_size_to_pixels("2em"), Some(32));
        assert_eq!(parse_size_to_pixels("24"), Some(24));
        assert_eq!(parse_size_to_pixels("invalid"), None);
    }

    #[test]
    fn test_color_token_single() {
        let json = r###"{"primary": "#4f46e5"}"###;
        let colors: HashMap<String, ColorToken> = serde_json::from_str(json).unwrap();

        match colors.get("primary") {
            Some(ColorToken::Single(hex)) => assert_eq!(hex, "#4f46e5"),
            _ => panic!("Expected Single color token"),
        }
    }

    #[test]
    fn test_color_token_palette() {
        let json = r###"{"blue": {"50": "#eff6ff", "100": "#dbeafe"}}"###;
        let colors: HashMap<String, ColorToken> = serde_json::from_str(json).unwrap();

        match colors.get("blue") {
            Some(ColorToken::Palette(shades)) => {
                assert_eq!(shades.get("50").unwrap(), "#eff6ff");
                assert_eq!(shades.get("100").unwrap(), "#dbeafe");
            }
            _ => panic!("Expected Palette color token"),
        }
    }

    #[test]
    fn test_design_tokens_deserialization() {
        let json = r###"{
            "colors": {
                "primary": "#4f46e5",
                "blue": {
                    "50": "#eff6ff",
                    "500": "#3b82f6"
                }
            },
            "spacing": {
                "xs": "4px",
                "sm": "8px"
            },
            "typography": {
                "font_families": {
                    "sans": "Inter, system-ui"
                },
                "font_sizes": {
                    "sm": "14px",
                    "base": "16px"
                }
            }
        }"###;

        let tokens: DesignTokens = serde_json::from_str(json).unwrap();

        // Check colors
        assert_eq!(tokens.colors.len(), 2);
        assert!(matches!(tokens.colors.get("primary"), Some(ColorToken::Single(_))));
        assert!(matches!(tokens.colors.get("blue"), Some(ColorToken::Palette(_))));

        // Check spacing
        assert_eq!(tokens.spacing.len(), 2);
        assert_eq!(tokens.spacing.get("xs").unwrap(), "4px");

        // Check typography
        assert_eq!(tokens.typography.font_families.get("sans").unwrap(), "Inter, system-ui");
        assert_eq!(tokens.typography.font_sizes.get("base").unwrap(), "16px");
    }

    #[test]
    fn test_to_color_palettes() {
        let json = r###"{
            "colors": {
                "primary": "#4f46e5",
                "blue": {
                    "50": "#eff6ff",
                    "500": "#3b82f6",
                    "900": "#1e3a8a"
                }
            }
        }"###;

        let tokens: DesignTokens = serde_json::from_str(json).unwrap();
        let palettes = tokens.to_color_palettes();

        // Primary becomes a single-shade palette
        let primary = palettes.get("primary").unwrap();
        assert_eq!(primary.len(), 1);
        assert_eq!(primary.get(&500).unwrap(), "#4f46e5");

        // Blue has 3 shades
        let blue = palettes.get("blue").unwrap();
        assert_eq!(blue.len(), 3);
        assert_eq!(blue.get(&50).unwrap(), "#eff6ff");
        assert_eq!(blue.get(&500).unwrap(), "#3b82f6");
        assert_eq!(blue.get(&900).unwrap(), "#1e3a8a");
    }

    #[test]
    fn test_to_spacing_values() {
        let json = r###"{
            "spacing": {
                "xs": "4px",
                "sm": "8px",
                "md": "1rem",
                "lg": "2rem"
            }
        }"###;

        let tokens: DesignTokens = serde_json::from_str(json).unwrap();
        let spacing = tokens.to_spacing_values();

        // Should have: 4, 8, 16, 32 (sorted and deduped)
        assert_eq!(spacing, vec![4, 8, 16, 32]);
    }

    #[test]
    fn test_merge_tokens() {
        let mut base = DesignTokens::default();
        base.colors.insert("primary".to_string(), ColorToken::Single("#000000".to_string()));
        base.spacing.insert("sm".to_string(), "8px".to_string());

        let mut override_tokens = DesignTokens::default();
        override_tokens.colors.insert("primary".to_string(), ColorToken::Single("#4f46e5".to_string()));
        override_tokens.colors.insert("secondary".to_string(), ColorToken::Single("#06b6d4".to_string()));

        base.merge(override_tokens);

        // primary should be overridden
        match base.colors.get("primary") {
            Some(ColorToken::Single(hex)) => assert_eq!(hex, "#4f46e5"),
            _ => panic!("Expected overridden primary color"),
        }

        // secondary should be added
        assert!(base.colors.contains_key("secondary"));

        // spacing should remain
        assert_eq!(base.spacing.get("sm").unwrap(), "8px");
    }
}

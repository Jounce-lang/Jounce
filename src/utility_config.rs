// RavensOne Utility Class System - Configuration
// Loads configuration from raven.config.toml or provides sensible defaults

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UtilityConfig {
    #[serde(default)]
    pub css: CssConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CssConfig {
    #[serde(default = "default_true")]
    pub utilities: bool,

    #[serde(default = "default_true")]
    pub jit: bool,

    #[serde(default = "default_false")]
    pub minify: bool,

    #[serde(default)]
    pub theme: ThemeConfig,

    #[serde(default)]
    pub utilities_custom: Option<HashMap<String, String>>,

    /// Path to design tokens file (JSON or YAML)
    /// Example: "tokens.json" or "design-tokens.yaml"
    #[serde(default)]
    pub tokens_file: Option<String>,

    /// Enable theme mode with CSS custom properties
    /// When enabled, generates :root variables like --color-blue-500, --spacing-4
    #[serde(default)]
    pub theme_mode: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThemeConfig {
    #[serde(default = "default_colors")]
    pub colors: Vec<ColorDefinition>,

    #[serde(default = "default_spacing")]
    pub spacing: Vec<u32>,

    #[serde(default = "default_font_sizes")]
    pub font_sizes: Vec<FontSizeDefinition>,

    #[serde(default = "default_border_radius")]
    pub border_radius: Vec<BorderRadiusDefinition>,

    #[serde(default = "default_breakpoints")]
    pub breakpoints: Vec<BreakpointDefinition>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ColorDefinition {
    pub name: String,
    pub shades: HashMap<u32, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FontSizeDefinition {
    pub name: String,
    pub size: String,
    pub line_height: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BorderRadiusDefinition {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BreakpointDefinition {
    pub name: String,
    pub min_width: String,
}

// Default value helpers for serde
fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

fn default_colors() -> Vec<ColorDefinition> {
    vec![
        ColorDefinition {
            name: "blue".to_string(),
            shades: [
                (50, "#eff6ff"),
                (100, "#dbeafe"),
                (200, "#bfdbfe"),
                (300, "#93c5fd"),
                (400, "#60a5fa"),
                (500, "#3b82f6"),
                (600, "#2563eb"),
                (700, "#1d4ed8"),
                (800, "#1e40af"),
                (900, "#1e3a8a"),
            ].iter().map(|(k, v)| (*k, v.to_string())).collect(),
        },
        ColorDefinition {
            name: "gray".to_string(),
            shades: [
                (50, "#f9fafb"),
                (100, "#f3f4f6"),
                (200, "#e5e7eb"),
                (300, "#d1d5db"),
                (400, "#9ca3af"),
                (500, "#6b7280"),
                (600, "#4b5563"),
                (700, "#374151"),
                (800, "#1f2937"),
                (900, "#111827"),
            ].iter().map(|(k, v)| (*k, v.to_string())).collect(),
        },
        ColorDefinition {
            name: "red".to_string(),
            shades: [
                (50, "#fef2f2"),
                (100, "#fee2e2"),
                (200, "#fecaca"),
                (300, "#fca5a5"),
                (400, "#f87171"),
                (500, "#ef4444"),
                (600, "#dc2626"),
                (700, "#b91c1c"),
                (800, "#991b1b"),
                (900, "#7f1d1d"),
            ].iter().map(|(k, v)| (*k, v.to_string())).collect(),
        },
        ColorDefinition {
            name: "green".to_string(),
            shades: [
                (50, "#f0fdf4"),
                (100, "#dcfce7"),
                (200, "#bbf7d0"),
                (300, "#86efac"),
                (400, "#4ade80"),
                (500, "#10b981"),
                (600, "#059669"),
                (700, "#047857"),
                (800, "#065f46"),
                (900, "#14532d"),
            ].iter().map(|(k, v)| (*k, v.to_string())).collect(),
        },
        ColorDefinition {
            name: "yellow".to_string(),
            shades: [
                (50, "#fefce8"),
                (100, "#fef9c3"),
                (200, "#fef08a"),
                (300, "#fde047"),
                (400, "#facc15"),
                (500, "#eab308"),
                (600, "#ca8a04"),
                (700, "#a16207"),
                (800, "#854d0e"),
                (900, "#713f12"),
            ].iter().map(|(k, v)| (*k, v.to_string())).collect(),
        },
        ColorDefinition {
            name: "purple".to_string(),
            shades: [
                (50, "#faf5ff"),
                (100, "#f3e8ff"),
                (200, "#e9d5ff"),
                (300, "#d8b4fe"),
                (400, "#c084fc"),
                (500, "#a855f7"),
                (600, "#9333ea"),
                (700, "#7e22ce"),
                (800, "#6b21a8"),
                (900, "#581c87"),
            ].iter().map(|(k, v)| (*k, v.to_string())).collect(),
        },
    ]
}

fn default_spacing() -> Vec<u32> {
    vec![0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 40, 48, 56, 64, 80, 96, 128, 160, 192, 224, 256]
}

fn default_font_sizes() -> Vec<FontSizeDefinition> {
    vec![
        FontSizeDefinition {
            name: "xs".to_string(),
            size: "12px".to_string(),
            line_height: "16px".to_string(),
        },
        FontSizeDefinition {
            name: "sm".to_string(),
            size: "14px".to_string(),
            line_height: "20px".to_string(),
        },
        FontSizeDefinition {
            name: "base".to_string(),
            size: "16px".to_string(),
            line_height: "24px".to_string(),
        },
        FontSizeDefinition {
            name: "lg".to_string(),
            size: "18px".to_string(),
            line_height: "28px".to_string(),
        },
        FontSizeDefinition {
            name: "xl".to_string(),
            size: "20px".to_string(),
            line_height: "28px".to_string(),
        },
        FontSizeDefinition {
            name: "2xl".to_string(),
            size: "24px".to_string(),
            line_height: "32px".to_string(),
        },
        FontSizeDefinition {
            name: "3xl".to_string(),
            size: "30px".to_string(),
            line_height: "36px".to_string(),
        },
        FontSizeDefinition {
            name: "4xl".to_string(),
            size: "36px".to_string(),
            line_height: "40px".to_string(),
        },
    ]
}

fn default_border_radius() -> Vec<BorderRadiusDefinition> {
    vec![
        BorderRadiusDefinition {
            name: "none".to_string(),
            value: "0".to_string(),
        },
        BorderRadiusDefinition {
            name: "sm".to_string(),
            value: "2px".to_string(),
        },
        BorderRadiusDefinition {
            name: "base".to_string(),
            value: "4px".to_string(),
        },
        BorderRadiusDefinition {
            name: "md".to_string(),
            value: "6px".to_string(),
        },
        BorderRadiusDefinition {
            name: "lg".to_string(),
            value: "8px".to_string(),
        },
        BorderRadiusDefinition {
            name: "xl".to_string(),
            value: "12px".to_string(),
        },
        BorderRadiusDefinition {
            name: "2xl".to_string(),
            value: "16px".to_string(),
        },
        BorderRadiusDefinition {
            name: "full".to_string(),
            value: "9999px".to_string(),
        },
    ]
}

fn default_breakpoints() -> Vec<BreakpointDefinition> {
    vec![
        BreakpointDefinition {
            name: "sm".to_string(),
            min_width: "640px".to_string(),
        },
        BreakpointDefinition {
            name: "md".to_string(),
            min_width: "768px".to_string(),
        },
        BreakpointDefinition {
            name: "lg".to_string(),
            min_width: "1024px".to_string(),
        },
        BreakpointDefinition {
            name: "xl".to_string(),
            min_width: "1280px".to_string(),
        },
        BreakpointDefinition {
            name: "2xl".to_string(),
            min_width: "1536px".to_string(),
        },
    ]
}

impl Default for UtilityConfig {
    fn default() -> Self {
        Self {
            css: CssConfig::default(),
        }
    }
}

impl Default for CssConfig {
    fn default() -> Self {
        Self {
            utilities: true,
            jit: true,
            minify: false,
            theme: ThemeConfig::default(),
            utilities_custom: None,
            tokens_file: None,
            theme_mode: None,
        }
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            colors: default_colors(),
            spacing: default_spacing(),
            font_sizes: default_font_sizes(),
            border_radius: default_border_radius(),
            breakpoints: default_breakpoints(),
        }
    }
}

impl UtilityConfig {
    /// Load configuration from raven.config.toml or use defaults
    pub fn load() -> Self {
        Self::load_from_path("raven.config.toml")
    }

    /// Load configuration from a specific path
    pub fn load_from_path(path: &str) -> Self {
        let config_path = Path::new(path);

        let mut config = if config_path.exists() {
            match fs::read_to_string(config_path) {
                Ok(content) => {
                    match toml::from_str(&content) {
                        Ok(config) => config,
                        Err(e) => {
                            eprintln!("Warning: Failed to parse {}: {}", path, e);
                            eprintln!("Using default configuration");
                            Self::default()
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Failed to read {}: {}", path, e);
                    Self::default()
                }
            }
        } else {
            Self::default()
        };

        // Load and merge design tokens if specified
        config.load_design_tokens();

        config
    }

    /// Load design tokens from the configured file and merge into theme
    pub fn load_design_tokens(&mut self) {
        use crate::design_tokens::DesignTokens;

        if let Some(tokens_path) = &self.css.tokens_file {
            match DesignTokens::from_file(tokens_path) {
                Ok(tokens) => {
                    // Merge color palettes
                    let color_palettes = tokens.to_color_palettes();
                    for (color_name, shades) in color_palettes {
                        // Check if color already exists
                        if let Some(existing) = self.css.theme.colors.iter_mut().find(|c| c.name == color_name) {
                            // Merge shades (override existing)
                            for (shade, hex) in shades {
                                existing.shades.insert(shade, hex);
                            }
                        } else {
                            // Add new color
                            self.css.theme.colors.push(ColorDefinition {
                                name: color_name,
                                shades,
                            });
                        }
                    }

                    // Merge spacing values
                    let spacing_values = tokens.to_spacing_values();
                    for value in spacing_values {
                        if !self.css.theme.spacing.contains(&value) {
                            self.css.theme.spacing.push(value);
                        }
                    }
                    self.css.theme.spacing.sort();
                    self.css.theme.spacing.dedup();
                }
                Err(e) => {
                    eprintln!("Warning: Failed to load design tokens from {}: {}", tokens_path, e);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = UtilityConfig::default();
        assert!(config.css.utilities);
        assert!(config.css.jit);
        assert!(!config.css.minify);
    }

    #[test]
    fn test_default_colors() {
        let config = UtilityConfig::default();
        assert!(!config.css.theme.colors.is_empty());

        let blue = config.css.theme.colors.iter().find(|c| c.name == "blue").unwrap();
        assert_eq!(blue.shades.get(&500), Some(&"#3b82f6".to_string()));
    }

    #[test]
    fn test_default_spacing() {
        let config = UtilityConfig::default();
        assert!(config.css.theme.spacing.contains(&4));
        assert!(config.css.theme.spacing.contains(&16));
    }

    #[test]
    fn test_load_nonexistent_file() {
        let config = UtilityConfig::load_from_path("nonexistent.toml");
        assert!(config.css.utilities); // Should use defaults
    }
}

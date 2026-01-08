//! Configuration schema structs

use serde::{Deserialize, Serialize};

/// Root configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub general: GeneralConfig,
    pub appearance: AppearanceConfig,
    pub colors: ColorConfig,
    pub keybindings: KeybindingConfig,
    pub terminal: TerminalConfig,
    pub claude: ClaudeConfig,
    pub persistence: PersistenceConfig,
}

/// General settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneralConfig {
    /// Default shell to spawn
    pub default_shell: String,
    /// Maximum Claude session depth
    pub max_depth: u32,
    /// Prefix key for commands
    pub prefix_key: String,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            default_shell: std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".into()),
            max_depth: 5,
            prefix_key: "Ctrl-a".into(),
        }
    }
}

/// Appearance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppearanceConfig {
    /// Theme name or path
    pub theme: String,
    /// Status bar position
    pub status_position: StatusPosition,
    /// Pane border style
    pub border_style: BorderStyle,
    /// Show pane titles
    pub show_pane_titles: bool,
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            theme: "default".into(),
            status_position: StatusPosition::Bottom,
            border_style: BorderStyle::Rounded,
            show_pane_titles: true,
        }
    }
}

/// Status bar position
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum StatusPosition {
    Top,
    #[default]
    Bottom,
}

/// Border style
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum BorderStyle {
    Single,
    Double,
    #[default]
    Rounded,
    None,
}

/// Color settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ColorConfig {
    pub status_bg: String,
    pub status_fg: String,
    pub active_border: String,
    pub inactive_border: String,
    pub claude_thinking: String,
    pub claude_idle: String,
    pub claude_error: String,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            status_bg: "#282c34".into(),
            status_fg: "#abb2bf".into(),
            active_border: "#61afef".into(),
            inactive_border: "#5c6370".into(),
            claude_thinking: "#e5c07b".into(),
            claude_idle: "#98c379".into(),
            claude_error: "#e06c75".into(),
        }
    }
}

/// Keybinding settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct KeybindingConfig {
    pub split_horizontal: String,
    pub split_vertical: String,
    pub focus_left: String,
    pub focus_right: String,
    pub focus_up: String,
    pub focus_down: String,
    pub new_session: String,
    pub detach: String,
    pub list_sessions: String,
}

impl Default for KeybindingConfig {
    fn default() -> Self {
        Self {
            split_horizontal: "prefix %".into(),
            split_vertical: "prefix \"".into(),
            focus_left: "prefix h".into(),
            focus_right: "prefix l".into(),
            focus_up: "prefix k".into(),
            focus_down: "prefix j".into(),
            new_session: "prefix c".into(),
            detach: "prefix d".into(),
            list_sessions: "prefix s".into(),
        }
    }
}

/// Terminal settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TerminalConfig {
    /// Scrollback buffer lines
    pub scrollback_lines: usize,
    /// Render interval (ms)
    pub render_interval_ms: u64,
    /// Parser timeout (seconds)
    pub parser_timeout_secs: u64,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            scrollback_lines: 10000,
            render_interval_ms: 16,
            parser_timeout_secs: 5,
        }
    }
}

/// Claude integration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ClaudeConfig {
    /// Enable detection
    pub detection_enabled: bool,
    /// Detection method
    pub detection_method: DetectionMethod,
    /// Show in status bar
    pub show_status: bool,
    /// Auto-resume crashed sessions
    pub auto_resume: bool,
}

impl Default for ClaudeConfig {
    fn default() -> Self {
        Self {
            detection_enabled: true,
            detection_method: DetectionMethod::Pty,
            show_status: true,
            auto_resume: true,
        }
    }
}

/// Claude detection method
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum DetectionMethod {
    #[default]
    Pty,
    StreamJson,
    Visual,
}

/// Persistence settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PersistenceConfig {
    /// Checkpoint interval (seconds)
    pub checkpoint_interval_secs: u64,
    /// Max WAL size (MB)
    pub max_wal_size_mb: u64,
    /// Screen snapshot lines
    pub screen_snapshot_lines: usize,
}

impl Default for PersistenceConfig {
    fn default() -> Self {
        Self {
            checkpoint_interval_secs: 30,
            max_wal_size_mb: 128,
            screen_snapshot_lines: 500,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.general.max_depth, 5);
        assert_eq!(config.terminal.render_interval_ms, 16);
    }

    #[test]
    fn test_serialize_roundtrip() {
        let config = AppConfig::default();
        let toml_str = toml::to_string(&config).unwrap();
        let parsed: AppConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.general.max_depth, config.general.max_depth);
    }

    #[test]
    fn test_partial_config() {
        let toml_str = r#"
            [general]
            max_depth = 10
        "#;
        let config: AppConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.general.max_depth, 10);
        // Other fields should have defaults
        assert_eq!(config.terminal.render_interval_ms, 16);
    }

    #[test]
    fn test_general_config_defaults() {
        let config = GeneralConfig::default();
        assert_eq!(config.max_depth, 5);
        assert_eq!(config.prefix_key, "Ctrl-a");
        assert!(!config.default_shell.is_empty());
    }

    #[test]
    fn test_appearance_config_defaults() {
        let config = AppearanceConfig::default();
        assert_eq!(config.theme, "default");
        assert_eq!(config.status_position, StatusPosition::Bottom);
        assert_eq!(config.border_style, BorderStyle::Rounded);
        assert!(config.show_pane_titles);
    }

    #[test]
    fn test_color_config_defaults() {
        let config = ColorConfig::default();
        assert_eq!(config.status_bg, "#282c34");
        assert_eq!(config.claude_thinking, "#e5c07b");
    }

    #[test]
    fn test_keybinding_config_defaults() {
        let config = KeybindingConfig::default();
        assert_eq!(config.split_horizontal, "prefix %");
        assert_eq!(config.detach, "prefix d");
    }

    #[test]
    fn test_terminal_config_defaults() {
        let config = TerminalConfig::default();
        assert_eq!(config.scrollback_lines, 10000);
        assert_eq!(config.render_interval_ms, 16);
        assert_eq!(config.parser_timeout_secs, 5);
    }

    #[test]
    fn test_claude_config_defaults() {
        let config = ClaudeConfig::default();
        assert!(config.detection_enabled);
        assert_eq!(config.detection_method, DetectionMethod::Pty);
        assert!(config.show_status);
        assert!(config.auto_resume);
    }

    #[test]
    fn test_persistence_config_defaults() {
        let config = PersistenceConfig::default();
        assert_eq!(config.checkpoint_interval_secs, 30);
        assert_eq!(config.max_wal_size_mb, 128);
        assert_eq!(config.screen_snapshot_lines, 500);
    }

    #[test]
    fn test_status_position_variants() {
        assert_eq!(StatusPosition::default(), StatusPosition::Bottom);

        // Test parsing from TOML config
        let toml_str = "[appearance]\nstatus_position = \"top\"";
        let config: AppConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.appearance.status_position, StatusPosition::Top);

        let toml_str = "[appearance]\nstatus_position = \"bottom\"";
        let config: AppConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.appearance.status_position, StatusPosition::Bottom);
    }

    #[test]
    fn test_border_style_variants() {
        assert_eq!(BorderStyle::default(), BorderStyle::Rounded);

        let toml_str = "[appearance]\nborder_style = \"single\"";
        let config: AppConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.appearance.border_style, BorderStyle::Single);

        let toml_str = "[appearance]\nborder_style = \"double\"";
        let config: AppConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.appearance.border_style, BorderStyle::Double);

        let toml_str = "[appearance]\nborder_style = \"none\"";
        let config: AppConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.appearance.border_style, BorderStyle::None);

        let toml_str = "[appearance]\nborder_style = \"rounded\"";
        let config: AppConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.appearance.border_style, BorderStyle::Rounded);
    }

    #[test]
    fn test_detection_method_variants() {
        assert_eq!(DetectionMethod::default(), DetectionMethod::Pty);

        let toml_str = "[claude]\ndetection_method = \"pty\"";
        let config: AppConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.claude.detection_method, DetectionMethod::Pty);

        let toml_str = "[claude]\ndetection_method = \"streamjson\"";
        let config: AppConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.claude.detection_method, DetectionMethod::StreamJson);

        let toml_str = "[claude]\ndetection_method = \"visual\"";
        let config: AppConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.claude.detection_method, DetectionMethod::Visual);
    }

    #[test]
    fn test_app_config_clone() {
        let config = AppConfig::default();
        let cloned = config.clone();
        assert_eq!(cloned.general.max_depth, config.general.max_depth);
    }

    #[test]
    fn test_app_config_debug() {
        let config = AppConfig::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("AppConfig"));
    }

    #[test]
    fn test_full_config_parse() {
        // Build the TOML string without using raw strings for hex colors
        let toml_str = "[general]
default_shell = \"/bin/zsh\"
max_depth = 3
prefix_key = \"Ctrl-b\"

[appearance]
theme = \"dark\"
status_position = \"top\"
border_style = \"double\"
show_pane_titles = false

[colors]
status_bg = \"#000000\"
status_fg = \"#ffffff\"
active_border = \"#ff0000\"
inactive_border = \"#333333\"
claude_thinking = \"#ffff00\"
claude_idle = \"#00ff00\"
claude_error = \"#ff0000\"

[keybindings]
split_horizontal = \"prefix |\"
split_vertical = \"prefix -\"
focus_left = \"prefix Left\"
focus_right = \"prefix Right\"
focus_up = \"prefix Up\"
focus_down = \"prefix Down\"
new_session = \"prefix n\"
detach = \"prefix d\"
list_sessions = \"prefix w\"

[terminal]
scrollback_lines = 5000
render_interval_ms = 8
parser_timeout_secs = 10

[claude]
detection_enabled = false
detection_method = \"visual\"
show_status = false
auto_resume = false

[persistence]
checkpoint_interval_secs = 60
max_wal_size_mb = 256
screen_snapshot_lines = 1000
";

        let config: AppConfig = toml::from_str(toml_str).unwrap();

        // General
        assert_eq!(config.general.default_shell, "/bin/zsh");
        assert_eq!(config.general.max_depth, 3);
        assert_eq!(config.general.prefix_key, "Ctrl-b");

        // Appearance
        assert_eq!(config.appearance.theme, "dark");
        assert_eq!(config.appearance.status_position, StatusPosition::Top);
        assert_eq!(config.appearance.border_style, BorderStyle::Double);
        assert!(!config.appearance.show_pane_titles);

        // Colors
        assert_eq!(config.colors.status_bg, "#000000");

        // Keybindings
        assert_eq!(config.keybindings.split_horizontal, "prefix |");

        // Terminal
        assert_eq!(config.terminal.scrollback_lines, 5000);
        assert_eq!(config.terminal.render_interval_ms, 8);

        // Claude
        assert!(!config.claude.detection_enabled);
        assert_eq!(config.claude.detection_method, DetectionMethod::Visual);

        // Persistence
        assert_eq!(config.persistence.checkpoint_interval_secs, 60);
    }

    #[test]
    fn test_status_position_clone_copy() {
        let pos = StatusPosition::Top;
        let cloned = pos.clone();
        let copied = pos;
        assert_eq!(pos, cloned);
        assert_eq!(pos, copied);
    }

    #[test]
    fn test_border_style_clone_copy() {
        let style = BorderStyle::Double;
        let cloned = style.clone();
        let copied = style;
        assert_eq!(style, cloned);
        assert_eq!(style, copied);
    }

    #[test]
    fn test_detection_method_clone_copy() {
        let method = DetectionMethod::StreamJson;
        let cloned = method.clone();
        let copied = method;
        assert_eq!(method, cloned);
        assert_eq!(method, copied);
    }

    #[test]
    fn test_config_sections_debug() {
        assert!(format!("{:?}", GeneralConfig::default()).contains("GeneralConfig"));
        assert!(format!("{:?}", AppearanceConfig::default()).contains("AppearanceConfig"));
        assert!(format!("{:?}", ColorConfig::default()).contains("ColorConfig"));
        assert!(format!("{:?}", KeybindingConfig::default()).contains("KeybindingConfig"));
        assert!(format!("{:?}", TerminalConfig::default()).contains("TerminalConfig"));
        assert!(format!("{:?}", ClaudeConfig::default()).contains("ClaudeConfig"));
        assert!(format!("{:?}", PersistenceConfig::default()).contains("PersistenceConfig"));
    }

    #[test]
    fn test_enum_debug() {
        assert!(format!("{:?}", StatusPosition::Top).contains("Top"));
        assert!(format!("{:?}", BorderStyle::Single).contains("Single"));
        assert!(format!("{:?}", DetectionMethod::Pty).contains("Pty"));
    }
}

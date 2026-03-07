use anyhow::Result;
use figment::{
    providers::{Env, Format, Yaml},
    Figment,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

/// Top-level shell configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellConfig {
    #[serde(default)]
    pub shell: ShellSettings,
    #[serde(default)]
    pub history: HistoryConfig,
    #[serde(default)]
    pub theme: ThemeConfig,
    #[serde(default)]
    pub prompt: PromptConfig,
    #[serde(default)]
    pub tools: BTreeMap<String, ToolConfig>,
    #[serde(default)]
    pub fuzzy: FuzzyConfig,
    #[serde(default)]
    pub completion: CompletionConfig,
    #[serde(default)]
    pub plugins: BTreeMap<String, PluginConfig>,
    #[serde(default)]
    pub keybindings: BTreeMap<String, BTreeMap<String, String>>,
    #[serde(default)]
    pub aliases: BTreeMap<String, BTreeMap<String, AliasValue>>,
    #[serde(default)]
    pub widgets: BTreeMap<String, WidgetConfig>,
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            shell: ShellSettings::default(),
            history: HistoryConfig::default(),
            theme: ThemeConfig::default(),
            prompt: PromptConfig::default(),
            tools: default_tools(),
            fuzzy: FuzzyConfig::default(),
            completion: CompletionConfig::default(),
            plugins: default_plugins(),
            keybindings: default_keybindings(),
            aliases: BTreeMap::new(),
            widgets: default_widgets(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellSettings {
    #[serde(default = "default_true")]
    pub vi_mode: bool,
    #[serde(default = "default_key_timeout")]
    pub key_timeout: u32,
    #[serde(default = "default_func_nest")]
    pub func_nest: u32,
    #[serde(default = "default_true")]
    pub auto_cd: bool,
    #[serde(default = "default_true")]
    pub extended_glob: bool,
    #[serde(default = "default_true")]
    pub interactive_comments: bool,
}

impl Default for ShellSettings {
    fn default() -> Self {
        Self {
            vi_mode: true,
            key_timeout: 20,
            func_nest: 1000,
            auto_cd: true,
            extended_glob: true,
            interactive_comments: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryConfig {
    #[serde(default = "default_histfile")]
    pub file: String,
    #[serde(default = "default_histsize")]
    pub size: usize,
    #[serde(default = "default_true")]
    pub dedup: bool,
    #[serde(default = "default_true")]
    pub share: bool,
    #[serde(default = "default_true")]
    pub ignore_space: bool,
}

impl Default for HistoryConfig {
    fn default() -> Self {
        Self {
            file: default_histfile(),
            size: 1_000_000,
            dedup: true,
            share: true,
            ignore_space: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    #[serde(default = "default_theme")]
    pub name: String,
    #[serde(default)]
    pub colors: BTreeMap<String, String>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        let mut colors = BTreeMap::new();
        // Nord palette
        colors.insert("nord0".into(), "#2E3440".into());
        colors.insert("nord1".into(), "#3B4252".into());
        colors.insert("nord2".into(), "#434C5E".into());
        colors.insert("nord3".into(), "#4C566A".into());
        colors.insert("nord4".into(), "#D8DEE9".into());
        colors.insert("nord5".into(), "#E5E9F0".into());
        colors.insert("nord6".into(), "#ECEFF4".into());
        colors.insert("nord7".into(), "#8FBCBB".into());
        colors.insert("nord8".into(), "#88C0D0".into());
        colors.insert("nord9".into(), "#81A1C1".into());
        colors.insert("nord10".into(), "#5E81AC".into());
        colors.insert("nord11".into(), "#BF616A".into());
        colors.insert("nord12".into(), "#D08770".into());
        colors.insert("nord13".into(), "#EBCB8B".into());
        colors.insert("nord14".into(), "#A3BE8C".into());
        colors.insert("nord15".into(), "#B48EAD".into());
        Self {
            name: "nord".into(),
            colors,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptConfig {
    #[serde(default = "default_prompt_engine")]
    pub engine: String,
    #[serde(default)]
    pub config_path: Option<String>,
    #[serde(default = "default_true")]
    pub defer: bool,
}

impl Default for PromptConfig {
    fn default() -> Self {
        Self {
            engine: "starship".into(),
            config_path: None,
            defer: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfig {
    #[serde(default)]
    pub replaces: Option<StringOrVec>,
    #[serde(default)]
    pub alias: Option<String>,
    #[serde(default = "default_alias_style")]
    pub alias_style: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrVec {
    Single(String),
    Multiple(Vec<String>),
}

impl StringOrVec {
    pub fn as_vec(&self) -> Vec<&str> {
        match self {
            StringOrVec::Single(s) => vec![s.as_str()],
            StringOrVec::Multiple(v) => v.iter().map(|s| s.as_str()).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzyConfig {
    #[serde(default = "default_fuzzy_backend")]
    pub backend: String,
    #[serde(default = "default_true")]
    pub ctrl_t: bool,
    #[serde(default)]
    pub ctrl_r: bool,
    #[serde(default = "default_true")]
    pub alt_c: bool,
}

impl Default for FuzzyConfig {
    fn default() -> Self {
        Self {
            backend: "skim".into(),
            ctrl_t: true,
            ctrl_r: false, // handled by atuin
            alt_c: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionConfig {
    #[serde(default = "default_completion_engine")]
    pub engine: String,
    #[serde(default = "default_cache_hours")]
    pub cache_hours: u32,
    #[serde(default = "default_fuzzy_ratio")]
    pub fuzzy_error_ratio: f64,
}

impl Default for CompletionConfig {
    fn default() -> Self {
        Self {
            engine: "zsh".into(),
            cache_hours: 24,
            fuzzy_error_ratio: 0.33,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    #[serde(default)]
    pub source: PluginSource,
    #[serde(default = "default_priority")]
    pub priority: u32,
    #[serde(default)]
    pub defer: bool,
    #[serde(default)]
    pub init: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginSource {
    #[serde(default)]
    pub github: Option<String>,
    #[serde(default)]
    pub rev: Option<String>,
    #[serde(default)]
    pub nixpkgs: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AliasValue {
    Simple(String),
    Complex {
        command: String,
        #[serde(default)]
        noglob: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetConfig {
    #[serde(rename = "type")]
    pub widget_type: String,
    #[serde(default)]
    pub bind: Option<String>,
}

// Default value helpers
fn default_true() -> bool { true }
fn default_key_timeout() -> u32 { 20 }
fn default_func_nest() -> u32 { 1000 }
fn default_histfile() -> String { "${XDG_STATE_HOME:-$HOME/.local/state}/zsh/history".into() }
fn default_histsize() -> usize { 1_000_000 }
fn default_theme() -> String { "nord".into() }
fn default_prompt_engine() -> String { "starship".into() }
fn default_alias_style() -> String { "transparent".into() }
fn default_fuzzy_backend() -> String { "skim".into() }
fn default_completion_engine() -> String { "zsh".into() }
fn default_cache_hours() -> u32 { 24 }
fn default_fuzzy_ratio() -> f64 { 0.33 }
fn default_priority() -> u32 { 50 }

fn default_tools() -> BTreeMap<String, ToolConfig> {
    let mut m = BTreeMap::new();
    m.insert("bat".into(), ToolConfig { replaces: Some(StringOrVec::Single("cat".into())), alias: None, alias_style: "transparent".into() });
    m.insert("eza".into(), ToolConfig { replaces: Some(StringOrVec::Single("ls".into())), alias: None, alias_style: "function".into() });
    m.insert("dust".into(), ToolConfig { replaces: Some(StringOrVec::Single("du".into())), alias: None, alias_style: "transparent".into() });
    m.insert("procs".into(), ToolConfig { replaces: Some(StringOrVec::Single("ps".into())), alias: None, alias_style: "transparent".into() });
    m.insert("bottom".into(), ToolConfig { replaces: Some(StringOrVec::Multiple(vec!["top".into(), "htop".into()])), alias: None, alias_style: "transparent".into() });
    m
}

fn default_plugins() -> BTreeMap<String, PluginConfig> {
    let mut m = BTreeMap::new();
    m.insert("zoxide".into(), PluginConfig { source: PluginSource { nixpkgs: Some("zoxide".into()), ..Default::default() }, priority: 40, defer: false, init: Some("eval \"$(zoxide init zsh --cmd cd)\"".into()) });
    m.insert("direnv".into(), PluginConfig { source: PluginSource { nixpkgs: Some("direnv".into()), ..Default::default() }, priority: 90, defer: false, init: Some("eval \"$(direnv hook zsh)\"".into()) });
    m.insert("atuin".into(), PluginConfig { source: PluginSource { nixpkgs: Some("atuin".into()), ..Default::default() }, priority: 50, defer: false, init: Some("eval \"$(atuin init zsh --disable-up-arrow)\"".into()) });
    m
}

fn default_keybindings() -> BTreeMap<String, BTreeMap<String, String>> {
    let mut modes = BTreeMap::new();
    let mut vicmd = BTreeMap::new();
    vicmd.insert("k".into(), "up-line-or-history".into());
    vicmd.insert("j".into(), "down-line-or-history".into());
    vicmd.insert("^A".into(), "beginning-of-line".into());
    vicmd.insert("^E".into(), "end-of-line".into());
    vicmd.insert("/".into(), "history-incremental-search-backward".into());
    vicmd.insert("?".into(), "history-incremental-search-forward".into());
    vicmd.insert("v".into(), "edit-command-line".into());
    vicmd.insert("y".into(), "vi-yank-clip".into());
    modes.insert("vicmd".into(), vicmd);

    let mut viins = BTreeMap::new();
    viins.insert("^A".into(), "beginning-of-line".into());
    viins.insert("^E".into(), "end-of-line".into());
    viins.insert("^?".into(), "backward-delete-char".into());
    viins.insert("^H".into(), "backward-delete-char".into());
    viins.insert("^[[3~".into(), "delete-char".into());
    viins.insert("^[[1;5C".into(), "forward-word".into());
    viins.insert("^[[1;5D".into(), "backward-word".into());
    viins.insert("^[[1;3C".into(), "forward-word".into());
    viins.insert("^[[1;3D".into(), "backward-word".into());
    viins.insert("^[[A".into(), "up-line-or-beginning-search".into());
    viins.insert("^[[B".into(), "down-line-or-beginning-search".into());
    viins.insert("^P".into(), "up-line-or-beginning-search".into());
    viins.insert("^N".into(), "down-line-or-beginning-search".into());
    modes.insert("viins".into(), viins);

    modes
}

fn default_widgets() -> BTreeMap<String, WidgetConfig> {
    let mut m = BTreeMap::new();
    m.insert("vi-yank-clip".into(), WidgetConfig { widget_type: "clipboard-yank".into(), bind: None });
    m.insert("skim-file-content-widget".into(), WidgetConfig { widget_type: "fuzzy-grep".into(), bind: Some("^F".into()) });
    m
}

/// Load configuration using Figment (defaults -> YAML file -> env vars)
pub fn load(path: &Path) -> Result<ShellConfig> {
    let config: ShellConfig = Figment::new()
        .merge(Yaml::file(path))
        .merge(Env::prefixed("BM_SHELL_").split("__"))
        .extract()
        .map_err(|e| anyhow::anyhow!("config error: {e}"))?;
    Ok(config)
}

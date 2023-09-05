use std::path::PathBuf;

use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use crate::config::frontend::auth::{AuthConfig, AuthState};

/// The frontend authentication configuration module.
pub mod auth;

use crate::frontend::helpers;

fn enabled_def() -> bool {
    true
}

fn base_address_def() -> String {
    "/".to_string()
}

/// Represent a link entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    /// The name of the related link.
    pub name: String,
    /// The target of the related link.
    pub href: String,
}

/// The asset configuration options struct.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetsConfig {
    /// Assets directory path.
    pub path: PathBuf,
}

/// The templates configuration options struct.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemplatesConfig {
    /// Templates directory path.
    pub path: PathBuf,
}

/// The session-handling configuration struct.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionsConfig {
    /// The name of the session's cookie.
    pub cookie_name: String,
    /// The secret to use to sign cookies with.
    pub secret: String,
}

/// The frontend configuration struct.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrontendConfig {
    /// Is the frontend enabled?
    #[serde(default = "enabled_def")]
    pub enabled: bool,
    /// The instance's title.
    pub title: Option<String>,
    /// The instance's description.
    pub description: Option<String>,
    /// The path to the instance's favicon.
    pub favicon: Option<String>,
    /// Some related links.
    pub links: Option<Vec<Link>>,
    /// Whether to disallow anonymous browsing of the registry.
    pub login_required: bool,
    /// Assets configuration options.
    pub assets: AssetsConfig,
    /// Templates configuration options.
    pub templates: TemplatesConfig,
    /// The session-handling configuration.
    pub sessions: SessionsConfig,
    /// The authentication configuration.
    pub auth: AuthConfig,
    /// Base address of app
    #[serde(default = "base_address_def")]
    pub base_address: String,
}

/// The frontend state struct, created from [FrontendConfig].
pub struct FrontendState {
    /// The authentication state (like OAuth clients).
    pub auth: AuthState,
    /// The Handlebars rendering struct.
    pub handlebars: Handlebars<'static>,
    /// The frontend configuration.
    pub config: FrontendConfig,
}

impl From<FrontendConfig> for FrontendState {
    fn from(config: FrontendConfig) -> FrontendState {
        let auth = AuthState::new(&config.auth).expect("could not initialize authentication state");

        let mut handlebars = Handlebars::new();
        handlebars
            .register_templates_directory(".hbs", &config.templates.path)
            .expect("could not register templates directory to Handlebars");

        handlebars.register_helper("equal", Box::new(helpers::hbs_equal));

        FrontendState {
            auth,
            handlebars,
            config,
        }
    }
}

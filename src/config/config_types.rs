use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CStreamConfig {
    pub engine: Engine,
    pub plugins: Option<Vec<PluginConfig>>,
    pub sources: Option<Vec<SourceConfig>>,
    pub pipeline: Option<PipelineConfig>,
}

// Engine config
#[derive(Debug, Serialize, Deserialize)]
pub struct Engine {
    pub version: f64,
    pub plugins_enabled: bool,
    pub schedule_interval: String,
}

// Plugin config
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub plugin_type: String,
    pub path: String,
}

// Source config
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SourceConfig {
    #[serde(rename = "api")]
    Api(ApiSource),
    #[serde(rename = "webhook")]
    Webhook(WebhookSource),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiSource {
    pub id: String,
    pub interval: Option<String>,
    pub request: ApiRequest,
    #[serde(default)]
    pub response_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiRequest {
    pub method: Option<String>,
    pub url: String,
    #[serde(default)]
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookSource {
    pub id: String,
    pub listen_port: u16,
    pub path: String,
    #[serde(default)]
    pub secret: Option<String>,
}

// Pipeline config
#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineConfig {
    pub transforms: Vec<TransformConfig>,
    pub outputs: Vec<OutputConfig>,
}

// Transform config
#[derive(Debug, Serialize, Deserialize)]
pub struct TransformConfig {
    #[serde(rename = "type")]
    pub kind: TransformType,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub field: Option<String>,
    #[serde(default)]
    pub op: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub map: Option<HashMap<String, String>>,
    #[serde(default)]
    pub fields: Option<HashMap<String, String>>,
    #[serde(default)]
    pub name: Option<String>,
}

// Transform types
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransformType {
    Filter,
    Rename,
    AddFields,
    Plugin,
}

// Output config
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OutputConfig {
    #[serde(rename = "webhook")]
    Webhook(WebhookOutput),
    #[serde(rename = "postgres")]
    Postgres(PostgresOutput),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookOutput {
    pub id: Option<String>,
    pub url: String,
    #[serde(default)]
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostgresOutput {
    pub id: Option<String>,
    pub connection: DbConnection,
    pub table: String,
    #[serde(default)]
    pub field_mapping: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbConnection {
    pub host: String,
    pub port: u16,
    pub db: String,
    pub user: String,
    pub password: String,
}

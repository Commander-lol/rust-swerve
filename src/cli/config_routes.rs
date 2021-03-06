use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct RouteHandler {
    pub route: String,
    pub response: Option<ResponseHandler>,
    pub script: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ResponseHandler {
    failure_rate: Option<u32>,
    #[serde(default="get_default_headers")]
    headers: HashMap<String, String>,
}

fn get_default_headers() -> HashMap<String, String> { HashMap::new() }

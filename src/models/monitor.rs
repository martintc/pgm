use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Monitors {
    pub databases: Option<Vec<Monitor>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Monitor {
    pub host: Option<String>,
    pub port: Option<i32>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub database_name: Option<String>,
}

impl std::fmt::Display for Monitor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "- {}", self.host.clone().unwrap().as_str())
    }
}

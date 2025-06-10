use serde::{Deserialize, Serialize};

/// Request structure for MCP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerRequest {
    pub id: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    pub args: Vec<String>,
    pub command: String,
    #[serde(rename = "type")]
    pub server_type: String,
    pub name: String,
}

/// Response structure for MCP server information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerResponse {
    pub id: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    pub args: Vec<String>,
    pub command: String,
    #[serde(rename = "type")]
    pub server_type: String,
    pub name: String,
}

/// Request structure for updating MCP server list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfigRequest {
    pub servers: Vec<ServerRequest>,
}

/// Response structure for MCP server list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfigResponse {
    pub servers: Vec<ServerResponse>,
}

/// Response structure for listing servers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerListResponse {
    pub servers: Vec<ServerResponse>,
    pub total_count: usize,
}

// Internal structures for database operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct DatabaseEntry {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub json_data: Option<serde_json::Value>,
}

// Internal structure that matches Cherry Studio's actual format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CherryMcpConfig {
    pub servers: Vec<ServerResponse>,
}

impl From<ServerRequest> for ServerResponse {
    fn from(req: ServerRequest) -> Self {
        ServerResponse {
            id: req.id,
            is_active: req.is_active,
            args: req.args,
            command: req.command,
            server_type: req.server_type,
            name: req.name,
        }
    }
}

impl From<ServerResponse> for ServerRequest {
    fn from(resp: ServerResponse) -> Self {
        ServerRequest {
            id: resp.id,
            is_active: resp.is_active,
            args: resp.args,
            command: resp.command,
            server_type: resp.server_type,
            name: resp.name,
        }
    }
}

impl From<McpConfigRequest> for McpConfigResponse {
    fn from(req: McpConfigRequest) -> Self {
        McpConfigResponse {
            servers: req.servers.into_iter().map(ServerResponse::from).collect(),
        }
    }
}

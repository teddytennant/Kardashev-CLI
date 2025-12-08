mod device_code_auth;
mod pkce;
mod server;

pub use device_code_auth::run_device_code_login;
pub use server::LoginServer;
pub use server::ServerOptions;
pub use server::ShutdownHandle;
pub use server::run_login_server;

// Re-export commonly used auth types and helpers from kardashev-core for compatibility
pub use kardashev_app_server_protocol::AuthMode;
pub use kardashev_core::AuthManager;
pub use kardashev_core::CodexAuth;
pub use kardashev_core::auth::AuthDotJson;
pub use kardashev_core::auth::CLIENT_ID;
pub use kardashev_core::auth::KARDASHEV_API_KEY_ENV_VAR;
pub use kardashev_core::auth::OPENAI_API_KEY_ENV_VAR;
pub use kardashev_core::auth::login_with_api_key;
pub use kardashev_core::auth::logout;
pub use kardashev_core::auth::save_auth;
pub use kardashev_core::token_data::TokenData;

// backend/src/config.rs
#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub openai_api_key: String,
    pub model: String,
    pub system_prompt: String,
    pub response_prompt: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or("mysql://smrt:smrtpass@localhost/smrt_mcp".to_string());

        let openai_api_key = std::env::var("OPENAI_API_KEY").unwrap_or("sk-xxxx".to_string());

        let model = std::env::var("OPENAI_MODEL").unwrap_or("gpt-4o-mini".to_string());

        let system_prompt = std::env::var("SYSTEM_PROMPT")
            .unwrap_or("You are an MCP intent router for SMRT IT Department.".to_string());

        let response_prompt = std::env::var("RESPONSE_PROMPT").unwrap_or(
            "You are an assistant for SMRT Singapore IT Department. Summarize and explain monitoring data clearly to the user.".to_string(),
        );

        Self {
            database_url,
            openai_api_key,
            model,
            system_prompt,
            response_prompt,
        }
    }
}

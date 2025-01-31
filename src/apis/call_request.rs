use crate::models::general::llm::{ChatCompletion, Message};
use dotenv::dotenv;
use std::env;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};

// Calls Large Language Model (i.e. GTP-4)
pub async fn call_gpt(messages: Vec<Message>) {
    dotenv().ok();

    // Extracts API Key infromations
    let api_key: String =
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found in env variables");
    let api_org: String = env::var("OPENAI_ORG").expect("OPENAI_ORG not found in env variables");

    // Confirms endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // Creates headers
    let mut headers: HeaderMap = HeaderMap::new();

    // Creates API key header
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );

    // Creates OpenAI Org header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str()).unwrap(),
    );

    // Creates client
    let client = Client::builder().default_headers(headers).build().unwrap();

    // Create chat completion
    let chat_completion: ChatCompletion = ChatCompletion {
        model: "gpt-3.5-turbo".to_string(),
        messages,
        temperature: 0.1,
    };

    // Troubleshooting
    let res_raw = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .unwrap();

    dbg!(res_raw.text().await.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "Hi there, this is a test. Give me a short response".to_string(),
        };

        let messages = vec![message];

        call_gpt(messages).await;
    }
}

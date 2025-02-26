use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::{apis::call_request::call_gpt, models::general::llm::Message};

use super::command_line::PrintCommand;
use std::fs;

const CODE_TEMPLATE_PATH: &str =
    "/Users/caique/Documents/codes/rust/autogpt-course/web_template/src/code_template.rs";
const EXECUTE_MAIN_PATH: &str =
    "/Users/caique/Documents/codes/rust/autogpt-course/web_template/src/main.rs";
const API_SCHEMA_PATH: &str =
    "/Users/caique/Documents/codes/rust/autogpt-course/auto_gippity/schemas/api_schema.json";

// Extends AI function to encourage certain specific output
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    // the "ai_func" is a function that when executed prints is own informations to be used in LLM
    let ai_function_str = ai_func(func_input);

    // Extends the string to encourage only printing the output
    let msg: String = format!(
        "FUNCTION {}
        INSTRUCTION: You are a function printer. You ONLY print the results of functions.
        Nothing else. No commentary, please. Here is the input to the function: {}.
        Print out what the function will return.",
        ai_function_str, func_input
    );

    // Returns message
    Message {
        role: "system".to_string(),
        content: msg,
    }
}

// Performs call to LLM GPT
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    // Extends AI function
    let extended_msg: Message = extend_ai_function(function_pass, &msg_context);

    // Prints current status
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    // Gets LLM response
    let llm_response_res = call_gpt(vec![extended_msg.clone()]).await;

    // Handles success or try again
    let llm_response: String = match llm_response_res {
        Ok(llm_resp) => llm_resp,
        Err(_) => call_gpt(vec![extended_msg.clone()])
            .await
            .expect("Failed twice to call OpenAI"),
    };

    return llm_response;
}

// Performs call to LLM GPT - Decoded
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response: String =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;

    let decoded_response: T = serde_json::from_str(llm_response.as_str())
        .expect("Failed to decode AI response from serde_json");

    return decoded_response;
}

// Check whether request URL is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

// Get code template
pub fn read_code_template_contents() -> String {
    let path: String = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}

// Save new backend code
pub fn save_backend_code(contents: &str) {
    let path: String = String::from(EXECUTE_MAIN_PATH);
    fs::write(path, contents).expect("Failed write main.rs file");
}

// Save the JSON API endpoint schema
pub fn save_api_endpoints(api_endpoints: &str) {
    let path: String = String::from(API_SCHEMA_PATH);
    fs::write(path, api_endpoints).expect("Failed write main.rs file");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extend_ai_function() {
        let extended_msg: Message =
            extend_ai_function(convert_user_input_to_goal, "dummy variable");
        assert_eq!(extended_msg.role, "system".to_string());
    }

    #[tokio::test]
    async fn tests_ai_task_request() {
        let ai_func_param = "Build me a webserver for making stock price API requests.".to_string();

        let res = ai_task_request(
            ai_func_param,
            "Managing Agent",
            "Defining user requirements",
            convert_user_input_to_goal,
        )
        .await;

        assert!(res.len() > 20);
    }
}

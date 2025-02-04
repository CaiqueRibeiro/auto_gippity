use crate::{
    apis::call_request::call_gpt,
    models::general::llm::{self, Message},
};

use super::command_line::PrintCommand;

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

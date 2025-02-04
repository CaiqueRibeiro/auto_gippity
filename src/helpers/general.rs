use crate::models::general::llm::Message;

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
}

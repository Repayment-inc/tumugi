use super::models::{Message, ResponseFormat, Tool, ToolChoice};
use super::models::{Role, StandardMessage, CustomUserMessage, CustomSystemMessage, CustomAssistantMessage, ToolMessage, ToolMessageContent};
use super::client::ChatOpenAI;
use super::error::ChatOpenAIError;
use std::collections::HashMap;
use serde_json::Value;
/// Builder for configuring and creating a ChatOpenAI instance
#[derive(Debug, Clone)]
pub struct ChatOpenAIBuilder {
    messages: Vec<Message>,
    model: String,
    frequency_penalty: Option<f32>,
    logit_bias: Option<HashMap<i32, f32>>,
    logprobs: Option<bool>,
    top_logprobs: Option<i32>,
    max_completion_tokens: Option<i32>,
    n: Option<i32>,
    presence_penalty: Option<f32>,
    response_format: Option<ResponseFormat>,
    seed: Option<i32>,
    service_tier: Option<String>,
    stop: Option<Vec<String>>,
    stream: Option<bool>,
    stream_options: Option<HashMap<String, bool>>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    tools: Option<Vec<Tool>>,
    tool_choice: Option<ToolChoice>,
    parallel_tool_calls: Option<bool>,
    user: Option<String>,

    // 下記非推奨パラメータ (2023年9月16日時点)
    max_tokens: Option<i32>,
    function_call: Option<Value>,
    functions: Option<Vec<Value>>,
}

impl ChatOpenAIBuilder {
    /// Creates a new ChatOpenAIBuilder with the specified model
    pub fn new(model: String) -> Self {
        Self {
            messages: Vec::new(),
            model,
            frequency_penalty: None,
            logit_bias: None,
            logprobs: None,
            top_logprobs: None,
            max_completion_tokens: None,
            n: None,
            presence_penalty: None,
            response_format: None,
            seed: None,
            service_tier: None,
            stop: None,
            stream: None,
            stream_options: None,
            temperature: None,
            top_p: None,
            tools: None,
            tool_choice: None,
            parallel_tool_calls: None,
            user: None,

            // 下記非推奨パラメータ (2023年9月16日時点)
            max_tokens: None,
            function_call: None,
            functions: None,
        }
    }


    /// Adds a message to the conversation
    pub fn add_message(mut self, role: Role, content: String) -> Self {
        // self.messages.push(Message::Standard{ role, content });
        self.messages.push(Message::Standard(StandardMessage { role, content }));
        self
    }
    
    pub fn add_user_message(mut self, content: String, name: Option<String>) -> Self {
        self.messages.push(Message::CustomUser(Box::new(CustomUserMessage { role: Role::User, content, name })));
        self
    }

    pub fn add_system_message(mut self, content: String, name: Option<String>) -> Self {
        self.messages.push(Message::CustomSystem(Box::new(CustomSystemMessage { role: Role::System, content, name })));
        self
    }

    pub fn add_assistant_message(mut self, content: Option<String>, name: Option<String>, refusal: Option<String>, tool_calls: Option<Vec<Value>>, function_call: Option<Value>) -> Self {
        self.messages.push(Message::CustomAssistant(Box::new(CustomAssistantMessage { role: Role::Assistant, content,name, refusal, tool_calls, function_call})));
        self
    }

    pub fn add_tool_message(mut self, content: ToolMessageContent, tool_call_id: String) {
        self.messages.push(Message::Tool(Box::new(ToolMessage { role: Role::Tool, content, tool_call_id })));
    }

    /// Sets the temperature for response generation
    /// Temperature controls randomness. Higher values make output more random, lower values make it more focused and deterministic.
    pub fn temperature(self, temp: f32) -> Self {
        Self {
            temperature: Some(temp),
            ..self
        }
    }

    /// Sets the maximum number of tokens that can be generated in the chat completion
    pub fn max_completion_tokens(self, tokens: i32) -> Self {
        Self {
            max_completion_tokens: Some(tokens),
            ..self
        }
    }

    /// Enables or disables streaming of responses
    pub fn stream(self, stream: bool) -> Self {
        Self {
            stream: Some(stream),
            ..self
        }
    }

    /// Sets the frequency penalty
    /// Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
    pub fn frequency_penalty(self, penalty: f32) -> Self {
        Self {
            frequency_penalty: Some(penalty),
            ..self
        }
    }

    /// Sets the presence penalty
    /// Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.
    pub fn presence_penalty(self, penalty: f32) -> Self {
        Self {
            presence_penalty: Some(penalty),
            ..self
        }
    }

    /// Sets the top_p value
    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass.
    pub fn top_p(self, top_p: f32) -> Self {
        Self {
            top_p: Some(top_p),
            ..self
        }
    }

    /// Sets the number of chat completion choices to generate
    pub fn n(self, n: i32) -> Self {
        Self { n: Some(n), ..self }
    }

    /// Sets the stop sequences
    /// Up to 4 sequences where the API will stop generating further tokens.
    pub fn stop(self, stop: Vec<String>) -> Self {
        Self {
            stop: Some(stop),
            ..self
        }
    }

    /// Sets the logit bias
    /// Modify the likelihood of specified tokens appearing in the completion.
    pub fn logit_bias(self, bias: HashMap<i32, f32>) -> Self {
        Self {
            logit_bias: Some(bias),
            ..self
        }
    }

    /// Sets the user identifier
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    pub fn user(self, user: String) -> Self {
        Self {
            user: Some(user),
            ..self
        }
    }

    /// Sets the response format
    /// An object specifying the format that the model must output.
    pub fn response_format(self, format: ResponseFormat) -> Self {
        Self {
            response_format: Some(format),
            ..self
        }
    }

    /// Sets the seed for deterministic sampling
    pub fn seed(self, seed: i32) -> Self {
        Self {
            seed: Some(seed),
            ..self
        }
    }

    /// Sets the service tier
    /// Specifies the latency tier to use for processing the request.
    pub fn service_tier(self, tier: String) -> Self {
        Self {
            service_tier: Some(tier),
            ..self
        }
    }

    /// Sets the tools
    /// A list of tools the model may call. Currently, only functions are supported as a tool.
    pub fn tools(self, tools: Vec<Tool>) -> Self {
        Self {
            tools: Some(tools),
            ..self
        }
    }

    /// Sets the tool choice
    /// Controls which (if any) tool is called by the model.
    pub fn tool_choice(self, choice: ToolChoice) -> Self {
        Self {
            tool_choice: Some(choice),
            ..self
        }
    }

    /// Enables or disables parallel function calling during tool use
    pub fn parallel_tool_calls(self, enable: bool) -> Self {
        Self {
            parallel_tool_calls: Some(enable),
            ..self
        }
    }

    /// Sets the maximum number of tokens to generate (Deprecated as of 2023-09-16)
    /// This parameter is deprecated in favor of max_completion_tokens
    #[deprecated(
        since = "2023-09-16",
        note = "Use max_completion_tokens instead"
    )]
    pub fn max_tokens(self, tokens: i32) -> Self {
        Self {
            max_tokens: Some(tokens),
            ..self
        }
    }

    /// Sets the function call (Deprecated as of 2023-09-16)
    /// Controls which (if any) function is called by the model.
    #[deprecated(
        since = "2023-09-16",
        note = "Use tool_choice instead"
    )]
    pub fn function_call(self, call: serde_json::Value) -> Self {
        Self {
            function_call: Some(call),
            ..self
        }
    }

    /// Sets the functions (Deprecated as of 2023-09-16)
    /// A list of functions the model may generate JSON inputs for.
    #[deprecated(
        since = "2023-09-16",
        note = "Use tools instead"
    )]
    pub fn functions(self, functions: Vec<Value>) -> Self {
        Self {
            functions: Some(functions),
            ..self
        }
    }

    /// Builds the ChatOpenAI instance
    pub fn build(self) -> Result<ChatOpenAI, ChatOpenAIError> {
        // Perform validation
        if let Some(temp) = self.temperature {
            if temp < 0.0 || temp > 2.0 {
                return Err(ChatOpenAIError::InvalidTemperature);
            }
        }

        if self.max_tokens.is_some() {
            println!("Warning: max_tokens is deprecated. Use max_completion_tokens instead.");
        }

        Ok(ChatOpenAI {
            messages: self.messages,
            model: self.model,
            frequency_penalty: self.frequency_penalty,
            logit_bias: self.logit_bias,
            logprobs: self.logprobs,
            top_logprobs: self.top_logprobs,
            max_completion_tokens: self.max_completion_tokens,
            n: self.n,
            presence_penalty: self.presence_penalty,
            response_format: self.response_format,
            seed: self.seed,
            service_tier: self.service_tier,
            stop: self.stop,
            stream: self.stream,
            stream_options: self.stream_options,
            temperature: self.temperature,
            top_p: self.top_p,
            tools: self.tools,
            tool_choice: self.tool_choice,
            parallel_tool_calls: self.parallel_tool_calls,
            user: self.user,
        
            // 下記非推奨パラメータ (2023年9月16日時点)
            max_tokens: self.max_tokens,    
            function_call: self.function_call,
            functions: self.functions,
        })
    }
}
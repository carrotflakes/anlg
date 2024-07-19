use gptcl::{
    model::{ChatMessage, ChatRequest},
    GptClient,
};
use gptcl_hyper::HyperClient;

pub type Gpt = GptClient<HyperClient>;

pub fn new_gpt(openai_api_key: String) -> Gpt {
    let client = GptClient::new(HyperClient::new(), openai_api_key);
    client
}

pub fn new_request(messages: Vec<ChatMessage>) -> ChatRequest {
    let mut req = ChatRequest::from_model(gptcl::MODEL_GPT_4O_MINI.to_string());
    req.temperature = Some(0.0);
    req.messages = messages;
    req.response_format = Some(gptcl::model::ResponseFormat::Json);
    req
}

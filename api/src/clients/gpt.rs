use gptcl::{model::ChatMessage, GptClient};
use gptcl_hyper::HyperClient;

pub struct Gpt {
    client: GptClient<HyperClient>,
}

impl Gpt {
    pub fn new(openai_api_key: String) -> Self {
        Gpt {
            client: GptClient::new(
                HyperClient::new(),
                openai_api_key,
                gptcl::MODEL_GPT_3_5_TURBO,
            ),
        }
    }

    pub async fn simple_request(&self, prompt: &str) -> String {
        let res = self
            .client
            .call(&[ChatMessage::from_user(prompt.to_owned())])
            .await
            .unwrap();
        res.content.unwrap()
    }
}

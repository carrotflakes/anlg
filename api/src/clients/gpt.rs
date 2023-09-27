use gptcl::GptClient;
use gptcl_hyper::HyperClient;

pub type Gpt = GptClient<HyperClient>;

pub fn new_gpt(openai_api_key: String) -> Gpt {
    let mut client = GptClient::new(
        HyperClient::new(),
        openai_api_key,
        gptcl::MODEL_GPT_3_5_TURBO,
    );
    client.temperature = Some(0.0);
    client
}

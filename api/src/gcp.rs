use std::{io::Read, sync::Mutex};

const TTL: i64 = 3600;

pub struct TokenGetter {
    pub email: String,
    pub private_key: rustls::PrivateKey,
    pub cache: Mutex<Option<(i64, Result)>>,
}

impl TokenGetter {
    pub fn from_credentials_json(path: &str) -> Self {
        let mut buffer = String::new();
        std::fs::File::open(path)
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();
        let json: serde_json::Value =
            serde_json::from_reader(std::io::BufReader::new(std::fs::File::open(path).unwrap()))
                .unwrap();
        let email = json["client_email"].as_str().unwrap();
        let private_key_pem = json["private_key"].as_str().unwrap();

        let private_key = rustls_pemfile::pkcs8_private_keys(&mut std::io::BufReader::new(
            private_key_pem.as_bytes(),
        ))
        .unwrap()
        .pop()
        .unwrap();

        TokenGetter {
            email: email.to_owned(),
            private_key: rustls::PrivateKey(private_key),
            cache: Mutex::new(None),
        }
    }

    pub async fn get(&self) -> String {
        let current_time = chrono::Utc::now().timestamp();
        if let Some((expires_in, result)) = &*self.cache.lock().unwrap() {
            if current_time + 60 < *expires_in {
                return result.access_token.clone();
            }
        }
        let result = self.fetch().await;
        *self.cache.lock().unwrap() = Some(result.clone());
        result.1.access_token
    }

    pub async fn fetch(&self) -> (i64, Result) {
        let algo = base64_encode("{\"alg\":\"RS256\",\"typ\":\"JWT\"}".as_bytes());
        let current_time = chrono::Utc::now().timestamp();
        let token = base64_encode( &format!("{{\"iss\":\"{}\",\"scope\":\"https://www.googleapis.com/auth/datastore\",\"aud\":\"https://oauth2.googleapis.com/token\",\"exp\":{},\"iat\":{}}}", self.email, current_time + TTL, current_time).as_bytes());
        let signing_key = rustls::sign::any_supported_type(&self.private_key).unwrap();
        let signer = signing_key
            .choose_scheme(&[rustls::SignatureScheme::RSA_PKCS1_SHA256])
            .unwrap();
        let signature_string = format!("{}.{}", algo, token);
        let signature = signer.sign(signature_string.as_bytes()).unwrap();
        let signature = base64_encode(&signature);
        let jwt = format!("{}.{}.{}", algo, token, signature);

        let res = reqwest::Client::new()
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", &jwt),
            ])
            .send()
            .await
            .unwrap();
        let res = res.json::<Result>().await.unwrap();
        (current_time + res.expires_in, res)
    }
}

pub async fn get_access_token(credential: &str) -> Result {
    let mut buffer = String::new();
    std::fs::File::open(credential)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();
    let json: serde_json::Value = serde_json::from_reader(std::io::BufReader::new(
        std::fs::File::open(credential).unwrap(),
    ))
    .unwrap();
    let email = json["client_email"].as_str().unwrap();
    let private_key_pem = json["private_key"].as_str().unwrap();

    let private_key = rustls_pemfile::pkcs8_private_keys(&mut std::io::BufReader::new(
        private_key_pem.as_bytes(),
    ))
    .unwrap()
    .pop()
    .unwrap();

    let algo = base64_encode("{\"alg\":\"RS256\",\"typ\":\"JWT\"}".as_bytes());
    let current_time = chrono::Utc::now().timestamp();
    let token = base64_encode( &format!("{{\"iss\":\"{}\",\"scope\":\"https://www.googleapis.com/auth/datastore\",\"aud\":\"https://oauth2.googleapis.com/token\",\"exp\":{},\"iat\":{}}}", email, current_time + 3600, current_time).as_bytes());
    let signing_key = rustls::sign::any_supported_type(&rustls::PrivateKey(private_key)).unwrap();
    let signer = signing_key
        .choose_scheme(&[rustls::SignatureScheme::RSA_PKCS1_SHA256])
        .unwrap();
    let signature_string = format!("{}.{}", algo, token);
    let signature = signer.sign(signature_string.as_bytes()).unwrap();
    let signature = base64_encode(&signature);
    let jwt = format!("{}.{}.{}", algo, token, signature);

    let res = reqwest::Client::new()
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &jwt),
        ])
        .send()
        .await
        .unwrap();
    res.json::<Result>().await.unwrap()
}

pub fn base64_encode(data: &[u8]) -> String {
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, data)
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Result {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
}

pub async fn get_meta_data() -> Result {
    let res = reqwest::Client::new()
        .get("http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token")
        .header("Metadata-Flavor", "Google")
        .send().await
        .unwrap();
    let res = res.text().await.unwrap();
    serde_json::from_str::<Result>(&res).unwrap()
}

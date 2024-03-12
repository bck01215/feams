use oauth2::{basic::BasicTokenType, EmptyExtraTokenFields, StandardTokenResponse, TokenResponse};
use reqwest::{header, Client, ClientBuilder};
use serde::Deserialize;
pub struct GraphClient {
    client: Client,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GraphUser {
    pub user_principal_name: String,
    pub id: String,
    pub display_name: String,
    pub surname: String,
    pub given_name: String,
    pub preferred_language: String,
    pub mail: String,
}

impl GraphClient {
    pub fn new(token: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>) -> Self {
        let token = token.access_token().secret().to_string();
        println!("Token: {}", token);
        let build_client = ClientBuilder::new();
        let client = build_client
            .default_headers({
                let mut headers = header::HeaderMap::new();
                headers.insert(
                    header::AUTHORIZATION,
                    header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
                );
                headers
            })
            .build()
            .unwrap();

        Self { client }
    }

    pub async fn get_user(&self) -> Result<GraphUser, Box<dyn std::error::Error>> {
        let resp = self
            .client
            .get("https://graph.microsoft.com/v1.0/me")
            .send()
            .await?;
        if resp.status() != 200 {
            println!("Error: {}", resp.text().await?);
            return Err("Error".into());
        }
        let resp: GraphUser = resp.json().await?;

        Ok(resp)
    }
}

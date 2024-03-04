use oauth2::{basic::BasicTokenType, EmptyExtraTokenFields, StandardTokenResponse, TokenResponse};
use reqwest::{header, Client, ClientBuilder};
use serde::Deserialize;
pub struct GraphClient {
    client: Client,
    token: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
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
        let build_client = ClientBuilder::new();
        let client = build_client
            .default_headers({
                let mut headers = header::HeaderMap::new();
                headers.insert(
                    header::AUTHORIZATION,
                    header::HeaderValue::from_str(&format!(
                        "Bearer {}",
                        token.access_token().secret().to_string()
                    ))
                    .unwrap(),
                );
                headers
            })
            .build()
            .unwrap();

        Self { client, token }
    }

    pub async fn get_user(&self) -> Result<GraphUser, Box<dyn std::error::Error>> {
        let resp = self
            .client
            .get("https://graph.microsoft.com/v1.0/me")
            .send()
            .await?
            .json::<GraphUser>()
            .await?;
        Ok(resp)
    }
}

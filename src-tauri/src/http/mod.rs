use reqwest::{Client, header};

use crate::types::{AppProfile, Me};

#[derive(Debug)]
pub struct HttpClient {
	pub client: Client,
	pub id: String,
	pub username: String,
	pub email: String,
	token: String,
}

impl From<HttpClient> for AppProfile {
    fn from(client: HttpClient) -> Self {
        AppProfile {
            token: client.token,
            username: client.username,
            email: client.email,
        }
    }
}

impl HttpClient {
	pub async fn new(token: &str) -> Result<Self, reqwest::Error> {
	    let mut headers = header::HeaderMap::new();
	    headers.insert(
	        header::AUTHORIZATION,
	        header::HeaderValue::from_str(token).unwrap(),
	    );
	    let client = Client::builder()
	        .default_headers(headers)
	        .build()?;

	    let response = client
	        .get("https://discord.com/api/v9/users/@me")
	        .send()
	        .await?;

	    let response = response.error_for_status()?;

	    let body = response.json::<Me>().await?;

	    Ok(Self {
	        client,
	        id: body.id,
	        username: body.username,
	        email: body.email,
	        token: token.to_string(),
	    })
	}
}
use crate::error::LocoError;
use crate::models::ApiError;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

pub mod assets;
pub mod auth;
pub mod export;
pub mod import;
pub mod locales;
pub mod tags;
pub mod translations;

pub struct LocoClient {
    http: reqwest::Client,
    base_url: String,
}

impl LocoClient {
    pub fn new(api_key: &str, base_url: &str) -> Result<Self, LocoError> {
        let mut headers = HeaderMap::new();
        let auth_value = format!("Loco {api_key}");
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_value)
                .map_err(|_| LocoError::Config("Invalid API key format".into()))?,
        );

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .user_agent(format!("loco-cli/{}", env!("CARGO_PKG_VERSION")))
            .build()?;

        Ok(Self {
            http,
            base_url: base_url.trim_end_matches('/').to_string(),
        })
    }

    pub fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub(crate) fn client(&self) -> &reqwest::Client {
        &self.http
    }

    /// Check response status and convert API errors to LocoError
    pub async fn check_response(
        &self,
        response: reqwest::Response,
    ) -> Result<reqwest::Response, LocoError> {
        let status = response.status();
        if status.is_success() {
            return Ok(response);
        }

        match status.as_u16() {
            401 => Err(LocoError::Unauthorized),
            404 => {
                let body = response.text().await.unwrap_or_default();
                Err(LocoError::NotFound(body))
            }
            429 => Err(LocoError::RateLimited),
            _ => {
                let body = response.text().await.unwrap_or_default();
                let message = serde_json::from_str::<ApiError>(&body)
                    .map(|e| e.error)
                    .unwrap_or(body);
                Err(LocoError::Api {
                    status: status.as_u16(),
                    message,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn make_client(server: &MockServer) -> LocoClient {
        LocoClient::new("test-key", &server.uri()).expect("build client")
    }

    #[tokio::test]
    async fn check_response_200_passes_through() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/ok"))
            .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
            .mount(&server)
            .await;

        let client = make_client(&server).await;
        let resp = client.client().get(client.url("/ok")).send().await.unwrap();
        let resp = client.check_response(resp).await.unwrap();
        assert_eq!(resp.status().as_u16(), 200);
    }

    #[tokio::test]
    async fn check_response_401_unauthorized() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/fail"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = make_client(&server).await;
        let resp = client
            .client()
            .get(client.url("/fail"))
            .send()
            .await
            .unwrap();
        let err = client.check_response(resp).await.unwrap_err();
        assert!(matches!(err, LocoError::Unauthorized));
    }

    #[tokio::test]
    async fn check_response_404_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/missing"))
            .respond_with(ResponseTemplate::new(404).set_body_string("gone"))
            .mount(&server)
            .await;

        let client = make_client(&server).await;
        let resp = client
            .client()
            .get(client.url("/missing"))
            .send()
            .await
            .unwrap();
        let err = client.check_response(resp).await.unwrap_err();
        assert!(matches!(err, LocoError::NotFound(_)));
    }

    #[tokio::test]
    async fn check_response_429_rate_limited() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/slow"))
            .respond_with(ResponseTemplate::new(429))
            .mount(&server)
            .await;

        let client = make_client(&server).await;
        let resp = client
            .client()
            .get(client.url("/slow"))
            .send()
            .await
            .unwrap();
        let err = client.check_response(resp).await.unwrap_err();
        assert!(matches!(err, LocoError::RateLimited));
    }
}

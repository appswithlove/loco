use super::LocoClient;
use crate::error::LocoError;
use crate::models::AuthResponse;

impl LocoClient {
    pub async fn auth_verify(&self) -> Result<AuthResponse, LocoError> {
        let resp = self.client().get(self.url("/auth/verify")).send().await?;
        let resp = self.check_response(resp).await?;
        Ok(resp.json().await?)
    }
}

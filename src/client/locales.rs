use super::LocoClient;
use crate::error::LocoError;
use crate::models::Locale;

impl LocoClient {
    pub async fn list_locales(&self) -> Result<Vec<Locale>, LocoError> {
        let resp = self.client().get(self.url("/locales")).send().await?;
        let resp = self.check_response(resp).await?;
        Ok(resp.json().await?)
    }

    pub async fn get_locale(&self, code: &str) -> Result<Locale, LocoError> {
        let resp = self
            .client()
            .get(self.url(&format!("/locales/{code}")))
            .send()
            .await?;
        let resp = self.check_response(resp).await?;
        Ok(resp.json().await?)
    }

    pub async fn create_locale(&self, code: &str) -> Result<Locale, LocoError> {
        let resp = self
            .client()
            .post(self.url("/locales"))
            .form(&[("code", code)])
            .send()
            .await?;
        let resp = self.check_response(resp).await?;
        Ok(resp.json().await?)
    }

    pub async fn delete_locale(&self, code: &str) -> Result<(), LocoError> {
        let resp = self
            .client()
            .delete(self.url(&format!("/locales/{code}")))
            .send()
            .await?;
        self.check_response(resp).await?;
        Ok(())
    }
}

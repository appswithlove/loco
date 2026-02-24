use super::LocoClient;
use crate::error::LocoError;
use crate::models::Translation;

impl LocoClient {
    pub async fn get_translations(&self, id: &str) -> Result<Vec<Translation>, LocoError> {
        let resp = self
            .client()
            .get(self.url(&format!("/translations/{id}.json")))
            .send()
            .await?;
        let resp = self.check_response(resp).await?;
        Ok(resp.json().await?)
    }

    pub async fn get_translation(&self, id: &str, locale: &str) -> Result<Translation, LocoError> {
        let resp = self
            .client()
            .get(self.url(&format!("/translations/{id}/{locale}")))
            .send()
            .await?;
        let resp = self.check_response(resp).await?;
        Ok(resp.json().await?)
    }

    pub async fn set_translation(
        &self,
        id: &str,
        locale: &str,
        text: &str,
    ) -> Result<Translation, LocoError> {
        let resp = self
            .client()
            .post(self.url(&format!("/translations/{id}/{locale}")))
            .body(text.to_string())
            .send()
            .await?;
        let resp = self.check_response(resp).await?;
        Ok(resp.json().await?)
    }

    pub async fn delete_translation(&self, id: &str, locale: &str) -> Result<(), LocoError> {
        let resp = self
            .client()
            .delete(self.url(&format!("/translations/{id}/{locale}")))
            .send()
            .await?;
        self.check_response(resp).await?;
        Ok(())
    }

    pub async fn flag_translation(
        &self,
        id: &str,
        locale: &str,
        flag: Option<&str>,
    ) -> Result<(), LocoError> {
        let mut req = self
            .client()
            .post(self.url(&format!("/translations/{id}/{locale}/flag")));
        if let Some(f) = flag {
            req = req.form(&[("flag", f)]);
        }
        let resp = req.send().await?;
        self.check_response(resp).await?;
        Ok(())
    }

    pub async fn unflag_translation(&self, id: &str, locale: &str) -> Result<(), LocoError> {
        let resp = self
            .client()
            .delete(self.url(&format!("/translations/{id}/{locale}/flag")))
            .send()
            .await?;
        self.check_response(resp).await?;
        Ok(())
    }
}

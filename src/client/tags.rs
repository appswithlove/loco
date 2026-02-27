use super::LocoClient;
use crate::error::LocoError;

impl LocoClient {
    pub async fn list_tags(&self) -> Result<Vec<String>, LocoError> {
        let resp = self.client().get(self.url("/tags")).send().await?;
        let resp = self.check_response(resp).await?;
        Ok(resp.json().await?)
    }

    pub async fn create_tag(&self, name: &str) -> Result<(), LocoError> {
        let resp = self
            .client()
            .post(self.url("/tags"))
            .form(&[("name", name)])
            .send()
            .await?;
        self.check_response(resp).await?;
        Ok(())
    }

    pub async fn rename_tag(&self, old: &str, new: &str) -> Result<(), LocoError> {
        let resp = self
            .client()
            .patch(self.url(&format!("/tags/{old}.json")))
            .json(&serde_json::json!({"name": new}))
            .send()
            .await?;
        self.check_response(resp).await?;
        Ok(())
    }

    pub async fn delete_tag(&self, name: &str) -> Result<(), LocoError> {
        let resp = self
            .client()
            .delete(self.url(&format!("/tags/{name}.json")))
            .send()
            .await?;
        self.check_response(resp).await?;
        Ok(())
    }
}

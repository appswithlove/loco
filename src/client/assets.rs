use super::LocoClient;
use crate::error::LocoError;
use crate::models::{Asset, CreateAssetRequest};

impl LocoClient {
    pub async fn list_assets(&self, filter: Option<&str>) -> Result<Vec<Asset>, LocoError> {
        let mut req = self.client().get(self.url("/assets"));
        if let Some(f) = filter {
            req = req.query(&[("filter", f)]);
        }
        let resp = req.send().await?;
        let resp = self.check_response(resp).await?;
        Ok(resp.json().await?)
    }

    pub async fn get_asset(&self, id: &str) -> Result<Asset, LocoError> {
        let resp = self
            .client()
            .get(self.url(&format!("/assets/{id}.json")))
            .send()
            .await?;
        let resp = self.check_response(resp).await?;
        Ok(resp.json().await?)
    }

    pub async fn create_asset(&self, req: &CreateAssetRequest) -> Result<Asset, LocoError> {
        let resp = self
            .client()
            .post(self.url("/assets"))
            .form(req)
            .send()
            .await?;
        let resp = self.check_response(resp).await?;
        Ok(resp.json().await?)
    }

    pub async fn delete_asset(&self, id: &str) -> Result<(), LocoError> {
        let resp = self
            .client()
            .delete(self.url(&format!("/assets/{id}.json")))
            .send()
            .await?;
        self.check_response(resp).await?;
        Ok(())
    }

    pub async fn tag_asset(&self, id: &str, tag: &str) -> Result<(), LocoError> {
        let resp = self
            .client()
            .post(self.url(&format!("/assets/{id}/tags")))
            .form(&[("name", tag)])
            .send()
            .await?;
        self.check_response(resp).await?;
        Ok(())
    }

    pub async fn untag_asset(&self, id: &str, tag: &str) -> Result<(), LocoError> {
        let resp = self
            .client()
            .delete(self.url(&format!("/assets/{id}/tags/{tag}.json")))
            .send()
            .await?;
        self.check_response(resp).await?;
        Ok(())
    }
}

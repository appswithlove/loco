use super::LocoClient;
use crate::error::LocoError;
use crate::models::{ImportProgress, ImportResult};

pub struct ImportParams<'a> {
    pub ext: &'a str,
    pub body: Vec<u8>,
    pub locale: Option<&'a str>,
    pub index: Option<&'a str>,
    pub format: Option<&'a str>,
    pub tag_new: Option<&'a str>,
    pub is_async: bool,
}

impl LocoClient {
    /// Import a translation file
    pub async fn import_file(&self, params: ImportParams<'_>) -> Result<ImportResult, LocoError> {
        let url = self.url(&format!("/import/{}", params.ext));
        let mut req = self
            .client()
            .post(&url)
            .header("Content-Type", "application/octet-stream")
            .body(params.body);

        if let Some(l) = params.locale {
            req = req.query(&[("locale", l)]);
        }
        if let Some(i) = params.index {
            req = req.query(&[("index", i)]);
        }
        if let Some(f) = params.format {
            req = req.query(&[("format", f)]);
        }
        if let Some(t) = params.tag_new {
            req = req.query(&[("tag-new", t)]);
        }
        if params.is_async {
            req = req.query(&[("async", "1")]);
        }

        let resp = req.send().await?;
        let resp = self.check_response(resp).await?;
        Ok(resp.json().await?)
    }

    /// Check async import progress
    pub async fn import_progress(&self, id: &str) -> Result<ImportProgress, LocoError> {
        let resp = self
            .client()
            .get(self.url(&format!("/import/progress/{id}")))
            .send()
            .await?;
        let resp = self.check_response(resp).await?;
        Ok(resp.json().await?)
    }
}

#![allow(dead_code)]
use super::LocoClient;
use crate::error::LocoError;

#[derive(Debug, Default)]
pub struct ExportParams {
    pub format: Option<String>,
    pub filter: Option<String>,
    pub status: Option<String>,
    pub index: Option<String>,
}

impl LocoClient {
    /// Export a single locale
    pub async fn export_locale(
        &self,
        locale: &str,
        ext: &str,
        params: &ExportParams,
    ) -> Result<bytes::Bytes, LocoError> {
        let url = self.url(&format!("/export/locale/{locale}.{ext}"));
        let mut req = self.client().get(&url);

        if let Some(ref fmt) = params.format {
            req = req.query(&[("format", fmt.as_str())]);
        }
        if let Some(ref filter) = params.filter {
            req = req.query(&[("filter", filter.as_str())]);
        }
        if let Some(ref status) = params.status {
            req = req.query(&[("status", status.as_str())]);
        }
        if let Some(ref index) = params.index {
            req = req.query(&[("index", index.as_str())]);
        }

        let resp = req.send().await?;
        let resp = self.check_response(resp).await?;
        Ok(resp.bytes().await?)
    }

    /// Export all locales as archive (zip)
    pub async fn export_archive(
        &self,
        ext: &str,
        params: &ExportParams,
    ) -> Result<bytes::Bytes, LocoError> {
        let url = self.url(&format!("/export/archive/{ext}.zip"));
        let mut req = self.client().get(&url);

        if let Some(ref fmt) = params.format {
            req = req.query(&[("format", fmt.as_str())]);
        }
        if let Some(ref filter) = params.filter {
            req = req.query(&[("filter", filter.as_str())]);
        }
        if let Some(ref status) = params.status {
            req = req.query(&[("status", status.as_str())]);
        }

        let resp = req.send().await?;
        let resp = self.check_response(resp).await?;
        Ok(resp.bytes().await?)
    }

    /// Export all locales in a multi-locale format
    pub async fn export_all(
        &self,
        ext: &str,
        params: &ExportParams,
    ) -> Result<bytes::Bytes, LocoError> {
        let url = self.url(&format!("/export/all.{ext}"));
        let mut req = self.client().get(&url);

        if let Some(ref fmt) = params.format {
            req = req.query(&[("format", fmt.as_str())]);
        }
        if let Some(ref filter) = params.filter {
            req = req.query(&[("filter", filter.as_str())]);
        }
        if let Some(ref status) = params.status {
            req = req.query(&[("status", status.as_str())]);
        }

        let resp = req.send().await?;
        let resp = self.check_response(resp).await?;
        Ok(resp.bytes().await?)
    }
}

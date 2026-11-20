use crate::matchers::{Backend, SeriesMatch};

use anyhow::Result;
use reqwest::Client;

#[derive(Clone, Copy)]
pub struct TheTvdb;

impl Backend for TheTvdb {
    fn name(&self) -> &'static str {
        "TheTVDB"
    }

    async fn search(&self, _client: &Client, _query: &str) -> Result<Vec<SeriesMatch>> {
        Ok(Vec::new())
    }
}

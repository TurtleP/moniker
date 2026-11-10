use crate::matchers::{Backend, SeriesMatch};

use anyhow::Result;
use reqwest::Client;

#[derive(Debug, Default, Clone, Copy)]
pub struct TheTvdb;

impl Backend for TheTvdb {
    fn name(&self) -> &'static str {
        "TheTVDB"
    }

    async fn search(&self, client: &Client, query: &str) -> Result<Vec<SeriesMatch>> {
        Ok(Vec::new())
    }
}

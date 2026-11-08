use crate::matchers::{Backend, SearchResult};

use anyhow::Result;
use reqwest::Client;

#[derive(Debug, Default, Clone, Copy)]
pub struct AniDb;

impl Backend for AniDb {
    fn name(&self) -> &'static str {
        "AniDB"
    }

    async fn search(&self, client: &Client, query: &str) -> Result<Vec<SearchResult>> {
        Ok(Vec::new())
    }
}

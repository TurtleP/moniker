use anyhow::Result;
use reqwest::Client;

use tracing::info;

use crate::matchers::{Backend, Episode, Series, SeriesMatch};

#[derive(Debug, Default, Clone, Copy)]
pub struct TVMaze;

const TVMAZE_API_URL: &str = "https://api.tvmaze.com";

#[derive(serde::Deserialize, Debug)]
struct SearchHit {
    show: Series,
}

impl TVMaze {
    async fn get_series_id(&self, client: &Client, query: &str) -> Result<Vec<Series>> {
        info!("Searching for show: {query}");
        let mut url = reqwest::Url::parse(&format!("{TVMAZE_API_URL}/search/shows"))?;
        url.set_query(Some(&format!("q={query}")));

        let hits = client.get(url);
        let results: Vec<SearchHit> = hits.send().await?.error_for_status()?.json().await?;
        Ok(results.into_iter().map(|h| h.show).collect())
    }

    async fn get_show_episodes(&self, client: &Client, show_id: u64) -> Result<Vec<Episode>> {
        let url = reqwest::Url::parse(&format!("{TVMAZE_API_URL}/shows/{show_id}/episodes"))?;
        Ok(client.get(url).send().await?.json().await?)
    }
}

impl Backend for TVMaze {
    fn name(&self) -> &'static str {
        "TVmaze"
    }

    async fn search(&self, client: &Client, query: &str) -> Result<Vec<SeriesMatch>> {
        let shows = self.get_series_id(client, query).await?;
        let mut results = Vec::new();
        info!("Found {} results", shows.len());

        for series in shows {
            let series_id = series.id;
            let episodes = self.get_show_episodes(client, series_id).await?;
            results.push(SeriesMatch { series, episodes });
        }
        Ok(results)
    }
}

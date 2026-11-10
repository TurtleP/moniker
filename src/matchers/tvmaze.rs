use anyhow::Result;
use reqwest::Client;

use tracing::info;

use crate::matchers::{Backend, Episode, Series, SeriesMatch};

#[derive(Debug, Default, Clone, Copy)]
pub struct TVMaze;

const TVMAZE_API_URL: &str = "https://api.tvmaze.com";

impl TVMaze {
    async fn get_shows_by_id(&self, client: &Client, query: &str) -> Result<Vec<Series>> {
        info!("Searching for show: {query}");
        let mut url = reqwest::Url::parse(&format!("{TVMAZE_API_URL}/search/shows"))?;
        url.set_query(Some(&format!("q={query}")));

        let hits = client.get(url);
        let json: Vec<serde_json::Value> = hits.send().await?.json().await?;

        let mut shows: Vec<Series> = Vec::new();
        for show_info in json {
            let show = serde_json::from_value(show_info["show"].clone())?;
            shows.push(show);
        }
        Ok(shows)
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
        let shows = self.get_shows_by_id(client, query).await?;
        let mut results = Vec::new();
        info!("Found {} results", shows.len());

        for series in shows {
            let series_id = series.id;
            let episodes = self.get_show_episodes(client, series_id).await?;
            results.push(SeriesMatch {
                series,
                episodes,
            });
        }
        Ok(results)
    }
}

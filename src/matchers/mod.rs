use std::fmt::Display;

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Deserializer};

mod anidb;
mod thetvdb;
mod tvmaze;

pub fn year_from_date<'de, D>(deserializer: D) -> Result<Option<u16>, D::Error>
where
    D: Deserializer<'de>,
{
    let date: Option<String> = Option::deserialize(deserializer)?;
    Ok(date.and_then(|d| d.get(..4)?.parse().ok()))
}

#[derive(Clone, Default, Deserialize)]
pub struct Series {
    pub name: String,
    pub id: u64,
    pub year: Option<u16>,
}

impl Display for Series {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.year {
            Some(year) => write!(f, "{} ({year})", self.name),
            None => write!(f, "{}", self.name),
        }
    }
}

#[derive(Default, Clone, Deserialize)]
pub struct Episode {
    pub name: String,
    pub number: u16,
    pub season: u16,
}

#[derive(Default, Clone)]
pub struct SeriesMatch {
    pub series: Series,
    pub episodes: Vec<Episode>,
}

impl Display for SeriesMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.series.name)
    }
}

pub trait Backend {
    fn name(&self) -> &'static str;
    async fn search(&self, client: &Client, query: &str) -> Result<Vec<SeriesMatch>>;
}

#[derive(Clone, Copy)]
pub enum Provider {
    TVMaze(tvmaze::TVMaze),
    TheTvdb(thetvdb::TheTvdb),
    AniDb(anidb::AniDb),
}

impl Default for Provider {
    fn default() -> Self {
        Provider::TVMaze(tvmaze::TVMaze)
    }
}

impl Provider {
    pub const ALL: [Provider; 3] = [
        Provider::TVMaze(tvmaze::TVMaze),
        Provider::TheTvdb(thetvdb::TheTvdb),
        Provider::AniDb(anidb::AniDb),
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Self::TVMaze(s) => s.name(),
            Self::TheTvdb(s) => s.name(),
            Self::AniDb(s) => s.name(),
        }
    }

    pub async fn search(&self, client: &Client, query: &str) -> Result<Vec<SeriesMatch>> {
        match self {
            Self::TVMaze(s) => s.search(client, query).await,
            Self::TheTvdb(s) => s.search(client, query).await,
            Self::AniDb(s) => s.search(client, query).await,
        }
    }
}

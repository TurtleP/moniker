use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

mod anidb;
mod thetvdb;
mod tvmaze;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ShowResult {
    pub name: String,
    pub id: u64,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct EpisodeResult {
    pub name: String,
    pub number: u16,
    pub season: u16,
}

#[derive(Debug, Clone, Default)]
pub struct SearchResult {
    pub show: ShowResult,
    pub episodes: Vec<EpisodeResult>,
}

pub trait Backend {
    fn name(&self) -> &'static str;
    async fn search(&self, client: &Client, query: &str) -> Result<Vec<SearchResult>>;
}

#[derive(Clone, Copy)]
pub enum Provider {
    TVMaze(tvmaze::TVMaze),
    TheTvdb(thetvdb::TheTvdb),
    AniDb(anidb::AniDb),
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

    pub async fn search(&self, client: &Client, query: &str) -> Result<Vec<SearchResult>> {
        match self {
            Self::TVMaze(s) => s.search(client, query).await,
            Self::TheTvdb(s) => s.search(client, query).await,
            Self::AniDb(s) => s.search(client, query).await,
        }
    }
}

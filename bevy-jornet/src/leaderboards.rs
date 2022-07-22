use std::sync::{Arc, RwLock};

use bevy::tasks::IoTaskPool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::http;

pub struct Leaderboards {
    key: Option<Uuid>,
    leaderboard: Arc<RwLock<Vec<Score>>>,
    host: String,
}

impl Default for Leaderboards {
    fn default() -> Self {
        Self {
            key: Default::default(),
            leaderboard: Default::default(),
            host: "https://jornet.vleue.com".to_string(),
        }
    }
}

impl Leaderboards {
    pub fn set_dashboard(&mut self, key: Uuid) {
        self.key = Some(key);
    }

    pub fn send_score(&mut self, score: f32) {
        let thread_pool = IoTaskPool::get();
        let key = self.key.unwrap();
        let host = self.host.clone();

        let score_to_send = Some(Score {
            score,
            player: Uuid::new_v4(),
            timestamp: None,
            meta: None,
        });
        thread_pool
            .spawn(async move {
                http::post(&format!("{}/api/scores/{}", host, key), &score_to_send).await;
            })
            .detach();
    }

    pub fn refresh_leaderboard(&self) {
        let thread_pool = IoTaskPool::get();
        let key = self.key.unwrap();
        let host = self.host.clone();

        let leaderboard_to_update = self.leaderboard.clone();

        thread_pool
            .spawn(async move {
                let scores = http::get(&format!("{}/api/scores/{}", host, key)).await;
                *leaderboard_to_update.write().unwrap() = scores;
            })
            .detach();
    }

    pub fn get_leaderboard(&self) -> Vec<Score> {
        self.leaderboard.read().unwrap().clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Score {
    pub score: f32,
    pub player: Uuid,
    pub meta: Option<String>,
    timestamp: Option<String>,
}

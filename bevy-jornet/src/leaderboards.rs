use std::sync::{Arc, RwLock};

use bevy::{prelude::ResMut, tasks::IoTaskPool};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::http;

pub struct Leaderboard {
    key: Uuid,
    leaderboard: Vec<Score>,
    updating: Arc<RwLock<Vec<Score>>>,
    host: String,
}

impl Leaderboard {
    pub(crate) fn with_leaderboard(key: Uuid) -> Self {
        Self {
            key,
            leaderboard: Default::default(),
            updating: Default::default(),
            host: "https://jornet.vleue.com".to_string(),
        }
    }

    pub fn send_score(&self, score: f32) {
        let thread_pool = IoTaskPool::get();
        let key = self.key;
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
        let key = self.key;
        let host = self.host.clone();

        let leaderboard_to_update = self.updating.clone();

        thread_pool
            .spawn(async move {
                let scores = http::get(&format!("{}/api/scores/{}", host, key)).await;
                *leaderboard_to_update.write().unwrap() = scores;
            })
            .detach();
    }

    pub fn get_leaderboard(&self) -> Vec<Score> {
        self.leaderboard.clone()
    }
}

pub fn done_refreshing_leaderboard(mut leaderboard: ResMut<Leaderboard>) {
    if !leaderboard.updating.read().unwrap().is_empty() {
        let updated = leaderboard
            .updating
            .write()
            .unwrap()
            .drain(..)
            .collect::<Vec<_>>();
        leaderboard.leaderboard = updated;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Score {
    pub score: f32,
    pub player: Uuid,
    pub meta: Option<String>,
    timestamp: Option<String>,
}

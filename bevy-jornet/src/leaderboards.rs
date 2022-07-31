use std::sync::{Arc, RwLock};
#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};

use bevy::{
    prelude::{warn, ResMut},
    tasks::IoTaskPool,
};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use uuid::Uuid;

use crate::http;

pub struct Leaderboard {
    id: Uuid,
    key: Uuid,
    leaderboard: Vec<Score>,
    updating: Arc<RwLock<Vec<Score>>>,
    host: String,
    new_player: Arc<RwLock<Option<Player>>>,
    player: Option<Player>,
}

impl Leaderboard {
    pub(crate) fn with_leaderboard(id: Uuid, key: Uuid) -> Self {
        Self {
            id,
            key,
            leaderboard: Default::default(),
            updating: Default::default(),
            host: "https://jornet.vleue.com".to_string(),
            new_player: Default::default(),
            player: Default::default(),
        }
    }

    pub fn get_player_name(&self) -> Option<String> {
        self.player.as_ref().map(|p| p.name.clone())
    }

    pub fn create_player(&mut self, name: Option<&str>) {
        let thread_pool = IoTaskPool::get();
        let host = self.host.clone();

        let player = PlayerInput {
            name: name.map(|n| n.to_string()),
        };
        let complete_player = self.new_player.clone();

        thread_pool
            .spawn(async move {
                if let Some(player) = http::post(&format!("{}/api/v1/players", host), player).await
                {
                    *complete_player.write().unwrap() = Some(player);
                } else {
                    warn!("error creating a player");
                }
            })
            .detach();
    }

    pub fn send_score(&self, score: f32) -> Option<()> {
        let thread_pool = IoTaskPool::get();
        let leaderboard_id = self.id;
        let host = self.host.clone();

        if let Some(player) = self.player.as_ref() {
            let score_to_send = ScoreInput::new(self.key, score, player, None);
            thread_pool
                .spawn(async move {
                    if http::post::<_, ()>(
                        &format!("{}/api/v1/scores/{}", host, leaderboard_id),
                        score_to_send,
                    )
                    .await
                    .is_none()
                    {
                        warn!("error sending the score");
                    }
                })
                .detach();
            Some(())
        } else {
            None
        }
    }

    pub fn refresh_leaderboard(&self) {
        let thread_pool = IoTaskPool::get();
        let leaderboard_id = self.id;
        let host = self.host.clone();

        let leaderboard_to_update = self.updating.clone();

        thread_pool
            .spawn(async move {
                if let Some(scores) =
                    http::get(&format!("{}/api/v1/scores/{}", host, leaderboard_id)).await
                {
                    *leaderboard_to_update.write().unwrap() = scores;
                } else {
                    warn!("error getting the leaderboard");
                }
            })
            .detach();
    }

    pub fn get_leaderboard(&self) -> Vec<Score> {
        self.leaderboard.clone()
    }
}

pub fn done_refreshing_leaderboard(mut leaderboard: ResMut<Leaderboard>) {
    if !leaderboard
        .updating
        .try_read()
        .map(|v| v.is_empty())
        .unwrap_or(true)
    {
        let mut updated = leaderboard
            .updating
            .write()
            .unwrap()
            .drain(..)
            .collect::<Vec<_>>();
        updated.sort_unstable_by(|s1, s2| s2.score.partial_cmp(&s1.score).unwrap());
        updated.truncate(10);
        leaderboard.leaderboard = updated;
    }
    if leaderboard
        .new_player
        .try_read()
        .map(|v| v.is_some())
        .unwrap_or(false)
    {
        let new_player = leaderboard.new_player.write().unwrap().take();
        leaderboard.player = new_player;
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Score {
    pub score: f32,
    pub player: String,
    pub meta: Option<String>,
    pub timestamp: String,
}

#[derive(Serialize)]
struct ScoreInput {
    pub score: f32,
    pub player: Uuid,
    pub meta: Option<String>,
    pub timestamp: u64,
    pub k: String,
}

impl ScoreInput {
    fn new(leaderboard_key: Uuid, score: f32, player: &Player, meta: Option<String>) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        #[cfg(target_arch = "wasm32")]
        let timestamp = (js_sys::Date::now() / 1000.0) as u64;

        let mut mac = Hmac::<Sha256>::new_from_slice(player.key.as_bytes()).unwrap();
        mac.update(&timestamp.to_le_bytes());
        mac.update(leaderboard_key.as_bytes());
        mac.update(player.id.as_bytes());
        mac.update(&score.to_le_bytes());
        if let Some(meta) = meta.as_ref() {
            mac.update(meta.as_bytes());
        }

        let hmac = hex::encode(&mac.finalize().into_bytes()[..]);
        Self {
            score,
            player: player.id,
            meta,
            timestamp,
            k: hmac,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct Player {
    pub id: Uuid,
    pub key: Uuid,
    pub name: String,
}

#[derive(Serialize, Debug, Clone)]
struct PlayerInput {
    pub name: Option<String>,
}

use std::sync::{Arc, RwLock};
#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};

use bevy::{
    prelude::{warn, EventWriter, ResMut, Resource},
    tasks::IoTaskPool,
};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use uuid::Uuid;

use crate::http;

/// Bevy Event that is sent when calls to Jornet finish.
pub enum JornetEvent {
    /// A call to [`send_score`] succeeded.
    SendScoreSuccess,
    /// A Call to [`send_score`] failed.
    SendScoreFailure,
    /// A call to [`create_player`] succeeded.
    CreatePlayerSuccess,
    /// A call to [`create_player`] failed.
    CreatePlayerFailure,
    /// A call to [`refresh_leaderboard`] succeeded.
    RefreshLeaderboardSuccess,
    /// A call to [`refresh_leaderbord`] failed.
    RefreshLeaderboardFailure,
}
/// Leaderboard resource, used to interact with Jornet leaderboard.
#[derive(Resource)]
pub struct Leaderboard {
    id: Uuid,
    key: Uuid,
    leaderboard: Vec<Score>,
    updating: Arc<RwLock<Vec<Score>>>,
    events: Arc<RwLock<Vec<JornetEvent>>>,
    host: String,
    new_player: Arc<RwLock<Option<Player>>>,
    player: Option<Player>,
}

impl Leaderboard {
    pub(crate) fn with_host_and_leaderboard(host: Option<String>, id: Uuid, key: Uuid) -> Self {
        Self {
            id,
            key,
            leaderboard: Default::default(),
            updating: Default::default(),
            host: host.unwrap_or_else(|| "https://jornet.vleue.com".to_string()),
            events: Default::default(),
            new_player: Default::default(),
            player: Default::default(),
        }
    }

    /// Get the current player name.
    ///
    /// This can be used to get the random name generated if one was not specified when
    /// creating the player, or to save the `id`/`key` locally to be able to reconnect later
    /// as the same player.
    pub fn get_player(&self) -> Option<&Player> {
        self.player.as_ref()
    }

    /// Create a player. If you don't specify a name, one will be genertaed randomly.
    ///
    /// Either this or [`Self::as_player`] must be called before sending a score.
    pub fn create_player(&mut self, name: Option<&str>) {
        let thread_pool = IoTaskPool::get();
        let host = self.host.clone();
        let events = self.events.clone();

        let player_input = PlayerInput {
            name: name.map(|n| n.to_string()),
        };
        let complete_player = self.new_player.clone();

        thread_pool
            .spawn(async move {
                if let Some(player) =
                    http::post(&format!("{}/api/v1/players", host), player_input.clone()).await
                {
                    (*events)
                        .write()
                        .unwrap()
                        .push(JornetEvent::CreatePlayerSuccess);

                    *complete_player.write().unwrap() = Some(player);
                } else {
                    (*events)
                        .write()
                        .unwrap()
                        .push(JornetEvent::CreatePlayerFailure);

                    warn!("error creating a player");
                }
            })
            .detach();
    }

    /// Connect as a returning player.
    ///
    /// Either this or [`Self::create_player`] must be called before sending a score.
    pub fn as_player(&mut self, player: Player) {
        self.player = Some(player);
    }

    /// Send a score to the leaderboard.
    pub fn send_score(&self, score: f32) -> Option<()> {
        self.inner_send_score_with_meta(score, None)
    }

    /// Send a score with metadata to the leaderboard.
    ///
    /// Metadata can be information about the game, victory conditions, ...
    pub fn send_score_with_meta(&self, score: f32, meta: &str) -> Option<()> {
        self.inner_send_score_with_meta(score, Some(meta.to_string()))
    }

    fn inner_send_score_with_meta(&self, score: f32, meta: Option<String>) -> Option<()> {
        let thread_pool = IoTaskPool::get();
        let leaderboard_id = self.id;
        let host = self.host.clone();
        let events = self.events.clone();

        if let Some(player) = self.player.as_ref() {
            let score_to_send = ScoreInput::new(self.key, score, player, meta);
            thread_pool
                .spawn(async move {
                    if http::post::<_, ()>(
                        &format!("{}/api/v1/scores/{}", host, leaderboard_id),
                        score_to_send.clone(),
                    )
                    .await
                    .is_none()
                    {
                        (*events)
                            .write()
                            .unwrap()
                            .push(JornetEvent::SendScoreFailure);

                        warn!("error sending the score");
                    } else {
                        (*events)
                            .write()
                            .unwrap()
                            .push(JornetEvent::SendScoreSuccess);
                    }
                })
                .detach();
            Some(())
        } else {
            None
        }
    }

    /// Refresh the leaderboard, and get the most recent data from the server.
    ///
    /// This is done asynchronously, the resource [`Leaderboard`] will be marked as changed
    /// once the leaderboard data is available. You can then get those data with
    /// [`Self::get_leaderboard`].
    pub fn refresh_leaderboard(&self) {
        let thread_pool = IoTaskPool::get();
        let leaderboard_id = self.id;
        let host = self.host.clone();
        let events = self.events.clone();

        let leaderboard_to_update = self.updating.clone();

        thread_pool
            .spawn(async move {
                if let Some(scores) =
                    http::get(&format!("{}/api/v1/scores/{}", host, leaderboard_id)).await
                {
                    *leaderboard_to_update.write().unwrap() = scores;

                    (*events)
                        .write()
                        .unwrap()
                        .push(JornetEvent::RefreshLeaderboardSuccess);
                } else {
                    warn!("error getting the leaderboard");

                    (*events)
                        .write()
                        .unwrap()
                        .push(JornetEvent::RefreshLeaderboardFailure);
                }
            })
            .detach();
    }

    /// Get the leaderboard data. It must be refreshed first with [`Self::refresh_leaderboard`],
    /// which will mark the [`Leaderboard`] resource as changed once the data has been refreshed.
    ///
    /// Example system:
    ///
    /// ```rust
    /// # use bevy::prelude::*;
    /// # use bevy_jornet::Leaderboard;
    ///
    /// fn display_scores(
    ///     leaderboard: Res<Leaderboard>,
    /// ) {
    ///     if leaderboard.is_changed() {
    ///         for score in &leaderboard.get_leaderboard() {
    ///             // Display the score how you want
    ///         }
    ///     }
    /// }
    /// ```
    pub fn get_leaderboard(&self) -> Vec<Score> {
        self.leaderboard.clone()
    }
}

/// System to handle refreshing the [`Leaderboard`] resource when new data is available.
/// It is automatically added by the [`JornetPlugin`](crate::JornetPlugin) in stage
/// [`CoreStage::Update`](bevy::prelude::CoreStage).
pub fn done_refreshing_leaderboard(mut leaderboard: ResMut<Leaderboard>) {
    if !leaderboard
        .updating
        .try_read()
        .map(|v| v.is_empty())
        .unwrap_or(true)
    {
        let updated = leaderboard
            .updating
            .write()
            .unwrap()
            .drain(..)
            .collect::<Vec<_>>();
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

/// A score from a leaderboard
#[derive(Deserialize, Debug, Clone)]
pub struct Score {
    /// The score.
    pub score: f32,
    /// The player name.
    pub player: String,
    /// Optional metadata.
    pub meta: Option<String>,
    /// Timestamp of the score.
    pub timestamp: String,
}

#[derive(Serialize, Clone)]
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

/// A player, as returned from the server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    /// its ID
    pub id: Uuid,
    /// its key, this should be kept secret
    pub key: Uuid,
    /// its name, changing it here won't be reflected on the server
    pub name: String,
}

#[derive(Serialize, Debug, Clone)]
struct PlayerInput {
    name: Option<String>,
}

/// System to send bevy events for results from any tasks.
/// It is automatically added by the [`JornetPlugin`](crate::JornetPlugin) in stage
/// [`CoreStage::Update`](bevy::prelude::CoreStage).
pub fn send_events(leaderboard: ResMut<Leaderboard>, mut event_writer: EventWriter<JornetEvent>) {
    if !leaderboard
        .events
        .try_read()
        .map(|v| v.is_empty())
        .unwrap_or(true)
    {
        let mut events = leaderboard.events.write().unwrap();
        for event in events.drain(..) {
            event_writer.send(event);
        }
    }
}

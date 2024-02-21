#![warn(missing_docs)]

//! ![Jornet logo](https://jornet.vleue.com/logo-200.png)
//!
//! Bevy Plugin to integrate with [Jornet](https://jornet.vleue.com)
//! - save high scores
//! - get a leaderboard

use bevy::prelude::{App, IntoSystemConfigs, Plugin, Update};
use leaderboards::send_events;
pub use leaderboards::Leaderboard;
use uuid::Uuid;

mod http;
mod leaderboards;

pub use leaderboards::{done_refreshing_leaderboard, JornetEvent, Player, Score};

/// Bevy Plugin handling communications with the Jornet server.
pub struct JornetPlugin {
    leaderboard: Uuid,
    key: Uuid,
    host: Option<String>,
}

impl JornetPlugin {
    /// Setup the plugin with the `id` and `key`. They must be `UUID` from an existing leaderboard
    /// at <https://jornet.vleue.com>.
    ///
    /// Once the plugin is added, you can use the [`Leaderboard`] resource to interact with it,
    /// [create a player](Leaderboard::create_player), [send a score](Leaderboard::send_score) or
    /// [retrieve the leaderboard](Leaderboard::get_leaderboard).
    pub fn with_leaderboard(id: &str, key: &str) -> Self {
        Self {
            leaderboard: Uuid::parse_str(id).expect("invalid leaderboard ID"),
            key: Uuid::parse_str(key).expect("invalid leaderboard key"),
            host: None,
        }
    }

    /// Set the plugin to use another host than <https://jornet.vleue.com>.
    pub fn with_host(self, host: &str) -> Self {
        Self {
            host: Some(host.to_string()),
            ..self
        }
    }
}

impl Plugin for JornetPlugin {
    fn build(&self, app: &mut App) {
        let leaderboard =
            Leaderboard::with_host_and_leaderboard(self.host.clone(), self.leaderboard, self.key);
        app.add_event::<JornetEvent>()
            .insert_resource(leaderboard)
            .add_systems(Update, done_refreshing_leaderboard)
            .add_systems(Update, send_events.after(done_refreshing_leaderboard));
    }
}

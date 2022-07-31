use bevy::prelude::{App, Plugin};
pub use leaderboards::Leaderboard;
use uuid::Uuid;

mod http;
mod leaderboards;

pub use leaderboards::Score;

pub struct JornetPlugin {
    leaderboard: Uuid,
    key: Uuid,
}

impl JornetPlugin {
    pub fn with_leaderboard(id: &str, key: &str) -> Self {
        Self {
            leaderboard: Uuid::parse_str(id).expect("invalid leaderboard ID"),
            key: Uuid::parse_str(key).expect("invalid leaderboard key"),
        }
    }
}

impl Plugin for JornetPlugin {
    fn build(&self, app: &mut App) {
        let leaderboard = Leaderboard::with_leaderboard(self.leaderboard, self.key);
        app.insert_resource(leaderboard)
            .add_system(leaderboards::done_refreshing_leaderboard);
    }
}

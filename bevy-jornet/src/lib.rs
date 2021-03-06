use bevy::prelude::{App, Plugin};
pub use leaderboards::Leaderboard;
use uuid::Uuid;

mod http;
mod leaderboards;

pub struct JornetPlugin {
    leaderboard: Uuid,
}

impl JornetPlugin {
    pub fn with_leaderboard(key: Uuid) -> Self {
        Self { leaderboard: key }
    }
}

impl Plugin for JornetPlugin {
    fn build(&self, app: &mut App) {
        let leaderboard = Leaderboard::with_leaderboard(self.leaderboard);
        app.insert_resource(leaderboard)
            .add_system(leaderboards::done_refreshing_leaderboard);
    }
}

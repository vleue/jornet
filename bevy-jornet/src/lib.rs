use bevy::prelude::{App, Plugin};

pub mod leaderboards;
pub use leaderboards::Leaderboards;
mod http;

pub struct JornetPlugin;

impl Plugin for JornetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Leaderboards>();
    }
}

use bevy::prelude::*;
use bevy_jornet::{JornetPlugin, Leaderboards};
use uuid::Uuid;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(JornetPlugin)
        .add_startup_system(test)
        .add_system(display_scores)
        .run();
}

fn test(mut leaderboards: ResMut<Leaderboards>) {
    leaderboards.set_dashboard(Uuid::parse_str("e8b14303-c48e-463e-95da-2aa63e96b5f6").unwrap());
    leaderboards.send_score(120.7);
    leaderboards.refresh_leaderboard();
}

fn display_scores(leaderboards: Res<Leaderboards>, mut done: Local<bool>) {
    if !*done {
        if !leaderboards.get_leaderboard().is_empty() {
            info!("{:?}", leaderboards.get_leaderboard());
            *done = true;
        }
    }
}

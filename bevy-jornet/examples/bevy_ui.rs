use std::time::Duration;

use bevy::{
    prelude::*,
    winit::{UpdateMode, WinitSettings},
};
use bevy_jornet::{JornetPlugin, Leaderboards};
use uuid::Uuid;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            canvas: Some("#demo-leaderboard".to_string()),
            fit_canvas_to_parent: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings {
            focused_mode: UpdateMode::Reactive {
                max_wait: Duration::from_secs_f32(0.5),
            },
            ..WinitSettings::desktop_app()
        })
        .add_plugin(JornetPlugin)
        .add_startup_system(setup)
        .add_system(display_scores)
        .run();
}

fn setup(mut commands: Commands, mut leaderboards: ResMut<Leaderboards>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::ColumnReverse,
            ..default()
        },
        ..default()
    });
    leaderboards.set_dashboard(Uuid::parse_str("fb0bbe22-b047-494d-9519-1d36668fa5bc").unwrap());
    leaderboards.refresh_leaderboard();
    // leaderboards.send_score(59.8);
}

fn display_scores(
    leaderboards: Res<Leaderboards>,
    mut done: Local<bool>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    root_ui: Query<Entity, (With<Node>, Without<Parent>)>,
) {
    if !*done {
        if !leaderboards.get_leaderboard().is_empty() {
            let root_ui = root_ui.single();
            for score in leaderboards.get_leaderboard() {
                commands.entity(root_ui).with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        format!("{} ", score.score),
                        TextStyle {
                            font: asset_server.load("FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.2, 0.3, 0.2),
                        },
                    ));
                });
            }
            *done = true;
        }
    }
}

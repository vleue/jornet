use std::time::Duration;

const CLEAR: &str = "023047";
const BACKGROUND: &str = "fb8500";
const BUTTON: &str = "ffb703";
const TEXT: &str = "8ecae6";
const SQUARE: &str = "219ebc";

use bevy::{
    prelude::*,
    winit::{UpdateMode, WinitSettings},
};
use bevy_jornet::{JornetPlugin, Leaderboard};
use uuid::Uuid;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Whac-A-Square".to_string(),
            canvas: Some("#demo-leaderboard".to_string()),
            fit_canvas_to_parent: true,
            ..default()
        })
        .insert_resource(ClearColor(Color::hex(CLEAR).unwrap()))
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings {
            focused_mode: UpdateMode::Reactive {
                max_wait: Duration::from_secs_f32(0.5),
            },
            ..WinitSettings::desktop_app()
        })
        .add_plugin(JornetPlugin::with_leaderboard(
            Uuid::parse_str("fb0bbe22-b047-494d-9519-1d36668fa5bc").unwrap(),
        ))
        .add_startup_system(setup)
        .add_state(GameState::Menu)
        .add_plugin(menu::MenuPlugin)
        .run();
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum GameState {
    Game,
    Menu,
}

fn setup(mut commands: Commands, leaderboards: Res<Leaderboard>) {
    commands.spawn_bundle(Camera2dBundle::default());
    leaderboards.refresh_leaderboard();
}

mod menu {
    use bevy::prelude::*;
    use bevy_jornet::Leaderboard;

    use crate::{GameState, BACKGROUND, BUTTON, TEXT};
    pub struct MenuPlugin;

    impl Plugin for MenuPlugin {
        fn build(&self, app: &mut App) {
            app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(display_menu))
                .add_system_set(
                    SystemSet::on_update(GameState::Menu)
                        .with_system(button_system)
                        .with_system(display_scores),
                )
                .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(despawn_menu));
        }
    }
    fn display_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::ColumnReverse,
                    border: UiRect::all(Val::Px(30.0)),
                    ..default()
                },
                color: Color::hex(BACKGROUND).unwrap().into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section(
                    "Whac-A-Square",
                    TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 60.0,
                        color: Color::hex(TEXT).unwrap(),
                    },
                ));
                parent.spawn_bundle(TextBundle::from_section(
                    "Jornet Leaderboard Demo",
                    TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 35.0,
                        color: Color::hex(TEXT).unwrap(),
                    },
                ));

                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                            flex_direction: FlexDirection::ColumnReverse,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        },
                        color: Color::NONE.into(),
                        ..default()
                    })
                    .insert(LeaderboardMarker);

                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        color: Color::hex(BUTTON).unwrap().into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle::from_section(
                            "Play",
                            TextStyle {
                                font: asset_server.load("FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::hex(TEXT).unwrap(),
                            },
                        ));
                    });
            });
    }

    #[derive(Component)]
    struct LeaderboardMarker;

    fn display_scores(
        leaderboard: Res<Leaderboard>,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        root_ui: Query<Entity, (With<Node>, With<LeaderboardMarker>)>,
    ) {
        if leaderboard.is_changed() {
            info!("displaying leaderboard");
            if let Ok(root_ui) = root_ui.get_single() {
                for score in leaderboard.get_leaderboard() {
                    commands.entity(root_ui).with_children(|parent| {
                        parent.spawn_bundle(TextBundle::from_section(
                            format!("{} ", score.score),
                            TextStyle {
                                font: asset_server.load("FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::hex(TEXT).unwrap(),
                            },
                        ));
                    });
                }
            }
        }
    }

    fn despawn_menu(mut commands: Commands, root_ui: Query<Entity, (With<Node>, Without<Parent>)>) {
        for entity in &root_ui {
            commands.entity(entity).despawn_recursive();
        }
    }

    fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut UiColor),
            (Changed<Interaction>, With<Button>),
        >,
        mut state: ResMut<State<GameState>>,
    ) {
        for (interaction, mut color) in &mut interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    *color = (Color::hex(BUTTON).unwrap() + Color::GRAY).into();
                    let _ = state.set(GameState::Game);
                }
                Interaction::Hovered => {
                    *color = (Color::hex(BUTTON).unwrap() + Color::DARK_GRAY).into();
                }
                Interaction::None => {
                    *color = Color::hex(BUTTON).unwrap().into();
                }
            }
        }
    }
}

const CLEAR: &str = "023047";
const BACKGROUND: &str = "439775";
const BUTTON: &str = "2A4747";
const TEXT: &str = "8ecae6";
const SQUARE: &str = "219ebc";

use bevy::{prelude::*, time::Stopwatch};
use bevy_jornet::{JornetPlugin, Leaderboard};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Srgba(Srgba::hex(CLEAR).unwrap())))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Whac-A-Square".to_string(),
                canvas: Some("#demo-leaderboard".to_string()),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(JornetPlugin::with_leaderboard(
            option_env!("JORNET_LEADERBOARD_ID").unwrap_or("a920de64-3bdb-4f8e-87a8-e7bf20f00f81"),
            option_env!("JORNET_LEADERBOARD_KEY").unwrap_or("a797039b-a91d-43e6-8e1c-94f9ca0aa1d6"),
        ))
        .add_systems(Startup, setup)
        .init_state::<GameState>()
        .add_plugins((menu::MenuPlugin, game::GamePlugin, done::DonePlugin))
        .run();
}

#[derive(Clone, States, PartialEq, Eq, Debug, Hash)]
enum GameState {
    Game,
    Menu,
    Done,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Menu
    }
}

fn setup(mut commands: Commands, mut leaderboard: ResMut<Leaderboard>) {
    commands.spawn(Camera2dBundle::default());
    leaderboard.create_player(None);
}

mod menu {
    use std::{cmp::Ordering, time::Duration};

    use bevy::{
        color::palettes,
        prelude::*,
        winit::{UpdateMode, WinitSettings},
    };
    use bevy_jornet::Leaderboard;

    use crate::{GameState, BACKGROUND, BUTTON, TEXT};
    pub struct MenuPlugin;

    impl Plugin for MenuPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(GameState::Menu), display_menu)
                .add_systems(
                    Update,
                    (button_system, display_scores).run_if(in_state(GameState::Menu)),
                )
                .add_systems(OnExit(GameState::Menu), despawn_menu);
        }
    }

    fn display_menu(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        leaderboard: Res<Leaderboard>,
    ) {
        commands.insert_resource(WinitSettings {
            focused_mode: UpdateMode::reactive(Duration::from_secs_f32(0.5)),
            ..WinitSettings::desktop_app()
        });
        commands
            .spawn(NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    border: UiRect::all(Val::Px(30.0)),
                    ..default()
                },
                background_color: Color::Srgba(Srgba::hex(BACKGROUND).unwrap()).into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Whac-A-Square",
                    TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 60.0,
                        color: Color::Srgba(Srgba::hex(TEXT).unwrap()),
                    },
                ));
                parent.spawn(TextBundle::from_section(
                    "Jornet Leaderboard Demo",
                    TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 35.0,
                        color: Color::Srgba(Srgba::hex(TEXT).unwrap()),
                    },
                ));

                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(300.0),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::all(Val::Px(20.0)),
                                    ..default()
                                },

                                ..default()
                            },
                            LeaderboardMarker::Player,
                        ));
                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(150.0),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::all(Val::Px(20.0)),
                                    ..default()
                                },
                                ..default()
                            },
                            LeaderboardMarker::Score,
                        ));
                    });

                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,

                            ..default()
                        },
                        image: UiImage::default()
                            .with_color(Color::Srgba(Srgba::hex(BUTTON).unwrap())),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Play",
                            TextStyle {
                                font: asset_server.load("FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::Srgba(Srgba::hex(TEXT).unwrap()),
                            },
                        ));
                    });
            });
        commands.spawn((
            TextBundle::from_sections([
                TextSection {
                    value: "you are: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 20.0,
                        color: Color::Srgba(Srgba::hex(TEXT).unwrap()),
                    },
                },
                TextSection {
                    value: leaderboard
                        .get_player()
                        .map(|p| p.name.clone())
                        .unwrap_or_default(),
                    style: TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 25.0,
                        color: Color::Srgba(Srgba::hex(TEXT).unwrap()),
                    },
                },
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                bottom: Val::Px(10.0),
                ..default()
            }),
            PlayerName,
        ));

        leaderboard.refresh_leaderboard();
    }

    #[derive(Component)]
    struct PlayerName;

    #[derive(Component)]
    enum LeaderboardMarker {
        Score,
        Player,
    }

    fn display_scores(
        leaderboard: Res<Leaderboard>,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        root_ui: Query<(Entity, &LeaderboardMarker)>,
        mut player_name: Query<&mut Text, With<PlayerName>>,
    ) {
        if leaderboard.is_changed() {
            if let Some(player) = leaderboard.get_player() {
                player_name.single_mut().sections[1].value = player.name.clone();
            }
            let mut leaderboard = leaderboard.get_leaderboard();
            leaderboard.sort_unstable_by(|s1, s2| {
                s2.score.partial_cmp(&s1.score).unwrap_or(Ordering::Equal)
            });
            leaderboard.truncate(10);
            for (root_entity, marker) in &root_ui {
                commands.entity(root_entity).despawn_descendants();
                for score in &leaderboard {
                    commands.entity(root_entity).with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            match marker {
                                LeaderboardMarker::Score => format!("{} ", score.score),
                                LeaderboardMarker::Player => score.player.clone(),
                            },
                            TextStyle {
                                font: asset_server.load("FiraSans-Bold.ttf"),
                                font_size: 30.0,
                                color: Color::Srgba(Srgba::hex(TEXT).unwrap()),
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
            (&Interaction, &mut UiImage),
            (Changed<Interaction>, With<Button>),
        >,
        mut state: ResMut<NextState<GameState>>,
    ) {
        for (interaction, mut image) in &mut interaction_query {
            match *interaction {
                Interaction::Pressed => {
                    image.color =
                        (Color::Srgba(Srgba::hex(BUTTON).unwrap() + palettes::css::GRAY)).into();
                    state.set(GameState::Game);
                }
                Interaction::Hovered => {
                    image.color =
                        (Color::Srgba(Srgba::hex(BUTTON).unwrap() + palettes::css::DARK_GRAY))
                            .into();
                }
                Interaction::None => {
                    image.color = Color::Srgba(Srgba::hex(BUTTON).unwrap()).into();
                }
            }
        }
    }
}

#[derive(Resource)]
struct GameStatus {
    score: i32,
    time_to_click: Timer,
    since_start: Stopwatch,
}

mod game {
    use std::time::Duration;

    use bevy::{
        math::bounding::{Aabb2d, IntersectsVolume},
        prelude::*,
        time::Stopwatch,
        window::PrimaryWindow,
        winit::{UpdateMode, WinitSettings},
    };
    use bevy_jornet::Leaderboard;
    use rand::Rng;

    use crate::{GameState, GameStatus, SQUARE, TEXT};

    pub struct GamePlugin;

    impl Plugin for GamePlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(GameState::Game), setup_game)
                .add_systems(
                    Update,
                    (square_lifecycle, handle_clicks, game_state).run_if(in_state(GameState::Game)),
                )
                .add_systems(OnExit(GameState::Game), save_score);
        }
    }

    fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.insert_resource(WinitSettings {
            focused_mode: UpdateMode::reactive(Duration::from_secs_f32(0.05)),
            ..WinitSettings::desktop_app()
        });
        commands.insert_resource(GameStatus {
            score: 0,
            time_to_click: Timer::from_seconds(10.0, TimerMode::Once),
            since_start: Stopwatch::new(),
        });
        commands.spawn(
            TextBundle::from_section(
                "0",
                TextStyle {
                    font: asset_server.load("FiraSans-Bold.ttf"),
                    font_size: 50.0,
                    color: Color::Srgba(Srgba::hex(TEXT).unwrap()),
                },
            )
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(15.0),
                ..default()
            }),
        );
        commands.spawn(NodeBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(15.0),
                width: Val::Px(200.0),
                height: Val::Px(8.0),
                ..default()
            },
            background_color: Color::Srgba(Srgba::hex(SQUARE).unwrap()).into(),
            ..default()
        });
    }

    #[derive(Component)]
    struct SquareTimer(Timer);

    fn square_lifecycle(
        mut commands: Commands,
        mut status: ResMut<GameStatus>,
        primary_window_query: Query<&Window, With<PrimaryWindow>>,
        time: Res<Time>,
        mut squares: Query<(Entity, &mut SquareTimer)>,
    ) {
        let Ok(primary_window) = primary_window_query.get_single() else {
            return;
        };
        let mut rng = rand::thread_rng();
        if rng.gen_bool(time.delta_seconds_f64().min(1.0)) {
            let width = primary_window.width() / 2.0 - 50.0;
            let height = primary_window.height() / 2.0 - 50.0;
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::Srgba(Srgba::hex(SQUARE).unwrap()),
                        custom_size: Some(Vec2::splat(rng.gen_range(25.0..50.0))),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        rng.gen_range(-width..width),
                        rng.gen_range(-height..height),
                        0.0,
                    ),
                    ..default()
                },
                SquareTimer(Timer::from_seconds(
                    rng.gen_range(2.0..10.0),
                    TimerMode::Once,
                )),
            ));
        }
        for (entity, mut timer) in &mut squares {
            if timer.0.tick(time.delta()).just_finished() {
                commands.entity(entity).despawn();
                status.score -= 1;
            }
        }
    }

    fn handle_clicks(
        mut commands: Commands,
        mut status: ResMut<GameStatus>,
        squares: Query<(Entity, &Sprite, &Transform)>,
        mouse_input: Res<ButtonInput<MouseButton>>,
        primary_window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        if mouse_input.get_just_pressed().next().is_some() {
            let Ok(primary_window) = primary_window_query.get_single() else {
                return;
            };
            let mut clicked_at = primary_window.cursor_position().unwrap();
            clicked_at.x -= primary_window.width() / 2.0;
            clicked_at.y = -1.0 * (clicked_at.y - primary_window.height() / 2.0);
            for (entity, sprite, transform) in &squares {
                if Aabb2d::new(clicked_at, Vec2::ONE).intersects(&Aabb2d::new(
                    transform.translation.truncate(),
                    sprite.custom_size.unwrap() / 2.,
                )) {
                    commands.entity(entity).despawn();
                    status.score += 10;
                    status.time_to_click = Timer::from_seconds(
                        10.0 / (status.since_start.elapsed_secs() / 3.0),
                        TimerMode::Once,
                    );
                }
            }
        }
    }

    fn game_state(
        mut status: ResMut<GameStatus>,
        mut score_text: Query<&mut Text>,
        mut timer: Query<&mut Style, Without<Text>>,
        time: Res<Time>,
        mut state: ResMut<NextState<GameState>>,
    ) {
        score_text.single_mut().sections[0].value = format!("{}", status.score);
        timer.single_mut().width = Val::Px(status.time_to_click.fraction_remaining() * 200.0);
        status.since_start.tick(time.delta());
        if status.time_to_click.tick(time.delta()).just_finished() {
            state.set(GameState::Done);
        }
    }

    fn save_score(
        status: Res<GameStatus>,
        leaderboard: Res<Leaderboard>,
        mut commands: Commands,
        game_ui: Query<Entity, With<Node>>,
        squares: Query<Entity, With<Sprite>>,
    ) {
        for entity in &game_ui {
            commands.entity(entity).despawn();
        }
        for entity in &squares {
            commands.entity(entity).despawn();
        }
        leaderboard.send_score(status.score as f32);
    }
}

mod done {
    use bevy::prelude::*;

    use crate::{GameState, GameStatus, BACKGROUND, TEXT};

    pub struct DonePlugin;

    impl Plugin for DonePlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(GameState::Done), setup_done)
                .add_systems(Update, tick_done.run_if(in_state(GameState::Done)))
                .add_systems(OnExit(GameState::Done), clear_done);
        }
    }

    #[derive(Component)]
    struct DoneTimer(Timer);

    fn setup_done(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        game_status: Res<GameStatus>,
    ) {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        margin: UiRect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        border: UiRect::all(Val::Px(30.0)),
                        ..default()
                    },
                    background_color: Color::Srgba(Srgba::hex(BACKGROUND).unwrap()).into(),
                    ..default()
                },
                DoneTimer(Timer::from_seconds(3.0, TimerMode::Once)),
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Your Score",
                    TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::Srgba(Srgba::hex(TEXT).unwrap()),
                    },
                ));
                parent.spawn(TextBundle::from_section(
                    format!("{}", game_status.score),
                    TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 70.0,
                        color: Color::Srgba(Srgba::hex(TEXT).unwrap()),
                    },
                ));
            });
    }

    fn tick_done(
        time: Res<Time>,
        mut timer: Query<&mut DoneTimer>,
        mut state: ResMut<NextState<GameState>>,
    ) {
        if timer.single_mut().0.tick(time.delta()).just_finished() {
            state.set(GameState::Menu);
        }
    }

    fn clear_done(mut commands: Commands, ui: Query<Entity, With<DoneTimer>>) {
        commands.entity(ui.single()).despawn_recursive();
    }
}

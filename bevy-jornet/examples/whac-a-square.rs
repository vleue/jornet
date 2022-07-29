const CLEAR: &str = "023047";
const BACKGROUND: &str = "fb8500";
const BUTTON: &str = "ffb703";
const TEXT: &str = "8ecae6";
const SQUARE: &str = "219ebc";

use bevy::{prelude::*, time::Stopwatch};
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
        .add_plugin(JornetPlugin::with_leaderboard(
            Uuid::parse_str("fb0bbe22-b047-494d-9519-1d36668fa5bc").unwrap(),
        ))
        .add_startup_system(setup)
        .add_state(GameState::Menu)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(done::DonePlugin)
        .run();
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum GameState {
    Game,
    Menu,
    Done,
}

fn setup(mut commands: Commands, mut leaderboard: ResMut<Leaderboard>) {
    commands.spawn_bundle(Camera2dBundle::default());
    leaderboard.create_player(None);
}

mod menu {
    use std::time::Duration;

    use bevy::{
        prelude::*,
        winit::{UpdateMode, WinitSettings},
    };
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

    fn display_menu(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        leaderboard: Res<Leaderboard>,
    ) {
        commands.insert_resource(WinitSettings {
            focused_mode: UpdateMode::Reactive {
                max_wait: Duration::from_secs_f32(0.5),
            },
            ..WinitSettings::desktop_app()
        });
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
                            size: Size::new(Val::Px(150.0), Val::Undefined),
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
        commands.spawn_bundle(
            TextBundle::from_sections([
                TextSection {
                    value: "you are: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 20.0,
                        color: Color::hex(TEXT).unwrap(),
                    },
                },
                TextSection {
                    value: leaderboard.get_player_name().unwrap_or_default(),
                    style: TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 25.0,
                        color: Color::hex(TEXT).unwrap(),
                    },
                },
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    ..default()
                },
                ..default()
            }),
        );

        leaderboard.refresh_leaderboard();
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
            if let Ok(root_ui) = root_ui.get_single() {
                commands.entity(root_ui).despawn_descendants();
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

struct GameStatus {
    score: i32,
    time_to_click: Timer,
    since_start: Stopwatch,
}

mod game {
    use std::time::Duration;

    use bevy::{
        prelude::*,
        sprite::collide_aabb::collide,
        time::Stopwatch,
        winit::{UpdateMode, WinitSettings},
    };
    use bevy_jornet::Leaderboard;
    use rand::Rng;

    use crate::{GameState, GameStatus, SQUARE, TEXT};

    pub struct GamePlugin;

    impl Plugin for GamePlugin {
        fn build(&self, app: &mut App) {
            app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup_game))
                .add_system_set(
                    SystemSet::on_update(GameState::Game)
                        .with_system(square_lifecycle)
                        .with_system(handle_clicks)
                        .with_system(game_state),
                )
                .add_system_set(SystemSet::on_exit(GameState::Game).with_system(save_score));
        }
    }

    fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.insert_resource(WinitSettings {
            focused_mode: UpdateMode::Reactive {
                max_wait: Duration::from_secs_f32(0.05),
            },
            ..WinitSettings::desktop_app()
        });
        commands.insert_resource(GameStatus {
            score: 0,
            time_to_click: Timer::from_seconds(10.0, false),
            since_start: Stopwatch::new(),
        });
        commands.spawn_bundle(
            TextBundle::from_section(
                "0",
                TextStyle {
                    font: asset_server.load("FiraSans-Bold.ttf"),
                    font_size: 50.0,
                    color: Color::hex(TEXT).unwrap(),
                },
            )
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(10.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        );
        commands.spawn_bundle(NodeBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(0.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                size: Size::new(Val::Px(200.0), Val::Px(8.0)),
                ..default()
            },
            color: Color::hex(SQUARE).unwrap().into(),
            ..default()
        });
    }

    #[derive(Component)]
    struct SquareTimer(Timer);

    fn square_lifecycle(
        mut commands: Commands,
        mut status: ResMut<GameStatus>,
        windows: Res<Windows>,
        time: Res<Time>,
        mut squares: Query<(Entity, &mut SquareTimer)>,
    ) {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(time.delta_seconds_f64().min(1.0)) {
            let width = windows.primary().width() / 2.0 - 50.0;
            let height = windows.primary().height() / 2.0 - 50.0;
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::hex(SQUARE).unwrap(),
                        custom_size: Some(Vec2::splat(rng.gen_range(25.0..50.0))),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        rng.gen_range(-width..width),
                        rng.gen_range(-height..height),
                        0.0,
                    ),
                    ..default()
                })
                .insert(SquareTimer(Timer::from_seconds(
                    rng.gen_range(2.0..10.0),
                    false,
                )));
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
        mouse_input: Res<Input<MouseButton>>,
        windows: Res<Windows>,
    ) {
        if mouse_input.get_just_pressed().next().is_some() {
            let mut clicked_at = windows.primary().cursor_position().unwrap();
            clicked_at.x -= windows.primary().width() / 2.0;
            clicked_at.y -= windows.primary().height() / 2.0;
            for (entity, sprite, transform) in &squares {
                if collide(
                    clicked_at.extend(0.0),
                    Vec2::ONE,
                    transform.translation,
                    sprite.custom_size.unwrap(),
                )
                .is_some()
                {
                    commands.entity(entity).despawn();
                    status.score += 10;
                    status.time_to_click = Timer::from_seconds(
                        10.0 / (status.since_start.elapsed_secs() / 3.0),
                        false,
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
        mut state: ResMut<State<GameState>>,
    ) {
        score_text.single_mut().sections[0].value = format!("{}", status.score);
        timer.single_mut().size.width = Val::Px(status.time_to_click.percent_left() * 200.0);
        status.since_start.tick(time.delta());
        if status.time_to_click.tick(time.delta()).just_finished() {
            let _ = state.set(GameState::Done);
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
            app.add_system_set(SystemSet::on_enter(GameState::Done).with_system(setup_done))
                .add_system_set(SystemSet::on_update(GameState::Done).with_system(tick_done))
                .add_system_set(SystemSet::on_exit(GameState::Done).with_system(clear_done));
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
                    "Your Score",
                    TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::hex(TEXT).unwrap(),
                    },
                ));
                parent.spawn_bundle(TextBundle::from_section(
                    format!("{}", game_status.score),
                    TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 70.0,
                        color: Color::hex(TEXT).unwrap(),
                    },
                ));
            })
            .insert(DoneTimer(Timer::from_seconds(5.0, false)));
    }

    fn tick_done(
        time: Res<Time>,
        mut timer: Query<&mut DoneTimer>,
        mut state: ResMut<State<GameState>>,
    ) {
        if timer.single_mut().0.tick(time.delta()).just_finished() {
            let _ = state.set(GameState::Menu);
        }
    }

    fn clear_done(mut commands: Commands, ui: Query<Entity, With<DoneTimer>>) {
        commands.entity(ui.single()).despawn_recursive();
    }
}

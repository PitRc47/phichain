use super::{GameConfig, GameSet, GameViewport};
use crate::score::GameScore;
use bevy::prelude::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BaseTextScale(1.0))
            .add_systems(
                Update,
                (update_base_text_scale_system, update_text_scale_system)
                    .chain()
                    .in_set(GameSet),
            )
            .add_systems(Update, update_ui_text_margin_system)
            // combo
            .add_systems(Startup, setup_combo_ui_system)
            .add_systems(Update, update_combo_system.in_set(GameSet))
            .add_systems(Update, hide_combo_below_3_system.in_set(GameSet))
            // score
            .add_systems(Startup, spawn_score_ui_system)
            .add_systems(Update, update_score_system.in_set(GameSet))
            // name
            .add_systems(Startup, spawn_name_ui_system)
            .add_systems(Update, update_name_system.in_set(GameSet))
            // level
            .add_systems(Startup, spawn_level_ui_system)
            .add_systems(Update, update_level_system.in_set(GameSet));
    }
}

/// Scale based on [BaseTextScale] for a specific text
#[derive(Component, Debug)]
struct TextScale(f32);

/// Base game ui base text scale
#[derive(Resource, Debug)]
struct BaseTextScale(f32);

fn update_base_text_scale_system(
    mut scale: ResMut<BaseTextScale>,
    game_viewport: Res<GameViewport>,
) {
    scale.0 = if game_viewport.0.width() > game_viewport.0.height() * 0.75 {
        game_viewport.0.height() / 18.75
    } else {
        game_viewport.0.width() / 14.0625
    };
}

/// Marker component to represent the combo number text
#[derive(Component, Debug)]
struct ComboText;

/// Marker component to represent the "COMBO" text
#[derive(Component, Debug)]
struct ComboIndicator;

/// Marker component to represent the combo container
#[derive(Component, Debug)]
struct Combo;

#[derive(Component, Debug)]
struct ApplyMargin {
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
}

impl ApplyMargin {
    fn all() -> Self {
        Self {
            left: true,
            right: true,
            top: true,
            bottom: true,
        }
    }

    fn none() -> Self {
        Self {
            left: false,
            right: false,
            top: false,
            bottom: false,
        }
    }
}

fn update_ui_text_margin_system(
    mut query: Query<(&mut Style, &ApplyMargin)>,
    scale: Res<BaseTextScale>,
) {
    for (mut style, sides) in &mut query {
        let value = Val::Px(scale.0 * 0.5);
        let mut rect = UiRect::ZERO;
        if sides.left {
            rect.left = value;
        }
        if sides.right {
            rect.right = value;
        }
        if sides.top {
            rect.top = value;
        }
        if sides.bottom {
            rect.bottom = value;
        }
        style.margin = rect;
    }
}

fn setup_combo_ui_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                top: Val::Px(0.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            align_self: AlignSelf::Center,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                    Combo,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle {
                            text: Text::from_section(
                                "COMBO", // this will be replaced every frame at update_combo_system
                                TextStyle {
                                    font: asset_server.load("font/phigros.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            ),
                            ..default()
                        },
                        ComboText,
                        TextScale(1.0),
                        ApplyMargin {
                            left: false,
                            right: false,
                            top: true,
                            bottom: false,
                        },
                    ));

                    parent.spawn((
                        TextBundle {
                            text: Text::from_section(
                                "COMBO",
                                TextStyle {
                                    font: asset_server.load("font/phigros.ttf"),
                                    font_size: 10.0,
                                    color: Color::WHITE,
                                },
                            ),
                            ..default()
                        },
                        ComboIndicator,
                        TextScale(0.4),
                        ApplyMargin::none(),
                    ));
                });
        });
}

/// Marker component to represent the score text
#[derive(Component)]
struct ScoreText;

fn spawn_score_ui_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                right: Val::Px(0.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "0000000",
                        TextStyle {
                            font: asset_server.load("font/phigros.ttf"),
                            font_size: 10.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                },
                ScoreText,
                TextScale(0.8),
                ApplyMargin::all(),
            ));
        });
}

/// Marker component to represent the name text
#[derive(Component)]
struct NameText;

fn spawn_name_ui_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                bottom: Val::Px(0.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Name",
                        TextStyle {
                            font: asset_server.load("font/phigros.ttf"),
                            font_size: 10.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                },
                NameText,
                TextScale(0.5),
                ApplyMargin::all(),
            ));
        });
}

/// Marker component to represent the level text
#[derive(Component)]
struct LevelText;

fn spawn_level_ui_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                right: Val::Px(0.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Level",
                        TextStyle {
                            font: asset_server.load("font/phigros.ttf"),
                            font_size: 10.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                },
                LevelText,
                TextScale(0.5),
                ApplyMargin::all(),
            ));
        });
}

fn update_text_scale_system(scale: Res<BaseTextScale>, mut query: Query<(&mut Text, &TextScale)>) {
    for (mut text, text_scale) in &mut query {
        text.sections[0].style.font_size = scale.0 * 1.32 * text_scale.0;
    }
}

fn update_combo_system(mut text_query: Query<&mut Text, With<ComboText>>, score: Res<GameScore>) {
    let mut combo_text = text_query.single_mut();
    combo_text.sections[0].value = score.combo().to_string();
}

fn hide_combo_below_3_system(
    mut combo_query: Query<&mut Visibility, With<Combo>>,
    score: Res<GameScore>,
) {
    let mut visibility = combo_query.single_mut();
    *visibility = if score.combo() >= 3 {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };
}

fn update_score_system(
    mut score_text_query: Query<&mut Text, With<ScoreText>>,
    score: Res<GameScore>,
) {
    let mut score_text = score_text_query.single_mut();
    score_text.sections[0].value = score.score_text();
}

fn update_name_system(
    mut name_text_query: Query<&mut Text, With<NameText>>,
    config: Res<GameConfig>,
) {
    let mut name_text = name_text_query.single_mut();
    name_text.sections[0].value = config.name.replace(' ', "\u{00A0}");
}

fn update_level_system(
    mut name_text_query: Query<&mut Text, With<LevelText>>,
    config: Res<GameConfig>,
) {
    let mut name_text = name_text_query.single_mut();
    name_text.sections[0].value = config.level.replace(' ', "\u{00A0}");
}
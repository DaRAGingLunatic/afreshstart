use bevy::prelude::*;
use crate::game::ui::hud::styles::*;

use crate::game::ui::*;
use crate::game::ui::hud::components::{AxeManButton, AxeThrowerButton, GiantSlothButton, HudMenu, KnightAndHorseButton, MinerButton, OgreMagiButton, ShieldAndSwordButton};
use crate::game::ui::pause_menu::styles::*;

pub fn spawn_hud_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Spawning Pause Menu");
    build_hud_menu(&mut commands, &asset_server);
}

pub fn despawn_hud_menu(
    mut commands: Commands,
    hud_menu_query: Query<Entity, With<HudMenu>>,
) {
    if let Ok(hud_menu_entity) = hud_menu_query.get_single() {
        commands.entity(hud_menu_entity).despawn_recursive();
    }
}

// System Piping Example
pub fn build_hud_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let hud_menu_entity = commands
        .spawn((
            NodeBundle {
                style: HUD_MENU_STYLE,
                z_index: ZIndex::Local(1), // See Ref. 1
                ..default()
            },
            HudMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: HUD_MENU_CONTAINER_STYLE,
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "",
                                get_title_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                    // Miner Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: HUD_BUTTON_STYLE,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MinerButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Miner",
                                        get_button_text_style(&asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // AxeMan Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: HUD_BUTTON_STYLE,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            AxeManButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Axeman",
                                        get_button_text_style(&asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // Axe Thrower Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: HUD_BUTTON_STYLE,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            AxeThrowerButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Axe Thrower",
                                        get_button_text_style(&asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // Shield and Sword Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: HUD_BUTTON_STYLE,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            ShieldAndSwordButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Shield n' Sword",
                                        get_button_text_style(&asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // Knight and Horse Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: HUD_BUTTON_STYLE,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            KnightAndHorseButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Knight",
                                        get_button_text_style(&asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // Ogre Magi Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: HUD_BUTTON_STYLE,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            OgreMagiButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Ogre Magi",
                                        get_button_text_style(&asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // Giant Sloth Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: HUD_BUTTON_STYLE,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            GiantSlothButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Giant Sloth",
                                        get_button_text_style(&asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                });
        })
        .id();

    hud_menu_entity
}

// References
// 1. UI Z-Index
// https://github.com/bevyengine/bevy/blob/latest/examples/ui/z_index.rs

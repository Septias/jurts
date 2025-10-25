use crate::EditingMode;
use bevy::{
    feathers::{
        controls::{ButtonProps, ButtonVariant, button},
        theme::ThemedText,
    },
    prelude::*,
    ui_widgets::{Activate, observe},
};

#[allow(unused)]
#[derive(Component)]
pub struct CameraControlButton {
    pub axis: CameraAxis,
}

#[derive(Clone, Copy)]
pub enum CameraAxis {
    X,
    Y,
    Z,
}

#[derive(Component)]
pub struct EditingModeButton;

pub fn setup_ui(mut commands: Commands) {
    commands.spawn(ui_root());
}

pub fn update_editing_mode_button_text(
    editing_mode: Res<EditingMode>,
    mut query: Query<&mut Text, With<EditingModeButton>>,
) {
    if editing_mode.is_changed() {
        for mut text in query.iter_mut() {
            text.0 = if editing_mode.is_editing {
                "Edit Mode: ON".to_string()
            } else {
                "Edit Mode: OFF".to_string()
            };
        }
    }
}

fn ui_root() -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(10),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::SpaceBetween,
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            padding: UiRect::all(px(10)),
            ..default()
        },
        children![
            // Top left - Camera buttons
            (
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    column_gap: px(5),
                    ..default()
                },
                children![
                    (
                        button(
                            ButtonProps {
                                variant: ButtonVariant::Normal,
                                ..default()
                            },
                            (CameraControlButton {
                                axis: CameraAxis::X
                            },),
                            Spawn((Text::new("X"), ThemedText))
                        ),
                        observe(|_activate: On<Activate>| {
                            info!("X axis camera button clicked!");
                        })
                    ),
                    (
                        button(
                            ButtonProps {
                                variant: ButtonVariant::Normal,
                                ..default()
                            },
                            (CameraControlButton {
                                axis: CameraAxis::Y
                            },),
                            Spawn((Text::new("Y"), ThemedText))
                        ),
                        observe(|_activate: On<Activate>| {
                            info!("Y axis camera button clicked!");
                        })
                    ),
                    (
                        button(
                            ButtonProps {
                                variant: ButtonVariant::Normal,
                                ..default()
                            },
                            (CameraControlButton {
                                axis: CameraAxis::Z
                            },),
                            Spawn((Text::new("Z"), ThemedText))
                        ),
                        observe(|_activate: On<Activate>| {
                            info!("Z axis camera button clicked!");
                        })
                    ),
                ]
            ),
            // Top right - Editing mode button
            (
                Node {
                    // align_self: AlignSelf::End,
                    ..Default::default()
                },
                children![(
                    button(
                        ButtonProps {
                            variant: ButtonVariant::Normal,
                            ..default()
                        },
                        (EditingModeButton,),
                        Spawn((Text::new("Edit Mode: OFF"), ThemedText))
                    ),
                    observe(
                        |_activate: On<Activate>, mut editing_mode: ResMut<EditingMode>| {
                            editing_mode.is_editing = !editing_mode.is_editing;
                            info!("Editing mode toggled: {}", editing_mode.is_editing);
                        }
                    )
                )]
            ),
        ],
    )
}

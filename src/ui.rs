use bevy::{
    color::palettes::tailwind::GRAY_400,
    feathers::{
        FeathersPlugins,
        controls::{ButtonProps, button},
        rounded_corners::RoundedCorners,
        theme::ThemedText,
    },
    prelude::*,
    ui_widgets::{Activate, observe},
};

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
pub struct DrawerButton;

pub fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(ui_root());
}

fn ui_root() -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(100),
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
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    column_gap: px(5),
                    ..default()
                },
                children![
                    (
                        button(
                            ButtonProps {
                                corners: RoundedCorners::All,
                                ..default()
                            },
                            (CameraControlButton {
                                axis: CameraAxis::X
                            }, BackgroundColor(GRAY_400.into())),
                            Spawn((Text::new("X"), ThemedText))
                        ),
                        observe(|_activate: On<Activate>| {
                            info!("X axis camera button clicked!");
                        })
                    ),
                    (
                        button(
                            ButtonProps {
                                corners: RoundedCorners::All,
                                ..default()
                            },
                            (CameraControlButton {
                                axis: CameraAxis::Y
                            }, BackgroundColor(GRAY_400.into())),
                            Spawn((Text::new("Y"), ThemedText))
                        ),
                        observe(|_activate: On<Activate>| {
                            info!("Y axis camera button clicked!");
                        })
                    ),
                    (
                        button(
                            ButtonProps {
                                corners: RoundedCorners::All,
                                ..default()
                            },
                            (CameraControlButton {
                                axis: CameraAxis::Z
                            }, BackgroundColor(GRAY_400.into())),
                            Spawn((Text::new("Z"), ThemedText))
                        ),
                        observe(|_activate: On<Activate>| {
                            info!("Z axis camera button clicked!");
                        })
                    ),
                ]
            ),
            // Top right - Drawer button
            (
                button(
                    ButtonProps {
                        corners: RoundedCorners::All,
                        ..default()
                    },
                    (DrawerButton, BackgroundColor(GRAY_400.into())),
                    Spawn((Text::new("☰"), ThemedText))
                ),
                observe(|_activate: On<Activate>| {
                    info!("Drawer button clicked!");
                })
            ),
        ],
    )
}

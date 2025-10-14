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

pub fn setup_ui(mut commands: Commands) {
    commands.spawn(ui_root());
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
            // // Top right - Drawer button
            // (
            //     button(
            //         ButtonProps {
            //             corners: RoundedCorners::All,
            //             ..default()
            //         },
            //         (DrawerButton, BackgroundColor(GRAY_400.into())),
            //         Spawn((Text::new("☰"), ThemedText))
            //     ),
            //     observe(|_activate: On<Activate>| {
            //         info!("Drawer button clicked!");
            //     })
            // ),
        ],
    )
}

use bevy::{
    prelude::*,
};
use crate::{
    ui::*,
};

pub fn setup(
    mut commands: Commands,
    style: Res<MainStyle>,
) {
    // commands.spawn_bundle(NodeBundle {
    //     style: Style {
    //         display: Display::Flex,
    //         position_type: PositionType::Absolute,
    //         flex_direction: FlexDirection::ColumnReverse,
    //         align_items: AlignItems::Center,
    //         justify_content: JustifyContent::FlexStart,
    //         position: Rect {
    //             left: Val::Px(10.0),
    //             top: Val::Px(10.0),
    //             ..Default::default()
    //         },
    //         padding: Rect::all(Val::Px(5.0)),
    //         size: Size {
    //             width: Val::Px(100.0),
    //             height: Val::Px(400.0),
    //         },
    //         // min_size: Size<Val>,
    //         ..Default::default()
    //     },
    //     material: style.panel.color.clone(),
    //     ..Default::default()
    // })
    // .with_children(|parent| {
    //     parent.spawn_bundle(TextBundle {
    //         text: Text::with_section(
    //             "Game mode:",
    //             style.text.clone(),
    //             Default::default()
    //         ),
    //         ..Default::default()
    //     });
    //     parent.spawn_bundle(NodeBundle {
    //         style: Style {
    //             display: Display::Flex,
    //             flex_direction: FlexDirection::Row,
    //             size: Size {
    //                 width: Val::Percent(100.0),
    //                 height: Val::Px(50.0),
    //                 ..Default::default()
    //             },
    //             ..Default::default()
    //         },
    //         material: style.panel.clear.clone(),
    //         ..Default::default()
    //     })
    //     .with_children(|parent| {
    //         parent.spawn_bundle(TextBundle {
    //             text: Text::with_section(
    //                 "Editor",
    //                 style.text.clone(),
    //                 Default::default()
    //             ),
    //             ..Default::default()
    //         });
    //         parent.spawn_bundle(ButtonBundle {
    //             style: Style {
    //                 size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
    //                 ..Default::default()
    //             },
    //             material: style.button.color_normal.clone(),
    //             ..Default::default()
    //         });
    //     });
    // })
    // ;
}
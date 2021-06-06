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
    // commands.spawn_bundle(NodeBundle { // CONTAINER
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
    //         // padding: Rect::all(Val::Px(5.0)),
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
    //     parent.spawn_bundle(TextBundle { // HEADER
    //         style: Style {
    //             margin: Rect::all(Val::Px(5.0)),
    //             ..Default::default()
    //         },
    //         text: Text::with_section(
    //             "Controls mode".to_string(),
    //             style.text.clone(),
    //             Default::default()
    //         ),
    //         ..Default::default()
    //     });
    //     parent.spawn_bundle(NodeBundle { // RADIOBUTTON CONTAINER
    //         style: Style {
    //             display: Display::Flex,
    //             flex_direction: FlexDirection::Row,
    //             margin: Rect::all(Val::Px(5.0)),
    //             size: Size {
    //                 width: Val::Percent(100.0),
    //                 ..Default::default()
    //             },
    //             ..Default::default()
    //         },
    //         material: style.panel.clear.clone(),
    //         ..Default::default()
    //     })
    //     .with_children(|parent| {
    //         parent.spawn_bundle(TextBundle { // RADIOBUTTON LABEL
    //             style: Style {
    //                 margin: Rect::all(Val::Px(5.0)),
    //                 ..Default::default()
    //             },
    //             text: Text::with_section(
    //                 "Editor".to_string(),
    //                 style.text.clone(),
    //                 Default::default()
    //             ),
    //             ..Default::default()
    //         });
    //         parent.spawn_bundle(ButtonBundle { // RADIO BUTTON BUTTON
    //             style: Style {
    //                 size: Size::new(Val::Px(150.0), Val::Px(65.0)),
    //                 margin: Rect::all(Val::Auto),
    //                 justify_content: JustifyContent::Center,
    //                 align_items: AlignItems::Center,
    //                 ..Default::default()
    //             },
    //             material: materials.add(Color::GRAY.into()),
    //             ..Default::default()
    //         });
    //     });
    // })
    // ;
}
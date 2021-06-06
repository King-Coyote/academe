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
    commands.spawn_bundle(NodeBundle {
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            position: Rect {
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                ..Default::default()
            },
            padding: Rect::all(Val::Px(5.0)),
            size: Size {
                width: Val::Px(100.0),
                height: Val::Px(400.0),
            },
            // min_size: Size<Val>,
            ..Default::default()
        },
        material: style.panel.color.clone(),
        ..Default::default()
    })
    // .with_children(|parent| {
    //     parent.spawn_bundle(TextBundle {
    //         node: Node,
    //         style: Style,
    //         draw: Draw,
    //         visible: Visible,
    //         text: Text,
    //         calculated_size: CalculatedSize,
    //         focus_policy: FocusPolicy,
    //         transform: Transform,
    //         global_transform: GlobalTransform,
    //         ..Default::default()
    //     })
    // })
    ;
}
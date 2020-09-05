use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .init_resource::<Assets<WidgetColors>>()
        .add_startup_system(setup.system())
        .add_system(buttons.system());
    }
}

pub struct WidgetColors {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

fn buttons(
    widget_colors: Res<Assets<WidgetColors>>,
    mut interaction_query: Query<(
        &Button,
        Mutated<Interaction>,
        &mut Handle<WidgetColors>,
        &mut Handle<ColorMaterial>,
        &Children,
    )>,
    text_query: Query<&mut Text>,
) {
    for (_button, interaction, colors_h, mut material, children) in &mut interaction_query.iter() {
        let mut text = text_query.get_mut::<Text>(children[0]).unwrap();
        let colors = widget_colors.get(&colors_h).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.value = "Press".to_string();
                *material = colors.pressed;
            }
            Interaction::Hovered => {
                text.value = "Hover".to_string();
                *material = colors.hovered;
            }
            Interaction::None => {
                text.value = "Button".to_string();
                *material = colors.normal;
            }
        }
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut widget_colors: ResMut<Assets<WidgetColors>>,
    mut colors: ResMut<Assets<ColorMaterial>>,
) {
    // let color_normal = colors.add(Color::rgb(0.5, 0.5, 0.5).into());
    // let colors_handle = widget_colors.add(WidgetColors{
    //     normal: color_normal,
    //     hovered: colors.add(Color::rgb(0.5, 0.5, 0.7).into()),
    //     pressed: colors.add(Color::rgb(0.5, 1.0, 0.5).into()),
    // });
    // commands
    //     .spawn(UiCameraComponents::default())
    //     .spawn(NodeComponents {
    //         style: Style {
    //             size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    //             justify_content: JustifyContent::SpaceBetween,
    //             ..Default::default()
    //         },
    //         material: colors.add(Color::NONE.into()),
    //         ..Default::default()
    //     })
    //     .with_children(|parent| {
    //         parent.spawn(ButtonComponents {
    //             style: Style {
    //                 size: Size::new(Val::Px(150.0), Val::Px(75.0)),
    //                 margin: Rect::all(Val::Auto),
    //                 justify_content: JustifyContent::Center,
    //                 align_items: AlignItems::Center,
    //                 ..Default::default()
    //             },
    //             material: color_normal,
    //             ..Default::default()
    //         })
    //         .with(colors_handle)
    //         .with_children(|parent| {
    //             parent.spawn(TextComponents {
    //                 text: Text {
    //                     value: "Button".to_string(),
    //                     font: asset_server.load("assets/fonts/OpenSans-Regular.ttf").unwrap(),
    //                     style: TextStyle {
    //                         font_size: 40.0,
    //                         color: Color::rgb(0.8, 0.8, 0.8),
    //                     },
    //                 },
    //                 ..Default::default()
    //             });
    //         });
    //     });
}
use bevy::prelude::*;

pub struct ElementMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromResources for ElementMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ElementMaterials {
            normal: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            hovered: materials.add(Color::rgb(0.5, 0.7, 0.5).into()),
            pressed: materials.add(Color::rgb(0.5, 1.0, 0.5).into()),
        }
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<ElementMaterials>,
    mut materials_assets: ResMut<Assets<ColorMaterial>>
) {
    commands
        .spawn(UiCameraComponents::default())
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: materials_assets.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(ButtonComponents {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(75.0)),
                    margin: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: materials.normal,
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextComponents {
                    text: Text {
                        value: "Button".to_string(),
                        font: asset_server.load("cuntassets/fonts/OpenSans-Regular.ttf").unwrap(),
                        style: TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                        },
                    },
                    ..Default::default()
                });
            });
        });
}
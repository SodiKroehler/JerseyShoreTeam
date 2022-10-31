use bevy::{prelude::*};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui);
    }
}

fn despawn_gui(mut commands: Commands, button_query: Query<Entity, With<Button>>) {
    for ent in button_query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    //commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(
        TextBundle::from_section(
            "hello\nbevy!",
            TextStyle {
                font: asset_server.load("Jersey.ttf"),
                font_size: 100.0,
                color: Color::WHITE,
            },
        ) 
       // .with_text_alignment(TextAlignment::TOP_CENTER)
        .with_style(Style {
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        }),
    );
}


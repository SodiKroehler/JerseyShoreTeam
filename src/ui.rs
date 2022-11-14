use bevy::{prelude::*};
use super::GameState;
use iyes_loopless::prelude::*;
use super::shared_styles::*;

pub struct UiPlugin;

#[derive(Component, Deref, DerefMut)]
struct CreditTimer(Timer);


#[derive(Component)]
struct pause_screen;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
      // .add_startup_system(setup_ui)
        .add_system (
            process_triggers
                .run_in_state(GameState::InGame))
        .add_enter_system(GameState::Ending, setup_credits)
        .add_system (roll_credits.run_in_state(GameState::Ending))
        .add_exit_system(GameState::Ending, exit_credits)
        .add_enter_system(GameState::Paused, enter_paused)
        .add_system(pause_button.run_in_state(GameState::Paused))
        .add_exit_system(GameState::Paused, exit_paused);
    }
}

fn process_triggers(mut commands: Commands, kbd: Res<Input<KeyCode>>) {
    if kbd.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(GameState::Paused));
    }
    if kbd.just_pressed(KeyCode::B) {
        commands.insert_resource(NextState(GameState::Rover));
    }
    if kbd.just_pressed(KeyCode::C) {
        commands.insert_resource(NextState(GameState::Ending));
    }
}

fn setup_credits(mut commands: Commands,
                asset_server: Res<AssetServer>){
    let initial_offset: f32 = 640. + (1280.*3.);
    commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("credit-sheet.png"),
            transform: Transform::from_xyz(initial_offset, 0., 1.),
			..default()
		})
        .insert(CreditTimer(Timer::from_seconds(5., true)));
}

fn roll_credits(
	time: Res<Time>,
    mut commands: Commands,
	mut popup: Query<(&mut CreditTimer, &mut Transform)>) {

    let counter = -4800;
	for (mut timer, mut transform) in popup.iter_mut() {
		timer.tick(time.delta());
		if timer.just_finished() {
			transform.translation.x -= 1280.;
            // if counter == transform.translation.x {
            if counter == -4800 {
                timer.pause();
                commands.insert_resource(NextState(GameState::Paused));
            }
		}
	}
}

fn exit_credits(mut commands: Commands,
                qu: Query<Entity, With<CreditTimer>>) {

    for ent in qu.iter() {
        commands.entity(ent).despawn_recursive();    
    }
}

fn enter_paused(mut commands: Commands,
                asset_server: Res<AssetServer>){

    commands.spawn_bundle(NodeBundle{
        transform: Transform::from_xyz(0.0,0.0,1.0),
        style: Style {
            size: Size::new(Val::Px(1280.0), Val::Px(720.0)),
            ..default()
        },
        color: Color::rgba(0.0, 0.0, 0.0, 0.60).into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn_bundle(ButtonBundle {
            transform: Transform::from_xyz(0.0,0.0,4.0),
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: XP_BLUE.into(),
            ..default()
        }).with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Play!",
                TextStyle {
                    font: asset_server.load("Jersey.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ));
        });
    }).insert(pause_screen);

}

fn pause_button(mut commands: Commands,
                mut inter_query: Query<(&Interaction, &mut UiColor),
                                (Changed<Interaction>, With<Button>)>){

    for (interaction, mut color) in &mut inter_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                commands.insert_resource(NextState(GameState::InGame));
            }
            Interaction::Hovered => {
               // *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = XP_BLUE.into();
            }
        }
    }
}

fn exit_paused(mut commands: Commands,
                q : Query<Entity, With<pause_screen>>){
    for ent in q.iter() {
        commands.entity(ent).despawn_recursive();    
    }
}
use bevy::{
	prelude::*,
	window::PresentMode, ecs::system::EntityCommands,
};

#[derive(Component, Deref, DerefMut)]
struct CreditTimer(Timer);

fn main() {
	App::new()
		.insert_resource(WindowDescriptor {
			title: String::from("iExplorer"),
			width: 1280.,
			height: 720.,
			present_mode: PresentMode::Fifo,
			..default()
		})
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_system(roll_credits)
		.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let initial_offset: f32 = 640. + (1280.*3.);
	commands.spawn_bundle(Camera2dBundle::default());
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("credit-sheet.png"),
            transform: Transform::from_xyz(initial_offset, 0., 0.),
			..default()
		})
        .insert(CreditTimer(Timer::from_seconds(5., true)));

	info!("Hello world!");
}

fn roll_credits(
	time: Res<Time>,
	mut popup: Query<(&mut CreditTimer, &mut Transform)>
) {
    let counter = -4480.;
	for (mut timer, mut transform) in popup.iter_mut() {
		timer.tick(time.delta());
		if timer.just_finished() {
			transform.translation.x -= 1280.;
            if counter == transform.translation.x {timer.pause()}
			info!("Moved to next");
		}
	}
}
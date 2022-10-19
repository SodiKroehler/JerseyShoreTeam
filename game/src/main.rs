use bevy::{
	prelude::*,
	window::PresentMode, ecs::system::EntityCommands,
};

#[derive(Component, Deref, DerefMut)]
struct CreditTimer(Timer);

#[derive(Component)]
struct Player{
	x_pos: i32,
	y_pos: i32,
	max_x_speed: f32,
	x_accel: f32,
	grav: f32,
	jump_speed: f32,
}



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
		//.add_system(roll_credits)
		.add_system(move_player)
		.run();
}

fn setup(mut commands: Commands, 
	mut materials: ResMut<Assets<ColorMaterial>>, 
	asset_server: Res<AssetServer>) {
    let initial_offset: f32 = 640. + (1280.*3.);
	commands.spawn_bundle(Camera2dBundle::default());
	/*commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("credit-sheet.png"),
            transform: Transform::from_xyz(initial_offset, 0., 0.),
			..default()
		})
        .insert(CreditTimer(Timer::from_seconds(5., true)));
	*/
	commands.spawn()
		.insert_bundle(SpriteBundle{
		texture: asset_server.load("player_standin.png"),
		transform: Transform::from_xyz(0.0,0.0,0.0),
		..default()
		})
		.insert(Player{
			x_pos: 0,
			y_pos: 0,
			max_x_speed: 80.0,
			x_accel: 1.0,
			grav: -60.0,
			jump_speed: 100.0,
		});

		
	info!("Hello world!");
}
fn move_player(
	time: Res<Time>,
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&Player, &mut Transform)>,
){
	for(player, mut transform) in query.iter_mut(){
		let mut x_dir = 0.0;
		let mut jumping = 0.0;
		if keyboard_input.pressed(KeyCode::Left){
			x_dir -= 1.0;
			//info!("left");
		}
		if keyboard_input.pressed(KeyCode::Right){
			x_dir += 1.0;
			//info!("right");
		}
		if keyboard_input.pressed(KeyCode::Space){
			jumping += 1.0;
			//info!("jump");
		}

		let translation = &mut transform.translation;
		//accelerate in horizontal
		translation.x += time.delta_seconds()*x_dir * player.max_x_speed;
		translation.x = translation.x.clamp(-620.0,620.0);
		
		//let y_val = &mut transform.local_y().y;
		//if y_val == -300.0{
		translation.y += time.delta_seconds()*jumping * player.jump_speed;
		translation.y += time.delta_seconds()*player.grav;
		translation.y = translation.y.clamp(-335.0,335.0);
	}
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
		.add_system(roll_credits)

}
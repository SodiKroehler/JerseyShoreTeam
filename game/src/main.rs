use bevy::{
	prelude::*,
	window::PresentMode, ecs::system::EntityCommands,
};
use bevy::math::Vec2; 
use bevy::math::Vec3;
use bevy::asset::LoadState;
mod collide_circle;
use collide_circle::Collision;

#[derive(Component, Deref, DerefMut)]
struct CreditTimer(Timer);
#[derive(Component)]
struct Size{
	size: Vec2,
}
struct FolderSpawnEvent(Vec3);//holds the position vector of the spawner
#[derive(Component)]
struct Player{}
#[derive(Component)]
struct Folder{}
#[derive(Component)]
struct Physics{
	delta_x: f32,
	delta_y: f32,
	gravity: f32,
}

#[derive(Component)]
struct Recycle{}
static SCREEN_WIDTH:f32 = 1280.0;
static SCREEN_HEIGHT:f32 = 720.0;
fn main() {
	App::new()
		.insert_resource(WindowDescriptor {
			title: String::from("iExplorer"),
			width: SCREEN_WIDTH,
			height: SCREEN_HEIGHT,
			present_mode: PresentMode::Fifo,
			..default()
		})
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_event::<FolderSpawnEvent>()
		//.add_system(roll_credits)
		.add_system(move_everything)
		.add_system(run_collisions)
		.add_system(spawn_folder)
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
	commands.spawn().insert_bundle(SpriteBundle{
		texture: asset_server.load("folder.png"),
		..default()
	});
	/*commands.spawn().insert_bundle(SpriteBundle{
		texture: asset_server.load("windows_xd.png"),
		..default()
	});*/
	commands.spawn()
		.insert_bundle(SpriteBundle{
		texture: asset_server.load("player_standin.png"),//30x50
		transform: Transform::from_xyz(0.0,0.0,0.0),
		..default()
		}).insert(Player{
		})
		.insert(Size{
			size: Vec2{
				x:30.0,
				y:50.0,
			}
		}).insert(Physics{
			delta_x:0.0,
			delta_y:0.0,
			gravity:1.0,
		});
	
	commands.spawn()
		.insert_bundle(SpriteBundle{
		texture: asset_server.load("recycle_bin1.png"),
		transform: Transform::from_xyz(50.0,-290.0,0.0),
		..default()
		}).insert(Size{
			size: Vec2{
				x:37.0,
				y:43.0,
			}
		})
		.insert(Recycle{
		})
		.insert(Physics{
			delta_x:0.0,
			delta_y:0.0,
			gravity:1.0,
		});

}
fn spawn_folder(
	asset_server: Res<AssetServer>,
	mut ev_spawnfolder : EventReader<FolderSpawnEvent>,
	mut commands: Commands,
	entity_cap : Query<&Folder>,
){
	let c = entity_cap.iter().count();
	if c <= 4{
		for ev in ev_spawnfolder.iter(){
			commands.spawn()
				.insert_bundle(SpriteBundle{
				texture: asset_server.get_handle("folder.png"),
				transform: Transform::from_translation(ev.0),
				..default()
				}).insert(Folder{
				}).insert(Size{
					size: Vec2{
						x:37.0,
						y:32.0,
					}
				})
				.insert(Physics{
					delta_x:0.0,
					delta_y:0.0,
					gravity:1.0,
				});
			info!("spawned folder");
		}
	}
}
fn inbounds(trans : Vec3, size : Vec2,)->Vec3{
	return Vec3{
		x : trans.x.clamp(((-1.0*SCREEN_WIDTH)/2.0)+(size.x/2.0),((1.0*SCREEN_WIDTH)/2.0)-(size.x/2.0)),
		y : trans.y.clamp(((-1.0*SCREEN_HEIGHT)/2.0)+(size.y/2.0),((1.0*SCREEN_HEIGHT)/2.0)-(size.y/2.0)),
		z : 0.0
	};
}
fn run_collisions(//first object is colliding into second
	time: Res<Time>,
	mut ev_spawnfolder : EventWriter<FolderSpawnEvent>,
	mut obj_list: Query<(Entity, &Size, &mut Transform, &mut Physics, Option<&Player>, Option<&Recycle>)>,
){
	let mut obj_pairs = obj_list.iter_combinations_mut();
	while let Some([(e1, object1, mut transform1, mut phys1, player1, recycle1), (e2, object2, mut transform2, mut phys2, player2, recycle2)]) = obj_pairs.fetch_next(){
		if e1 != e2{//think this is pointless
			let translation1 = &mut transform1.translation;
			let translation2 = &mut transform2.translation;
			let size1 = object1.size;
			let size2 = object2.size;
			const LAUNCH: f32 = 80.0;
			const X_MAX_VEL: f32 = 100.0;
			const Y_MAX_VEL: f32 = 100.0;
			const RESTITUTION: f32 = 0.45;
			const CUTOFF: f32 = 60.0;
			let c = collide_circle::collide(*translation1,size1,*translation2,size2);
			if c.is_some(){
				info!("collide");
				if let Some(player1)=player1{
					
					if let Some(recycle2)=recycle2{	
						ev_spawnfolder.send(FolderSpawnEvent(*translation2));
					}
				}
				let temp1x = phys1.delta_x;
				let temp1y = phys1.delta_y;
				match c{
					/*Some(Collision::Left)=>{phys1.delta_x=phys2.delta_x*RESTITUTION;phys2.delta_x=temp1x*RESTITUTION;},
					Some(Collision::Right)=>{phys1.delta_x=phys2.delta_x*RESTITUTION;phys2.delta_x=temp1x*RESTITUTION;},
					Some(Collision::Top)=>{phys1.delta_y=phys2.delta_y*RESTITUTION;phys2.delta_y=temp1y*RESTITUTION;},
					Some(Collision::Bottom)=>{phys1.delta_y=phys2.delta_y*RESTITUTION;phys2.delta_y=temp1y*RESTITUTION;},
					Some(Collision::Inside)=>{phys1.delta_x=phys2.delta_x*RESTITUTION;phys2.delta_x=temp1x*RESTITUTION;},
					None=>(),*/
					Some(Collision::Left)=>{phys2.delta_x+=LAUNCH;phys1.delta_x-=LAUNCH;},
					Some(Collision::Right)=>{phys2.delta_x-=LAUNCH;phys1.delta_x+=LAUNCH;},
					Some(Collision::Top)=>{phys2.delta_y-=LAUNCH;phys1.delta_y+=LAUNCH;},
					Some(Collision::Bottom)=>{phys2.delta_y+=LAUNCH;phys1.delta_y-=LAUNCH;},
					Some(Collision::Inside)=>{phys2.delta_x+=LAUNCH;phys1.delta_x-=LAUNCH;},
					None=>(),
				}
				if phys1.delta_x.abs() < CUTOFF{
					phys1.delta_x = 0.0;
				}
				if phys2.delta_x.abs() < CUTOFF{
					phys2.delta_x = 0.0;
				}
				if phys1.delta_y.abs() < CUTOFF{
					phys1.delta_y = 0.0;
					phys1.gravity = 0.0;
				}
				if phys2.delta_y.abs() < CUTOFF{
					phys2.delta_y = 0.0;
					phys2.gravity = 0.0;
				}
				phys1.delta_y *= RESTITUTION;
				phys2.delta_y *= RESTITUTION;
				translation1.x += time.delta_seconds()*phys1.delta_x;
				translation1.y += time.delta_seconds()*phys1.delta_y;
				translation2.x += time.delta_seconds()*phys2.delta_x;
				translation2.y += time.delta_seconds()*phys2.delta_y;
				phys1.delta_x = phys1.delta_x.clamp(-X_MAX_VEL, X_MAX_VEL);
				phys2.delta_x = phys2.delta_x.clamp(-X_MAX_VEL, X_MAX_VEL);
				phys1.delta_y = phys1.delta_y.clamp(-Y_MAX_VEL, Y_MAX_VEL);
				phys2.delta_y = phys2.delta_y.clamp(-Y_MAX_VEL, Y_MAX_VEL);
				*translation1 = inbounds(*translation1, object1.size);
				*translation2 = inbounds(*translation2, object2.size);
			}
			else{
				info!("no collide");
				phys1.gravity = 1.0;
				phys2.gravity = 1.0;
			}
		}
	}
}
fn move_everything(
	time: Res<Time>,
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&mut Physics, &Size, &mut Transform, Option<&Player>)>,
){
	const X_ACCEL: f32 = 25.0;
	const X_MAX_VEL: f32 = 100.0;
	const GRAV: f32 = 10.0;
	const Y_ACCEL: f32 = 200.0;
	const FRICTION_SCALE: f32 = 0.75;
	for (mut phys, object, mut transform, player) in query.iter_mut(){
		let translation = &mut transform.translation;
		//accelerate in horizontal
		phys.delta_y -= GRAV * phys.gravity;
		if let Some(player)=player{
			info!("y vel:{}",phys.delta_y);
			let mut jumping = 0.0;
			if keyboard_input.pressed(KeyCode::Left){
				phys.delta_x-= X_ACCEL;
				//info!("left");
			}
			if keyboard_input.pressed(KeyCode::Right){
				phys.delta_x+= X_ACCEL;
				//info!("right");
			}
			if keyboard_input.pressed(KeyCode::Space){
				jumping = 1.0;
				//info!("jump");
			}
			if translation.y <= -335.0 /*|| phys.delta_y.abs() < 1.0*/{//note: need to replace this with a function that checks for grounded for all physics entities
				phys.delta_y = 0.0;
				phys.delta_y += jumping * Y_ACCEL;
			}
		}
		else{
			if translation.y <= (-1.0*SCREEN_HEIGHT/2.0) +(object.size.y/2.0){
				phys.delta_y = 0.0;
			}
		}
		
		
		phys.delta_x = phys.delta_x.clamp(-X_MAX_VEL, X_MAX_VEL);
		translation.x += time.delta_seconds()*phys.delta_x;
		translation.y += time.delta_seconds()*phys.delta_y;
		
		phys.delta_x *= FRICTION_SCALE;
		*translation = inbounds(*translation, object.size);
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

}

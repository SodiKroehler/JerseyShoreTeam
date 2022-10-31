use bevy::{
	prelude::*,
	window::PresentMode, ecs::system::EntityCommands,
};
use bevy::math::Vec2; 
use bevy::math::Vec3;
use bevy::asset::LoadState;
mod collide_circle;
use collide_circle::Collision;
mod collidenew;
use collidenew::Shape;
use collidenew::CollisionInfo;

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

mod rover;
use rover::RoverPlugin;
mod ui;
use ui::UiPlugin;

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
		.add_plugin(RoverPlugin)
		.add_plugin(UiPlugin)
		.add_event::<FolderSpawnEvent>()
		//.add_system(roll_credits)
		.add_system(move_everything)
		.add_system(run_collisions3)
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
	let handy:Handle<Image> = asset_server.load("folder3.png");
	commands.spawn().insert_bundle(SpriteBundle{
		texture: handy,
		..default()
	});
	commands.spawn()//note: shape vertices start at up-left most vertex and rotate clockwise, angle is math-based (0 at +x)
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
		}).insert(Shape{
			vertices: vec![Vec3::new(-15.0,25.0,0.0),Vec3::new(15.0,25.0,0.0),Vec3::new(15.0,-25.0,0.0),Vec3::new(-15.0,-25.0,0.0)],
			origin: Vec3::new(0.0,0.0,0.0),//needs to be same as starting transform
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
		}).insert(Shape{
			vertices: vec![Vec3::new(-18.5,21.5,0.0),Vec3::new(18.5,21.5,0.0),Vec3::new(15.5,-21.5,0.0),Vec3::new(-15.5,-21.5,0.0)],
			origin: Vec3::new(50.0,-290.0,0.0),//needs to be same as starting transform
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
			//info!("x:{} y:{}",ev.0.x,ev.0.y);
			commands.spawn()
				.insert_bundle(SpriteBundle{
				texture: asset_server.load("folder3.png"),
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
				}).insert(Shape{
					vertices: vec![Vec3::new(-18.5,16.0,0.0),Vec3::new(18.5,16.0,0.0),Vec3::new(18.5,-16.0,0.0),Vec3::new(-18.5,-16.0,0.0)],
					origin: ev.0,//needs to be same as starting transform
				});

			//info!("spawned folder");
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
fn run_collisions3(//first object is colliding into second
	time: Res<Time>,
	mut ev_spawnfolder : EventWriter<FolderSpawnEvent>,
	mut obj_list: Query<(&Size, &mut Transform, &mut Physics, &mut Shape, Option<&Player>, Option<&Recycle>, Option<&Folder>)>,
){
	let mut obj_pairs = obj_list.iter_combinations_mut();
	while let Some([(object1, mut transform1, mut phys1, mut shape1, player1, recycle1, folder1), (object2, mut transform2, mut phys2, mut shape2, player2, recycle2, folder2)]) = obj_pairs.fetch_next(){
		if let Some(folder1) = folder1{
			info!("folder 1");
		}
		if let Some(folder2) = folder2{
			info!("folder 2");
		}
		let translation1 = &mut transform1.translation;
		let translation2 = &mut transform2.translation;
		let size1 = object1.size;
		let size2 = object2.size;
		const LAUNCH: f32 = 80.0;
		const X_MAX_VEL: f32 = 100.0;
		const Y_MAX_VEL: f32 = 100.0;
		const RESTITUTION: f32 = 0.45;
		const CUTOFF: f32 = 60.0;
		if let Some(folder1) = folder1{
			//info!("folder 1 x:{} y:{}",shape1.origin.x,shape1.origin.y);
		}
		if let Some(folder2) = folder2{
			//info!("folder 2 x:{} y:{}",shape2.origin.x,shape2.origin.y);
		}
		let c = collidenew::sat(&*shape1,&*shape2);
		if c.is_some(){//if collision
			if let Some(player1) = player1{
				if let Some(recycle2) = recycle2{
					ev_spawnfolder.send(FolderSpawnEvent(*translation2));
				}
			}
			if let Some(recycle1) = recycle1{
				if let Some(player2) = player2{
					ev_spawnfolder.send(FolderSpawnEvent(*translation2));
				}
			}
			
			//info!("collide");
			let norm_c = c.unwrap().vector.normalize_or_zero();
			let norm_p1 = Vec2::new(phys1.delta_x,phys1.delta_y).normalize_or_zero();
			let norm_p2 = Vec2::new(phys2.delta_x,phys2.delta_y).normalize_or_zero();
			let norm_total = (norm_p1+norm_p2).normalize_or_zero();
			let angle_rad = norm_c.angle_between(norm_total)/2.0;
			let angle = (90.0/std::f32::consts::PI)*norm_c.angle_between(norm_total);
			//info!("angle: {}", angle);
			phys1.delta_x+=norm_c.x*LAUNCH;
			phys1.delta_y+=norm_c.y*LAUNCH;
			phys2.delta_x-=norm_c.x*LAUNCH;
			phys2.delta_y-=norm_c.y*LAUNCH;
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
			phys1.delta_x *= RESTITUTION;
			phys2.delta_x *= RESTITUTION;
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
			shape1.origin = *translation1;
			shape2.origin = *translation2;
			if !angle.is_nan() && angle.abs()!=90.0 && angle.abs()!=0.0{
				let temp_shape = collidenew::rotate(&mut shape2,angle);
				shape2.vertices = temp_shape.vertices.clone();
				//info!("player b4 trans x:{} y:{}",translation2.x.clone(),translation2.y.clone());
				transform2.rotate_local_z(angle_rad);
			}
			
			return;
		}
		else{
			//info!("no collide");
			phys1.gravity = 1.0;
			phys2.gravity = 1.0;
			return;
		}
	}		
}

fn run_collisions(//first object is colliding into second
	time: Res<Time>,
	mut ev_spawnfolder : EventWriter<FolderSpawnEvent>,
	mut obj_list: Query<(Entity, &Size, &mut Transform, &mut Physics, Option<&Player>, Option<&Recycle>, Option<&Folder>)>,
){
	let mut obj_pairs = obj_list.iter_combinations_mut();
	while let Some([(e1, object1, mut transform1, mut phys1, player1, recycle1, folder1), (e2, object2, mut transform2, mut phys2, player2, recycle2, folder2)]) = obj_pairs.fetch_next(){
		if let Some(recycle1) = recycle1{
			if let Some(player2) = player2{
				continue;
			}
		}
		if let Some(player1) = player1{
			if let Some(recycle2) = recycle2{
				continue;
			}
		}
			//info!("folder 1");
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
				//info!("collide");
				let temp1x = phys1.delta_x;
				let temp1y = phys1.delta_y;
				match c{
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
				//info!("no collide");
				phys1.gravity = 1.0;
				phys2.gravity = 1.0;
			}
		
	}
}
fn move_everything(
	time: Res<Time>,
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&mut Physics, &Size, &mut Transform, &mut Shape, Option<&Player>, Option<&Recycle>, Option<&Folder>)>,
){
	const X_ACCEL: f32 = 25.0;
	const X_MAX_VEL: f32 = 100.0;
	const GRAV: f32 = 10.0;
	const Y_ACCEL: f32 = 200.0;
	const FRICTION_SCALE: f32 = 0.75;
	for (mut phys, object, mut transform, mut shape, player, recycle, folder) in query.iter_mut(){
		let translation = &mut transform.translation;
		//accelerate in horizontal
		phys.delta_y -= GRAV * phys.gravity;
		if let Some(player)=player{
			//info!("player trans x:{} y:{}",translation.x,translation.y);
			//info!("player shape x:{} y:{}",shape.origin.x,shape.origin.y);
			//info!("y vel:{}",phys.delta_y);
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
			if let Some(recycle) = recycle{
				//info!("recy trans x:{} y:{}",translation.x,translation.y);
				//info!("folder shape x:{} y:{}",shape.origin.x,shape.origin.y);
				//info!("what");
			}
			if translation.y <= (-1.0*SCREEN_HEIGHT/2.0) +(object.size.y/2.0){
				phys.delta_y = 0.0;
			}
		}
		
		
		phys.delta_x = phys.delta_x.clamp(-X_MAX_VEL, X_MAX_VEL);
		translation.x += time.delta_seconds()*phys.delta_x;
		translation.y += time.delta_seconds()*phys.delta_y;
		
		phys.delta_x *= FRICTION_SCALE;
		*translation = inbounds(*translation, object.size);
		shape.origin = *translation;
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

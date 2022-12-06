use bevy::{
	prelude::*,
	window::PresentMode, ecs::system::EntityCommands, ecs::event::Events,
};
use iyes_loopless::prelude::*;
pub struct PhysicsPlugin;

use rand::Rng;
use bevy::math::Vec2; 
use bevy::math::Vec3;
use bevy::utils::Duration;
use bevy::asset::LoadState;
use super::collide_circle::Collision;
use super::collide_circle::collide;
use super::collidenew::Shape;
use super::collidenew::CollisionInfo;
use super::collidenew::sat;
use super::collidenew::rotate;
use super::collidenew::poly_circle_collide;
use super::Size;
use super::Stage;
use super::FolderSpawnEvent;
use super::DespawnEvent;
use super::PinballSpawner;
use super::Player;
use super::Folder;
use super::Ball;
use super::Bug;
use super::BugSpawner;
use super::Flipper;
use super::Background;
use super::RigidFolder;
use super::Border;
use super::Physics;
use super::Recycle;
use super::SCREEN_WIDTH;
use super::SCREEN_HEIGHT;
use super::GameState;
#[derive(Default)]
struct FolderOpen{
	opened: bool,
}
/*use super::collidenewer;
use super::collidenewer::ShapeNewer;
use super::collidenewer::CollisionInfoNewer;
use super::collidenewer::RB;
use super::collidenewer::poly_circle_collide;
use super::collidenewer::rotatenewer;*/

impl Plugin for PhysicsPlugin{
 	fn build(&self, app: &mut App){
 	
 	app
 		.add_fixed_timestep(
 			Duration::from_millis(17),
 			"physics_update",
 		)
 		.add_fixed_timestep(
 			Duration::from_millis(5),
 			"bug_update",
 		)

 		.add_event::<FolderSpawnEvent>()
 		.init_resource::<Events<DespawnEvent>>()
 		.insert_resource(PinballSpawner{spawned: false})
 		.insert_resource(FolderOpen{opened: false})
		.add_fixed_timestep_system("physics_update",0,move_everything.run_in_state(GameState::InGame))
		.add_fixed_timestep_system("physics_update",0,run_collisions.run_in_state(GameState::InGame))
		.add_fixed_timestep_system("physics_update",0,grounded_folder.run_in_state(GameState::InGame))
		.add_fixed_timestep_system("physics_update",0,spawn_folder.run_in_state(GameState::InGame))
		.add_fixed_timestep_system("physics_update",0,despawn.run_not_in_state(GameState::InGame))
		.add_fixed_timestep_system("physics_update",0,pinball_move.run_in_state(GameState::Pinball))
		.add_fixed_timestep_system("physics_update",0,pinball_swing.run_in_state(GameState::Pinball))
		.add_fixed_timestep_system("physics_update",0,pinball_collide.run_in_state(GameState::Pinball))
		.add_fixed_timestep_system("physics_update",0,spawn_bugs.run_in_state(GameState::Bugshoot))
		.add_fixed_timestep_system("bug_update",0,shoot_bugs.run_in_state(GameState::Bugshoot))
		//.add_fixed_timestep_system("physics_update",0,despawn_cleanup::<DespawnEvent>.run_not_in_state(GameState::InGame))
		.add_fixed_timestep_system("physics_update",0,switch_state.run_not_in_state(GameState::Rover).run_not_in_state(GameState::Email));
 	}
 }
fn despawn_cleanup<T: 'static + Send + Sync>(
	mut events: ResMut<Events<T>>,
){
	events.clear();
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
			//info!("x:{} y:{} z:{}",ev.0.x,ev.0.y,ev.0.z);
			commands.spawn()
				.insert_bundle(SpriteBundle{
				texture: asset_server.load("folder.png"),
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
					delta_omega:0.0,
					gravity:1.0,
				}).insert(Shape{
					vertices: vec![Vec3::new(-18.5,16.0,0.0),Vec3::new(18.5,16.0,0.0),Vec3::new(18.5,-16.0,0.0),Vec3::new(-18.5,-16.0,0.0)],
					origin: ev.0,//needs to be same as starting transform
				});

			//info!("spawned folder");
		}
	}
}
fn despawn(
	mut ev_despawn : EventReader<DespawnEvent>,
	mut commands: Commands,
	mut stage_change: ResMut<Stage>,
	mut folder_open: ResMut<FolderOpen>,
	despawn_query : Query<(Entity,Option<&Ball>,Option<&Background>,Option<&Bug>,Option<&Flipper>)>,
){
	
	for ev in ev_despawn.iter(){
		for (ent, ball, bg, bug, flipper) in despawn_query.iter(){
			if let Some(ball)=ball{
				commands.entity(ent).despawn();
			}
			if let Some(bg)=bg{
				commands.entity(ent).despawn();
			}
			if let Some(bug)=bug{
				commands.entity(ent).despawn();
			}
			if let Some(flipper)=flipper{
				commands.entity(ent).despawn();
			}
		}
		stage_change.val+=1;
		//info!("advance story:{}", stage_change.val);	//replace with whatever int increment thing sodi wants
		if !folder_open.opened{
			commands.insert_resource(NextState(GameState::InGame));
		}
		break;
		
	}
}
fn inbounds(trans : Vec3, size : Vec2)->Vec3{
	return Vec3{
		x : trans.x.clamp(((-1.0*SCREEN_WIDTH)/2.0)+(size.x/2.0),((1.0*SCREEN_WIDTH)/2.0)-(size.x/2.0)),
		y : trans.y.clamp(((-1.0*SCREEN_HEIGHT)/2.0)+(size.y/2.0),((1.0*SCREEN_HEIGHT)/2.0)-(size.y/2.0)),
		z : trans.z
	};
}
fn run_collisions(//first object is colliding into second
	time: Res<Time>,
	mut ev_spawnfolder : EventWriter<FolderSpawnEvent>,
	mut obj_list: Query<(&Size, &mut Transform, &mut Physics, &mut Shape, Option<&Player>, Option<&Recycle>, Option<&Folder>)>,
){
	let mut obj_pairs = obj_list.iter_combinations_mut();
	while let Some([(object1, mut transform1, mut phys1, mut shape1, player1, recycle1, folder1), (object2, mut transform2, mut phys2, mut shape2, player2, recycle2, folder2)]) = obj_pairs.fetch_next(){
		//info!("test");
		let translation1 = &mut transform1.translation;
		let translation2 = &mut transform2.translation;
		let size1 = object1.size;
		let size2 = object2.size;
		const LAUNCH: f32 = 4.0;
		const X_MAX_VEL: f32 = 100.0;
		const Y_MAX_VEL: f32 = 100.0;
		const ROTATE_LAUNCH: f32 =0.05;
		const FRAMERATE: f32 = 1.0/60.0;
		let c = sat(&*shape1,&*shape2);
		if c.is_some(){//if collision
			if let Some(player1) = player1{
				if let Some(recycle2) = recycle2{
					if phys2.gravity==0.0{
						phys2.gravity=1.0;
					}
					else{
						ev_spawnfolder.send(FolderSpawnEvent(*translation2));
					}
				}
			}
			if let Some(recycle1) = recycle1{
				if let Some(player2) = player2{
					if phys1.gravity==0.0{
						phys1.gravity=1.0;
					}
					else{
						ev_spawnfolder.send(FolderSpawnEvent(*translation1));
					}
				}
			}
			
			//info!("collide");
			let c_vec = c.unwrap().vector;
			let norm_c = c_vec.normalize_or_zero();
			let norm_p1 = Vec2::new(phys1.delta_x,phys1.delta_y).normalize_or_zero();
			let norm_p2 = Vec2::new(phys2.delta_x,phys2.delta_y).normalize_or_zero();
			let norm_total = (norm_p1+norm_p2).normalize_or_zero();
			let angle_rad = norm_c.angle_between(norm_total)/2.0;
			let angle = (90.0/std::f32::consts::PI)*norm_c.angle_between(norm_total);
			//info!("angle: {}", angle);
			phys1.delta_x+=phys2.delta_x.abs()*LAUNCH*norm_c.x;
			phys1.delta_y+=phys2.delta_y.abs()*LAUNCH*norm_c.y;
			phys2.delta_x-=phys1.delta_x.abs()*LAUNCH*norm_c.x;
			phys2.delta_y-=phys1.delta_y.abs()*LAUNCH*norm_c.y;
			translation1.x += FRAMERATE*phys1.delta_x;
			translation1.y += FRAMERATE*phys1.delta_y;
			translation2.x += FRAMERATE*phys2.delta_x;
			translation2.y += FRAMERATE*phys2.delta_y;
			
			phys1.delta_x = phys1.delta_x.clamp(-X_MAX_VEL, X_MAX_VEL);
			phys2.delta_x = phys2.delta_x.clamp(-X_MAX_VEL, X_MAX_VEL);
			phys1.delta_y = phys1.delta_y.clamp(-Y_MAX_VEL, Y_MAX_VEL);
			phys2.delta_y = phys2.delta_y.clamp(-Y_MAX_VEL, Y_MAX_VEL);	
			*translation1 = inbounds(*translation1, object1.size);
			*translation2 = inbounds(*translation2, object2.size);
			
			shape1.origin = *translation1;
			shape2.origin = *translation2;
			if !angle.is_nan() && angle.abs()!=90.0 && angle.abs()!=0.0{
				phys2.delta_omega += (phys1.delta_x.powi(2)+phys1.delta_y.powi(2)).sqrt()*angle_rad*ROTATE_LAUNCH;
				
			}
			
			
			
		}
		
	}		
}

fn grounded_folder(//first object is colliding into second
	time: Res<Time>,
	mut obj_list: Query<(Entity, &Size, &mut Transform, Option<&mut Physics>, Option<&mut Player>,  Option<&RigidFolder>)>,
){
	let mut obj_pairs = obj_list.iter_combinations_mut();
	while let Some([(e1, object1, mut transform1, mut phys1, mut player1, folder1), (e2, object2, mut transform2, mut phys2, mut player2, folder2)]) = obj_pairs.fetch_next(){
		if let Some(mut player1) = player1{
			let mut collision_check = player1.as_mut();
			if let Some(folder2) = folder2{
				let phys = phys1.as_mut().unwrap();
				//let folder = folder2.unwrap();
				let translation1 = &mut transform1.translation;
				let translation2 = &mut transform2.translation;
				let size1 = object1.size;
				let size2 = object2.size;
				let c = collide(*translation1,size1,*translation2,size2);
				if c.is_some(){
					
					match c{
						Some(Collision::Left)=>{phys.delta_x=0.0;collision_check.is_colliding_left=true;},
						Some(Collision::Right)=>{phys.delta_x=0.0;collision_check.is_colliding_right=true;},
						Some(Collision::Top)=>{phys.delta_y=0.0;phys.gravity=0.0;collision_check.is_grounded_folder=true;collision_check.folder_collide_id=folder2.state_id;collision_check.folder_collide_counter=5;},
						Some(Collision::Bottom)=>{phys.delta_y=0.0;},
						Some(Collision::Inside)=>{phys.delta_x=0.0;},
						None=>(),
					}
					//*translation1 = inbounds(*translation1, object1.size);
					
				}
				else{
					//info!("no collide");
					if collision_check.folder_collide_counter>=1{
						collision_check.folder_collide_counter-=1;
					}
					collision_check.is_colliding_left=false;
					collision_check.is_colliding_right=false;
					if collision_check.folder_collide_counter <= 0{
						collision_check.is_grounded_folder=false;
					}
					
				}
			}
		}
	}
}

fn switch_state(
	mut commands: Commands,
	keyboard: Res<Input<KeyCode>>,
	asset_server: Res<AssetServer>,
	mut ev_despawn : EventWriter<DespawnEvent>,
	mut pinball_spawner: ResMut<PinballSpawner>,
	mut folder_open: ResMut<FolderOpen>,
	state: Res<CurrentState<GameState>>,
	despawn_query: Query<(Entity,Option<&Ball>,Option<&Background>,Option<&Bug>,Option<&Flipper>)>,
	player_query: Query<(&Player)>,
){
	if let GameState::InGame=state.0{
		folder_open.opened=false;
	}
	for (player) in player_query.iter(){
		if keyboard.just_pressed(KeyCode::M) && player.is_grounded_folder{
			match player.folder_collide_id{
				0=>{
					commands.insert_resource(NextState(GameState::Pinball));
					if !pinball_spawner.spawned{
						//info!("state changed");
						let handy:Handle<Image> = asset_server.load("foosball_bg.png");
						commands.spawn().insert_bundle(SpriteBundle{
							texture: handy,
							transform: Transform::from_xyz(0.0,0.0,2.0),
							..default()
						}).insert(Background{});
						commands.spawn().insert_bundle(SpriteBundle{
							texture: asset_server.load("goal_smaller.png"),
							transform: Transform::from_xyz(592.0,100.0,3.0),
							..default()
						}).insert(Background{});
						commands.spawn()
								.insert_bundle(SpriteBundle{
								texture: asset_server.load("ball.png"),
								transform: Transform::from_xyz(-430.0,300.0,3.0),
								..default()
								}).insert(Size{
									size: Vec2{
										x:64.0,
										y:64.0,
									}
								}).insert(Ball{
									is_grounded: false,
								})
								.insert(Physics{
									delta_x:0.0,
									delta_y:0.0,
									delta_omega:0.0,
									gravity:1.0,
								});
						commands.spawn()
								.insert_bundle(SpriteBundle{
								texture: asset_server.load("flipper_new.png"),
								transform: Transform::from_xyz(-512.0,-250.0,3.0),
								..default()
								}).insert(Flipper{
									delta_omega: 0.0,
								})
								.insert(Shape{
									vertices: vec![Vec3::new(-1.0,28.0,3.0),Vec3::new(-1.0,-30.0,3.0),Vec3::new(180.0,0.0,3.0)],
									origin: Vec3::new(-512.0,-250.0,3.0),
								});
						pinball_spawner.spawned=true;
					}
				},
				1=>{
					commands.insert_resource(NextState(GameState::Jumpscare));
					//info!("state changed");
					let handy:Handle<Image> = asset_server.load("jumpscare_bg.png");
					commands.spawn().insert_bundle(SpriteBundle{
						texture: handy,
						transform: Transform::from_xyz(0.0,0.0,2.0),
						..default()
					}).insert(Background{});
					commands.spawn().insert_bundle(SpriteBundle{
						texture: asset_server.load("jumpscare.png"),
						transform: Transform::from_xyz(0.0,0.0,3.0),
						..default()
					}).insert(Background{});
				},
				2=>{
					commands.insert_resource(NextState(GameState::Bugshoot));
					let handy:Handle<Image> = asset_server.load("bug_bg.png");
					commands.spawn().insert_bundle(SpriteBundle{
						texture: handy,
						transform: Transform::from_xyz(0.0,0.0,2.0),
						..default()
					}).insert(Background{});
					commands.insert_resource(BugSpawner{
						timer: 2000,
						squished: 0,
					});
				},
				3=>{
					if !folder_open.opened{
						folder_open.opened=true;
						commands.insert_resource(NextState(GameState::Folder));
						ev_despawn.send(DespawnEvent());
					}
				},
				_=>(),
			}
		}
		if keyboard.just_pressed(KeyCode::N){
			commands.insert_resource(NextState(GameState::InGame));
			pinball_spawner.spawned=false;
			folder_open.opened=false;
			for (ent, ball, bg, bug, flipper) in despawn_query.iter(){
				if let Some(ball)=ball{
					commands.entity(ent).despawn();
				}
				if let Some(bg)=bg{
					commands.entity(ent).despawn();
				}
				if let Some(bug)=bug{
					commands.entity(ent).despawn();
				}
				if let Some(flipper)=flipper{
					commands.entity(ent).despawn();
				}
			}
			//info!("back in game");
			
		}
	}
}
fn shoot_bugs(
	mut commands: Commands,
	mut bug_query: Query<(Entity, &mut Bug, &Transform)>,
	time: Res<Time>,
	buttons: Res<Input<MouseButton>>,
	mut spawner: ResMut<BugSpawner>,
	windows: Res<Windows>,
	q_camera: Query<(&Camera, &GlobalTransform)>,
){
	let (camera, camera_transform) = q_camera.single();
	let window = windows.get_primary().unwrap();
	for (ent, mut bug, transform) in bug_query.iter_mut(){
		bug.timer=bug.timer-5;
		
		if let Some(cursor_pos) = window.cursor_position(){

			let world_pos = (cursor_pos+Vec2::new(-1.0*(SCREEN_WIDTH/2.0),-1.0*(SCREEN_HEIGHT/2.0)));
			//info!("cursor x:{} y:{}",world_pos.x,world_pos.y);
			//info!("bug x:{} y:{}",transform.translation.x,transform.translation.y);
			if f32::sqrt(f32::powi(transform.translation.x-world_pos.x,2)+f32::powi(transform.translation.y-world_pos.y,2)) <= 32.0 && buttons.just_pressed(MouseButton::Left){//if within bug range and clicked
				commands.entity(ent).despawn();
				//info!("bug squished");
				spawner.squished=spawner.squished+1;
				continue;
			}
		}
		
		if bug.timer<=0{
			commands.entity(ent).despawn();
			bug.timer = 2000;
		}
	}
}
fn spawn_bugs(
	time: Res<Time>,
	mut commands: Commands,
	mut spawner: ResMut<BugSpawner>,
	asset_server: Res<AssetServer>,
	mut ev_despawn : EventWriter<DespawnEvent>,
	despawn_query: Query<(Entity,Option<&Ball>,Option<&Background>,Option<&Bug>)>,
){
	if spawner.squished >= 10 {
		//info!("advance story");
		spawner.squished=0;
		ev_despawn.send(DespawnEvent());
	}
	spawner.timer=spawner.timer-17;
	if spawner.timer<=0{
		let mut rng = rand::thread_rng();
		commands.spawn()
				.insert_bundle(SpriteBundle{
				texture: asset_server.load("bug.png"),
				transform: Transform::from_xyz(rng.gen_range(-608..=608) as f32,rng.gen_range(-328..=328) as f32,3.0),
				..default()
				}).insert(Bug{
					timer: 2000,
				});
		spawner.timer = 2000;
	}
}
fn pinball_move(
	mut commands: Commands,
	mut pinball_spawner: ResMut<PinballSpawner>,
	mut query: Query<(&mut Physics, &Size, &mut Transform, &mut Ball)>,
	mut ev_despawn : EventWriter<DespawnEvent>,
){
	for (mut phys, object, mut transform, mut ball) in query.iter_mut(){
			const FRAMERATE: f32 = 1.0/60.0;
			const FRICTION_SCALE: f32 = 0.75;
			const GRAV: f32 = 10.0;
			phys.delta_y -= GRAV * phys.gravity;
			if transform.translation.y <= (-1.0*SCREEN_HEIGHT/2.0) +(object.size.y/2.0){
				phys.delta_y = -1.0*phys.delta_y*0.4;
				ball.is_grounded=true;
			}
			else{
				ball.is_grounded=false;
			}
			if transform.translation.y >= (1.0*SCREEN_HEIGHT/2.0) -(object.size.y/2.0){
				phys.delta_y = -1.0*phys.delta_y*0.4;
			}
			if transform.translation.x >= (1.0*SCREEN_WIDTH/2.0) -(object.size.x/2.0){
				phys.delta_x = -1.0*phys.delta_x*0.4;
			}
			if transform.translation.x <= (-1.0*SCREEN_WIDTH/2.0) +(object.size.x/2.0){
				phys.delta_x = -1.0*phys.delta_x*0.4;
			}
			//info!("player trans x:{} y:{} z:{}",transform.translation.x,transform.translation.y,transform.translation.z);
			if (transform.translation.x>=544.0 && transform.translation.x<=640.0) && (transform.translation.y>=26.0 && transform.translation.y<=174.0) && pinball_spawner.spawned{
				//info!("amogus");
				pinball_spawner.spawned=false;
				ev_despawn.send(DespawnEvent());
				
			}
			transform.translation.x += FRAMERATE*phys.delta_x;
			transform.translation.y += FRAMERATE*phys.delta_y;
			transform.rotate_local_z((std::f32::consts::PI/180.0)*phys.delta_omega*FRAMERATE);
			
			
			if ball.is_grounded{
				phys.delta_x *= FRICTION_SCALE;
				phys.delta_omega *=0.9;
			}
			transform.translation = inbounds(transform.translation, object.size);
	}
}
fn pinball_swing(
	mut query: Query<(&mut Shape, &mut Transform, &mut Flipper)>,
	keyboard_input: Res<Input<KeyCode>>,
){
	const FRAMERATE: f32 = 1.0/60.0;
	for (mut shape, mut transform, mut flipper) in query.iter_mut(){
		if keyboard_input.pressed(KeyCode::S) {
			flipper.delta_omega-=7.0;
		}
		if keyboard_input.pressed(KeyCode::W) {
			flipper.delta_omega+=7.0;
		}		
		let temp_shape = rotate(&mut shape,(std::f32::consts::PI/180.0)*1.0*flipper.delta_omega*FRAMERATE);
		shape.vertices = temp_shape.vertices.clone();
		transform.rotate_local_z((std::f32::consts::PI/180.0)*1.0*flipper.delta_omega*FRAMERATE);	
		if !keyboard_input.pressed(KeyCode::S) && !keyboard_input.pressed(KeyCode::W) {
			flipper.delta_omega *= 0.15;
		}
		//info!("rotate amt:{}",(1.0*flipper.delta_omega)*FRAMERATE);
		//info!("pain: {:?}", shape.vertices);
	}
}
fn pinball_collide(
	mut ball_query: Query<(&mut Transform, &mut Physics),With<Ball>>,
	mut flipper_query: Query<(&mut Flipper, &mut Shape)>,
){
	const FRAMERATE: f32 = 1.0/60.0;
	const LAUNCH: f32 = 7.0;
	const ROTATE_LAUNCH: f32 = 10.0;
	const LAUNCH_OTHER: f32 = 1.5;
	const X_MAX_VEL: f32 = 200.0;
	const Y_MAX_VEL: f32 = 200.0;
	for (mut ball_transform, mut phys) in ball_query.iter_mut(){
		let (mut flipper, mut shape) = flipper_query.single_mut();
		
		let temp_origin = Vec2::new(ball_transform.translation.x,ball_transform.translation.y);
		let c = poly_circle_collide(&*shape,&temp_origin,&32.0);
		if c.is_some(){//if collision
			let c_vec = c.unwrap().vector;
			let norm_c = c_vec.normalize_or_zero();
			let norm_p1 = Vec2::new(phys.delta_x,phys.delta_y).normalize_or_zero();
			//info!("omega: {}", flipper.delta_omega);
			phys.delta_x+=1.0*LAUNCH_OTHER*phys.delta_x*norm_c.x + -1.0*LAUNCH*flipper.delta_omega*norm_c.x;
			phys.delta_y+=1.0*LAUNCH_OTHER*phys.delta_y*norm_c.y + -1.0*LAUNCH*flipper.delta_omega*norm_c.y;
			phys.delta_omega+=norm_c.angle_between(Vec2::Y)*ROTATE_LAUNCH;
	
			ball_transform.translation.x += FRAMERATE*phys.delta_x;
			ball_transform.translation.y += FRAMERATE*phys.delta_y;
			
			/*phys.delta_x = phys.delta_x.clamp(-X_MAX_VEL, X_MAX_VEL);
			phys.delta_y = phys.delta_y.clamp(-Y_MAX_VEL, Y_MAX_VEL);*/

		}
	}
	
	
	
}
fn move_everything(
	time: Res<Time>,
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&mut Physics, &Size, &mut Transform, &mut Shape, Option<&mut Player>, Option<&Recycle>, Option<&Folder>, Option<&Border>)>,
){
	const FRAMERATE: f32 = 1.0/60.0;
	const X_ACCEL: f32 = 30.0;
	const X_MAX_VEL: f32 = 300.0;
	const GRAV: f32 = 10.0;
	const Y_ACCEL: f32 = 550.0;
	const FRICTION_SCALE: f32 = 0.75;
	
	for (mut phys, object, mut transform, mut shape, mut player, recycle, folder, border) in query.iter_mut(){
		if let Some(mut border)=border{
			continue;
		}
		//let translation = &mut transform.translation;
		
		//accelerate in horizontal
		
		if let Some(mut player)=player{
			//info!("player trans x:{} y:{} z:{}",translation.x,translation.y,translation.z);
			//info!("player shape x:{} y:{} z:{}",shape.origin.x,shape.origin.y,shape.origin.z);
			
			
			phys.delta_y -= GRAV * phys.gravity;
			let mut collision_check = player.as_mut();
			//info!("collision check grounded:{} folder:{}",collision_check.is_grounded, collision_check.is_grounded_folder);
			let mut jumping = 0.0;
			if !collision_check.is_grounded && !collision_check.is_grounded_folder {
				phys.gravity=1.0;
				//info!("y vel:{}",phys.delta_y);
				//info!("collision check grounded:{} folder:{}",collision_check.is_grounded, collision_check.is_grounded_folder);
			}
			if keyboard_input.pressed(KeyCode::A) && !collision_check.is_colliding_right{
				phys.delta_x-= X_ACCEL;
				//info!("left");
			}
			if keyboard_input.pressed(KeyCode::D) && !collision_check.is_colliding_left{
				phys.delta_x+= X_ACCEL;
				//info!("right");
			}
			if keyboard_input.pressed(KeyCode::Space){
				jumping = 1.0;
				//info!("jump");
			}
			if transform.translation.y <= -335.0{
				//info!("why");
				collision_check.is_grounded=true;
			}
			if collision_check.is_grounded || collision_check.is_grounded_folder{//note: need to replace this with a function that checks for grounded for all physics entities
				//info!("collision check grounded:{} folder:{}",collision_check.is_grounded, collision_check.is_grounded_folder);
				phys.delta_y = 0.0;
				if jumping==1.0{
					phys.delta_y += jumping * Y_ACCEL;
					collision_check.is_grounded=false;
					collision_check.is_grounded_folder=false;
				}
			}
			
		}
		else{
			phys.delta_y -= GRAV * phys.gravity;
			if transform.translation.y <= (-1.0*SCREEN_HEIGHT/2.0) +(object.size.y/2.0){
				phys.delta_y = 0.0;
			}
			let temp_shape = rotate(&mut shape,((90.0/std::f32::consts::PI)*phys.delta_omega*FRAMERATE)%360.0);
			shape.vertices = temp_shape.vertices.clone();
			//info!("player b4 trans x:{} y:{}",translation2.x.clone(),translation2.y.clone());
			//info!("omega: {}", phys.delta_omega);
			transform.rotate_local_z(((90.0/std::f32::consts::PI)*phys.delta_omega*FRAMERATE)%360.0);
			phys.delta_omega *= 0.75;
			
		}
		
		
		phys.delta_x = phys.delta_x.clamp(-X_MAX_VEL, X_MAX_VEL);
		transform.translation.x += FRAMERATE*phys.delta_x;
		transform.translation.y += FRAMERATE*phys.delta_y;
		
		phys.delta_x *= FRICTION_SCALE;
		transform.translation = inbounds(transform.translation, object.size);
		shape.origin = transform.translation;
		
	}
}
 

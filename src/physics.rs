use bevy::{
	prelude::*,
	window::PresentMode, ecs::system::EntityCommands,
};
use iyes_loopless::prelude::*;
pub struct PhysicsPlugin;

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
use super::Size;
use super::FolderSpawnEvent;
use super::Player;
use super::Folder;
use super::RigidFolder;
use super::Border;
use super::Physics;
use super::Recycle;
use super::SCREEN_WIDTH;
use super::SCREEN_HEIGHT;
use super::GameState;

impl Plugin for PhysicsPlugin{
 	fn build(&self, app: &mut App){
 	
 	app
 		.add_fixed_timestep(
 			Duration::from_millis(33),
 			"physics_update",
 		)
 		.add_event::<FolderSpawnEvent>()
		.add_fixed_timestep_system("physics_update",0,move_everything.run_in_state(GameState::InGame))
		.add_fixed_timestep_system("physics_update",0,run_collisions.run_in_state(GameState::InGame))
		.add_fixed_timestep_system("physics_update",0,grounded_folder.run_in_state(GameState::InGame))
		.add_fixed_timestep_system("physics_update",0,spawn_folder.run_in_state(GameState::InGame));
 	}
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
					gravity:1.0,
				}).insert(Shape{
					vertices: vec![Vec3::new(-18.5,16.0,0.0),Vec3::new(18.5,16.0,0.0),Vec3::new(18.5,-16.0,0.0),Vec3::new(-18.5,-16.0,0.0)],
					origin: ev.0,//needs to be same as starting transform
				});

			//info!("spawned folder");
		}
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
	mut obj_list: Query<(&Size, &mut Transform, &mut Physics, &mut Shape, Option<&Player>, Option<&Recycle>, Option<&Folder>, /*Option<&Border>*/)>,
){
	let mut obj_pairs = obj_list.iter_combinations_mut();
	while let Some([(object1, mut transform1, mut phys1, mut shape1, player1, recycle1, folder1), (object2, mut transform2, mut phys2, mut shape2, player2, recycle2, folder2)]) = obj_pairs.fetch_next(){
		//info!("test");
		let translation1 = &mut transform1.translation;
		let translation2 = &mut transform2.translation;
		let size1 = object1.size;
		let size2 = object2.size;
		const LAUNCH: f32 = 80.0;
		const X_MAX_VEL: f32 = 100.0;
		const Y_MAX_VEL: f32 = 100.0;
		const RESTITUTION: f32 = 0.45;
		const CUTOFF: f32 = 60.0;
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
			/*if let Some(folder1) = folder1{
				if let Some(border2) = border2{
					info!("amogus");
					let norm_c = c.unwrap().vector.normalize_or_zero();
					let norm_p1 = Vec2::new(phys1.delta_x,phys1.delta_y).normalize_or_zero();
					let norm_p2 = Vec2::new(phys2.delta_x,phys2.delta_y).normalize_or_zero();
					let norm_total = (norm_p1+norm_p2).normalize_or_zero();
					let angle_rad = norm_c.angle_between(norm_total)/2.0;
					let angle = (90.0/std::f32::consts::PI)*norm_c.angle_between(norm_total);
					//info!("angle: {}", angle);
					phys1.delta_x+=norm_c.x*LAUNCH;
					phys1.delta_y+=norm_c.y*LAUNCH;
					phys1.delta_x *= RESTITUTION;
					phys1.delta_y *= RESTITUTION;
					translation1.x += time.delta_seconds()*phys1.delta_x;
					translation1.y += time.delta_seconds()*phys1.delta_y;
					
					phys1.delta_x = phys1.delta_x.clamp(-X_MAX_VEL, X_MAX_VEL);
					phys1.delta_y = phys1.delta_y.clamp(-Y_MAX_VEL, Y_MAX_VEL);
					*translation1 = inbounds(*translation1, object1.size);
					shape1.origin = *translation1;
					if !angle.is_nan() && angle.abs()!=90.0 && angle.abs()!=0.0{
						let temp_shape = rotate(&mut shape1,angle);
						shape1.vertices = temp_shape.vertices.clone();
						//info!("player b4 trans x:{} y:{}",translation2.x.clone(),translation2.y.clone());
						transform1.rotate_local_z(angle_rad);
					}
					continue;
				}
				else{
					continue;
				}
			}
			if let Some(border1) = border1{
				if let Some(folder2) = folder2{
					info!("amogus");
					let norm_c = c.unwrap().vector.normalize_or_zero();
					let norm_p1 = Vec2::new(phys1.delta_x,phys1.delta_y).normalize_or_zero();
					let norm_p2 = Vec2::new(phys2.delta_x,phys2.delta_y).normalize_or_zero();
					let norm_total = (norm_p1+norm_p2).normalize_or_zero();
					let angle_rad = norm_c.angle_between(norm_total)/2.0;
					let angle = (90.0/std::f32::consts::PI)*norm_c.angle_between(norm_total);
					//info!("angle: {}", angle);
					phys2.delta_x+=norm_c.x*LAUNCH;
					phys2.delta_y+=norm_c.y*LAUNCH;
					phys2.delta_x *= RESTITUTION;
					phys2.delta_y *= RESTITUTION;
					translation2.x += time.delta_seconds()*phys2.delta_x;
					translation2.y += time.delta_seconds()*phys2.delta_y;
					
					phys2.delta_x = phys2.delta_x.clamp(-X_MAX_VEL, X_MAX_VEL);
					phys2.delta_y = phys2.delta_y.clamp(-Y_MAX_VEL, Y_MAX_VEL);
					*translation2 = inbounds(*translation2, object2.size);
					shape2.origin = *translation2;
					if !angle.is_nan() && angle.abs()!=90.0 && angle.abs()!=0.0{
						let temp_shape = rotate(&mut shape2,angle);
						shape2.vertices = temp_shape.vertices.clone();
						//info!("player b4 trans x:{} y:{}",translation2.x.clone(),translation2.y.clone());
						transform2.rotate_local_z(angle_rad);
					}
					continue;
				}
				
			}
			else{
				continue;
			}*/

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
				let temp_shape = rotate(&mut shape2,angle);
				shape2.vertices = temp_shape.vertices.clone();
				//info!("player b4 trans x:{} y:{}",translation2.x.clone(),translation2.y.clone());
				transform2.rotate_local_z(angle_rad);
			}
			
			return;
		}
		/*else{
			//info!("no collide");
			phys1.gravity = 1.0;
			phys2.gravity = 1.0;
			return;
		}*/
	}		
}

fn grounded_folder(//first object is colliding into second
	time: Res<Time>,
	mut ev_spawnfolder : EventWriter<FolderSpawnEvent>,
	mut obj_list: Query<(Entity, &Size, &mut Transform, Option<&mut Physics>, Option<&mut Player>,  Option<&RigidFolder>)>,
){
	let mut obj_pairs = obj_list.iter_combinations_mut();
	while let Some([(e1, object1, mut transform1, mut phys1, mut player1, folder1), (e2, object2, mut transform2, mut phys2, mut player2, folder2)]) = obj_pairs.fetch_next(){
		if let Some(mut player1) = player1{
			let mut collision_check = player1.as_mut();
			if let Some(folder2) = folder2{
				let phys = phys1.as_mut().unwrap();
				let translation1 = &mut transform1.translation;
				let translation2 = &mut transform2.translation;
				let size1 = object1.size;
				let size2 = object2.size;
				let c = collide(*translation1,size1,*translation2,size2);
				if c.is_some(){
					//info!("collide");
					match c{
						Some(Collision::Left)=>{phys.delta_x=0.0;collision_check.is_colliding_left=true;},
						Some(Collision::Right)=>{phys.delta_x=0.0;collision_check.is_colliding_right=true;},
						Some(Collision::Top)=>{phys.delta_y=0.0;phys.gravity=0.0;collision_check.is_grounded=true;},
						Some(Collision::Bottom)=>{phys.delta_y=0.0;},
						Some(Collision::Inside)=>{phys.delta_x=0.0;},
						None=>(),
					}
					*translation1 = inbounds(*translation1, object1.size);
				}
				else{
					//info!("no collide");
					collision_check.is_colliding_left=false;
					collision_check.is_colliding_right=false;
					phys.gravity = 1.0;
				}
			}
		}
	}
}
/*fn run_collisions_old(//first object is colliding into second
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
			let c = collide(*translation1,size1,*translation2,size2);
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
				/*if phys1.delta_x.abs() < CUTOFF{
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
				}*/
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
			/*else{
				//info!("no collide");
				phys1.gravity = 1.0;
				phys2.gravity = 1.0;
			}*/
		
	}
}*/
fn move_everything(
	time: Res<Time>,
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&mut Physics, &Size, &mut Transform, &mut Shape, Option<&mut Player>, Option<&Recycle>, Option<&Folder>, Option<&Border>)>,
){
	const X_ACCEL: f32 = 25.0;
	const X_MAX_VEL: f32 = 100.0;
	const GRAV: f32 = 10.0;
	const Y_ACCEL: f32 = 250.0;
	const FRICTION_SCALE: f32 = 0.75;
	for (mut phys, object, mut transform, mut shape, mut player, recycle, folder, border) in query.iter_mut(){
		if let Some(mut border)=border{
			continue;
		}
		let translation = &mut transform.translation;
		//accelerate in horizontal
		
		if let Some(mut player)=player{
			//info!("player trans x:{} y:{} z:{}",translation.x,translation.y,translation.z);
			//info!("player shape x:{} y:{} z:{}",shape.origin.x,shape.origin.y,shape.origin.z);
			//info!("y vel:{}",phys.delta_y);
			phys.delta_y -= GRAV * phys.gravity;
			let mut collision_check = player.as_mut();
			let mut jumping = 0.0;
			if !collision_check.is_grounded{
				phys.gravity=1.0;
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
			if translation.y <= -335.0{
				collision_check.is_grounded=true;
			}
			if collision_check.is_grounded{//note: need to replace this with a function that checks for grounded for all physics entities
				phys.delta_y = 0.0;
				phys.delta_y += jumping * Y_ACCEL;
				collision_check.is_grounded=false;
			}
			
		}
		else{
			phys.delta_y -= GRAV * phys.gravity;
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
 

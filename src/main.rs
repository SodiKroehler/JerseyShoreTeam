use bevy::{
	prelude::*,
	window::PresentMode, ecs::system::EntityCommands,
};
use iyes_loopless::prelude::*;
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
struct Player{
	is_grounded: bool,
	is_colliding_left: bool,
	is_colliding_right: bool,
}
#[derive(Component)]
struct Folder{}
#[derive(Component)]
struct RigidFolder{}
#[derive(Component)]
struct Border{}
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState{
	InGame,
}
//mod rover;
//use rover::RoverPlugin;
//mod ui;
//use ui::UiPlugin;
mod physics;
use physics::PhysicsPlugin;

fn main() {
	App::new()
		.insert_resource(WindowDescriptor {
			title: String::from("iExplorer"),
			width: SCREEN_WIDTH,
			height: SCREEN_HEIGHT,
			present_mode: PresentMode::Fifo,
			..default()
		})
		.add_loopless_state(GameState::InGame)
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		//.add_plugin(RoverPlugin)
		//.add_plugin(UiPlugin)
		.add_plugin(PhysicsPlugin)
		//.add_system(roll_credits)
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
	let handy:Handle<Image> = asset_server.load("windows_xd.png");
	commands.spawn().insert_bundle(SpriteBundle{
		texture: handy,
		transform: Transform::from_xyz(0.0,0.0,0.0),
		..default()
	});
	commands.spawn()//note: shape vertices start at up-left most vertex and rotate clockwise, angle is math-based (0 at +x)
		.insert_bundle(SpriteBundle{
		texture: asset_server.load("player_standin.png"),//30x50
		transform: Transform::from_xyz(0.0,0.0,1.0),
		..default()
		}).insert(Player{
			is_grounded:false,
			is_colliding_left:false,
			is_colliding_right:false,
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
			origin: Vec3::new(0.0,0.0,1.0),//needs to be same as starting transform
		});
	
	commands.spawn()
		.insert_bundle(SpriteBundle{
		texture: asset_server.load("recycle_bin1.png"),
		transform: Transform::from_xyz(-590.0,320.0,1.0),
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
			gravity:0.0,
		}).insert(Shape{
			vertices: vec![Vec3::new(-18.5,21.5,0.0),Vec3::new(18.5,21.5,0.0),Vec3::new(15.5,-21.5,0.0),Vec3::new(-15.5,-21.5,0.0)],
			origin: Vec3::new(-590.0,320.0,1.0),//needs to be same as starting transform
		});
	commands.spawn()
		.insert_bundle(SpriteBundle{
		texture: asset_server.load("folder_red.png"),
		transform: Transform::from_xyz(-350.0,140.0,1.0),
		..default()
		}).insert(RigidFolder{
		}).insert(Size{
			size: Vec2{
				x:37.0,
				y:32.0,
			}
		});
	commands.spawn()
		.insert_bundle(SpriteBundle{
		texture: asset_server.load("folder_red.png"),
		transform: Transform::from_xyz(-150.0,40.0,1.0),
		..default()
		}).insert(RigidFolder{
		}).insert(Size{
			size: Vec2{
				x:37.0,
				y:32.0,
			}
		});
	commands.spawn()
		.insert_bundle(SpriteBundle{
		texture: asset_server.load("folder_red.png"),
		transform: Transform::from_xyz(50.0,-60.0,1.0),
		..default()
		}).insert(RigidFolder{
		}).insert(Size{
			size: Vec2{
				x:37.0,
				y:32.0,
			}
		});
	commands.spawn()
		.insert_bundle(SpriteBundle{
		texture: asset_server.load("folder_red.png"),
		transform: Transform::from_xyz(200.0,-160.0,1.0),
		..default()
		}).insert(RigidFolder{
		}).insert(Size{
			size: Vec2{
				x:37.0,
				y:32.0,
			}
		});
	/*commands.spawn()//DO NOT MOVE
		.insert_bundle(SpriteBundle{
		transform: Transform::from_xyz(0.0,-385.0,1.0),
		..default()
		}).insert(Border{
		}).insert(Size{
			size: Vec2{
				x:1300.0,
				y:50.0,
			}
		}).insert(Physics{
			delta_x:0.0,
			delta_y:0.0,
			gravity:0.0,
		}).insert(Shape{
			vertices: vec![Vec3::new(-650.0,25.0,0.0),Vec3::new(650.0,25.0,0.0),Vec3::new(650.0,-25.0,0.0),Vec3::new(-650.0,-25.0,0.0)],
			origin: Vec3::new(0.0,-385.0,1.0),//needs to be same as starting transform
		});
	commands.spawn()//DO NOT MOVE
		.insert_bundle(SpriteBundle{
		transform: Transform::from_xyz(0.0,385.0,1.0),
		..default()
		}).insert(Border{
		}).insert(Size{
			size: Vec2{
				x:1300.0,
				y:50.0,
			}
		}).insert(Physics{
			delta_x:0.0,
			delta_y:0.0,
			gravity:0.0,
		}).insert(Shape{
			vertices: vec![Vec3::new(-650.0,25.0,0.0),Vec3::new(650.0,25.0,0.0),Vec3::new(650.0,-25.0,0.0),Vec3::new(-650.0,-25.0,0.0)],
			origin: Vec3::new(0.0,385.0,1.0),//needs to be same as starting transform
		});*/

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

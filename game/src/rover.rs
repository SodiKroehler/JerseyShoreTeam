use bevy::{prelude::*, ui::FocusPolicy};

pub struct RoverPlugin;

impl Plugin for RoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_rover)
        .add_system(text_input);
    }
}

fn despawn_gui(mut commands: Commands, button_query: Query<Entity, With<Button>>) {
    for ent in button_query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

fn setup_rover(mut commands: Commands, assets: Res<AssetServer>) {
    //commands.spawn_bundle(UiCameraBundle::default());
    // commands.spawn((
    //     TextBundle::from_section(
    //         "hello\nbevy!",
    //         TextStyle {
    //             font: asset_server.load("assets/Jersey.ttf"),
    //             font_size: 100.0,
    //             color: Color::WHITE,
    //         },
    //     ) 
    //     .with_text_alignment(TextAlignment::TOP_CENTER)
    //     .with_style(Style {
    //         position_type: PositionType::Absolute,
    //         position: UiRect {
    //             bottom: Val::Px(5.0),
    //             right: Val::Px(15.0),
    //             ..default()
    //         },
    //         ..default()
    //     }),
    //     ColorText,
    // ));
}   

fn text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
) {
    for ev in char_evr.iter() {
        string.push(ev.char);
    }

    if keys.just_pressed(KeyCode::Return) {
      let tokens = parser(&*string);
      for str in tokens.iter() {
        println!("{}", str);
      }
        string.clear();
    }
}


fn parser(input: &str) ->Vec<&str> {
    let mut strings = Vec::new();
    let split = input.split(" ");
    for s in split {
        strings.push(s);
    }  
    strings
}

fn stemmer(mut strings: Vec<&str>) ->Vec<&str>  {
    let mut i=0;
    let mut new_strings=Vec::new();
    let stopword = vec!["a","about","above","across","after","afterwards","again","against","all", "almost","purpose"];
    for s in strings{
         if stopword.contains(&&s)==false{
              new_strings.push(s);
         }
        i+=1;
    }
    new_strings
}

// fn spawnTextBox(
//     commands: &mut Commands,
//     text: &str,
// ){
//   //  commands.spawn(Camera2dBundle::default());
//     let width = text.len() as f32 + 2.0;
// }

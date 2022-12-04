use rand::Rng;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

use super::GameState;
use super::rover::RoverState;
use super::rover::LastChat;
use super::CONSTANTS;
use super::rover::Dict;

#[derive(Default)]
pub struct Password{
    pub val: String,
}

#[derive(Serialize, Deserialize)]
struct Deflection{
    answer: String,
    vector: Vec<u32>,
}

pub enum DeflectionType{
    NoMatch, //question not "understood"
    StageTooLow, //stage too low for answer
}

pub struct DeflectionsPlugin;

impl Plugin for DeflectionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Password {val : String::from("swiftfido3301")})
            .add_exit_system(RoverState::Listening, check_for_funny_values);                                        
    }
}

fn check_for_funny_values(mut commands: Commands,
                            mut msg: ResMut<LastChat>,
                            asset_server: Res<AssetServer>,
                            mut passw: ResMut<Password>){

    let question = msg.val.clone();

    //get parsed question
    let splits = super::rover::parser(question.clone()); 
    info!("{}", question);

    let mut rng = rand::thread_rng();

//red herring #2 - #1 is in physics
    if question == String::from("farnan is great"){
        commands.insert_resource(NextState(GameState::Paused));
        super::ui::spawn_blue_screen_of_death(commands, asset_server);
        return;
    };

//partial password matching
    let pwd_check_1: bool = splits[0].eq("is") && splits[1].eq("the") && splits[2].eq("password");
    let pwd_check_2: bool = splits[0].eq("is") && splits[splits.len()-2].eq("the") && splits[splits.len()-1].eq("password");
    let mut pwd = String::from("");

    if pwd_check_1 {pwd = question[18..question.len()].to_string();}
    if pwd_check_2 {pwd = format!("{:?}", question[1..(question.len()-12)].to_string());}

    //check for "is the password _____"
    if pwd_check_1 || pwd_check_2 {
    
        let mut starting_idx : isize = -1;        
      
        for (ix, ch) in passw.val.char_indices(){
            if ch == pwd.chars().nth(0).unwrap(){
                starting_idx = ix as isize;
            }
        }

        if starting_idx >= 0 {
            let starting_idx : usize = starting_idx as usize;
            let mut mutated_pwd = String::from("");
            mutated_pwd = passw.val[starting_idx..(starting_idx + pwd.len())].to_string();
            if pwd.eq(&mutated_pwd){
                let mut first_part = String::from("You're started on the right track!");
                if starting_idx > (passw.val.len()/2) {                
                    first_part = String::from("You're moving along the right track!");
                }
            }else {
                let curr_possibility = rng.gen::<f64>();
                if (curr_possibility * CONSTANTS::AMICABILITY) >= 1.0 {
                    passw.val = pwd;
                    //hehe
                }
                return;
            }
        }
    }

}

pub fn generate_deflection(d:DeflectionType) ->String{

    let mut rng = rand::thread_rng();

    let raw_deflect_dict: String = fs::read_to_string("./assets/deflections.json").unwrap();
    let dict = serde_json::from_str::<Dict<Deflection>>(&raw_deflect_dict).unwrap();


    match d{
        DeflectionType::NoMatch => {
            let rand_idx = rng.gen_range(3..7);
            return dict.items[rand_idx].answer.clone();
        }
        DeflectionType::StageTooLow => {
            let rand_idx = rng.gen_range(0..3);
            return dict.items[rand_idx].answer.clone();
        }
    }
}
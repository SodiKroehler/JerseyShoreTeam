use bevy::prelude::*;

pub const XP_BLUE: Color = Color::rgb(0.20784314, 0.49411765, 0.78039217); //357EC7
pub const START_GREEN: Color = Color::rgb(0.07450981, 0.36018432, 0.0627451); //135c10
pub const CLOSE_RED: Color = Color::rgb(0.6392157, 0.0, 0.0); //A30000
pub const BACKGROUND: Color = Color::rgb(0.9882353, 0.972549, 0.92941177); //FCF8ED

pub fn uicolor (c: Color) -> UiColor {
    return bevy::prelude::UiColor(c);
   // info!("XP_BLUE {:?}", Color::hex("357EC7").unwrap());
   // return Color::hex(h).expect("problem reading hex");
}

//Numerical constants
pub const H: usize = 300;
// pub const AMICABILITY: f64 = 100.0;
pub const COS_DIST_THRESHOLD: f64 = 0.72;

//z-offsets
pub const Z_UI:f32 = 10.0;
pub const Z_EXTRAFOLDER:f32 = 20.0;
pub const Z_PAUSE:f32 = 120.0;
pub const Z_PANIC:f32 = 320.0;

//assets
pub const FONT_FILE: &str =  "Segoe UI.ttf";
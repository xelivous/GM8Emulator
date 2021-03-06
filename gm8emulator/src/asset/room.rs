use crate::{
    game::{string::RCStr, Background, View},
    gml::runtime::Instruction,
    tile::Tile,
};
use serde::{Deserialize, Serialize};
use shared::types::{Colour, ID};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct Room {
    pub name: RCStr,
    pub caption: RCStr,
    pub width: u32,
    pub height: u32,
    pub speed: u32,
    pub persistent: bool,
    pub bg_colour: Colour,
    pub clear_screen: bool,
    pub creation_code: Rc<[Instruction]>,

    pub backgrounds: Vec<Background>,
    pub views_enabled: bool,
    pub views: Vec<View>,
    pub instances: Vec<Instance>,
    pub tiles: Vec<Tile>,
}

/// An instance stored in a Room
#[derive(Clone, Serialize, Deserialize)]
pub struct Instance {
    pub x: i32,
    pub y: i32,
    pub object: i32,
    pub id: ID,
    pub creation: Rc<[Instruction]>,
}

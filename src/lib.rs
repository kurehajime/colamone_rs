use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::rule::MapArray;

pub mod ai;
pub mod eval;
pub mod rule;

#[wasm_bindgen]
pub struct Result {
    pub from: usize,
    pub to: usize,
}

pub type MapArray2 = Vec<isize>;

#[derive(Serialize, Deserialize)]
pub struct Arg {
    pub map: MapArray,
    pub turn_player: isize,
    pub depth: isize,
}

#[wasm_bindgen]
pub fn think_ai(arg: JsValue) -> Result {
    let arg: Arg = serde_wasm_bindgen::from_value(arg).unwrap();
    let result = ai::think_ai(&arg.map, arg.turn_player, arg.depth, None, None, None);
    Result {
        from: result.0.unwrap().0,
        to: result.0.unwrap().1,
    }
}

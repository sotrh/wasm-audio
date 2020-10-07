use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, OscillatorType};

pub fn midi_to_freq(note: u8) -> f32 {
    27.5 * 2f32.powf((note as f32 - 21.0) / 12.0)
}

#[wasm_bindgen]
pub struct FmOsc {
    context: AudioContext,
    primary: web_sys::OscillatorNode,
    gain: web_sys::GainNode,
    fm_gain: web_sys::GainNode,
    fm_osc: web_sys::OscillatorNode,
    fm_freq_ratio: f32,
    fm_gain_ratio: f32,
}
use pyo3::prelude::*;

use crate::channel_wrapper::{wrap_pyxel_channel, Channel};
use crate::instance;
use crate::music_wrapper::{wrap_pyxel_music, Music};
use crate::sound_wrapper::{wrap_pyxel_sound, Sound};

#[pyfunction]
fn channel(ch: u32) -> Channel {
    wrap_pyxel_channel(instance().channel(ch))
}

#[pyfunction]
fn sound(snd: u32) -> Sound {
    wrap_pyxel_sound(instance().sound(snd))
}

#[pyfunction]
fn music(msc: u32) -> Music {
    wrap_pyxel_music(instance().music(msc))
}

#[pyfunction]
fn play_pos(ch: u32) -> Option<(u32, u32)> {
    instance().play_pos(ch)
}

#[pyfunction]
#[pyo3(text_signature = "(ch, snd, *, tick, loop)")]
fn play(ch: u32, snd: &PyAny, tick: Option<u32>, r#loop: Option<bool>) -> PyResult<()> {
    type_switch! {
        snd,
        u32, {
            instance().play1(ch, snd, tick, r#loop.unwrap_or(false));
        },
        Vec<u32>, {
            instance().play(ch, &snd, tick, r#loop.unwrap_or(false));
        },
        Sound, {
            instance().channel(ch).lock().play1(snd.pyxel_sound, tick, r#loop.unwrap_or(false));
        },
        Vec<Sound>, {
            let sounds = snd.iter().map(|snd| snd.pyxel_sound.clone()).collect();

            instance().channel(ch).lock().play(sounds, tick, r#loop.unwrap_or(false));
        }
    }
    Ok(())
}

#[pyfunction]
#[pyo3(text_signature = "(msc, *, tick, loop)")]
fn playm(msc: u32, tick: Option<u32>, r#loop: Option<bool>) {
    instance().playm(msc, tick, r#loop.unwrap_or(false));
}

#[pyfunction]
fn stop(ch: Option<u32>) {
    ch.map_or_else(|| instance().stop0(), |ch| instance().stop(ch));
}

pub fn add_audio_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(channel, m)?)?;
    m.add_function(wrap_pyfunction!(sound, m)?)?;
    m.add_function(wrap_pyfunction!(music, m)?)?;
    m.add_function(wrap_pyfunction!(play_pos, m)?)?;
    m.add_function(wrap_pyfunction!(play, m)?)?;
    m.add_function(wrap_pyfunction!(playm, m)?)?;
    m.add_function(wrap_pyfunction!(stop, m)?)?;
    Ok(())
}

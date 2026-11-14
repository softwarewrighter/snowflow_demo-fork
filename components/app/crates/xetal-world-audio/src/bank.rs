use crate::{Kind, samples};
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioContext, GainNode};

pub(crate) struct Bank {
    pub context: AudioContext,
    pub step: AudioBuffer,
    pub land: AudioBuffer,
}
impl Bank {
    pub fn new() -> Result<Self, String> {
        let context = AudioContext::new().map_err(|e| format!("{e:?}"))?;
        let step = buffer(&context, Kind::Step)?;
        let land = buffer(&context, Kind::Land)?;
        Ok(Self {
            context,
            step,
            land,
        })
    }
    pub fn voice(&self, kind: Kind, rate: f32, volume: f32) -> Result<GainNode, String> {
        let source = self
            .context
            .create_buffer_source()
            .map_err(|e| format!("{e:?}"))?;
        source.set_buffer(Some(if kind == Kind::Step {
            &self.step
        } else {
            &self.land
        }));
        source.playback_rate().set_value(rate);
        let gain = self.context.create_gain().map_err(|e| format!("{e:?}"))?;
        gain.gain().set_value(volume);
        source
            .connect_with_audio_node(&gain)
            .map_err(|e| format!("{e:?}"))?;
        gain.connect_with_audio_node(&self.context.destination())
            .map_err(|e| format!("{e:?}"))?;
        source.start().map_err(|e| format!("{e:?}"))?;
        cleanup(source, gain.clone());
        Ok(gain)
    }
}

fn buffer(context: &AudioContext, kind: Kind) -> Result<AudioBuffer, String> {
    let data = samples(kind);
    let buffer = context
        .create_buffer(1, data.len() as u32, 44100.0)
        .map_err(|e| format!("{e:?}"))?;
    buffer
        .copy_to_channel(&data, 0)
        .map_err(|e| format!("{e:?}"))?;
    Ok(buffer)
}

fn cleanup(source: AudioBufferSourceNode, gain: GainNode) {
    let ended_source: web_sys::AudioScheduledSourceNode = source.clone().unchecked_into();
    let ended = Closure::once_into_js(move || {
        let _ = ended_source.disconnect();
        let _ = gain.disconnect();
        ended_source.set_onended(None);
    });
    let scheduled: web_sys::AudioScheduledSourceNode = source.unchecked_into();
    scheduled.set_onended(Some(ended.unchecked_ref()));
}

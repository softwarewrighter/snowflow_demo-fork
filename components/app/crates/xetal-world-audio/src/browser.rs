use crate::{Kind, bank::Bank};
use web_sys::{AudioContextState, GainNode};

#[derive(Default)]
pub struct Sound {
    bank: Option<Bank>,
    active: Option<(Kind, GainNode)>,
    pub enabled: bool,
}

impl Sound {
    pub fn enable(&mut self) -> Result<js_sys::Promise, String> {
        if self.bank.is_none() {
            self.bank = Some(Bank::new()?);
        }
        self.enabled = true;
        self.bank
            .as_ref()
            .ok_or("audio bank unavailable")?
            .context
            .resume()
            .map_err(|e| format!("{e:?}"))
    }

    pub fn state(&self) -> &'static str {
        if !self.enabled {
            return "muted";
        }
        if self
            .bank
            .as_ref()
            .is_some_and(|bank| bank.context.state() == AudioContextState::Running)
        {
            "running"
        } else {
            "suspended"
        }
    }

    pub fn play(&mut self, kind: Kind, volume: f32, rate: f32) -> Result<(), String> {
        if self.state() != "running" {
            return Ok(());
        }
        let bank = self.bank.as_ref().ok_or("audio bank unavailable")?;
        let gain = bank.voice(kind, rate.clamp(0.1,4.0), volume.clamp(0.0,1.0))?;
        self.active = Some((kind, gain));
        Ok(())
    }

    pub fn mute(&mut self) {
        self.enabled = false;
        if let Some((_, gain)) = self.active.take() {
            gain.gain().set_value(0.0);
        }
    }

    pub fn quiet_steps(&mut self) {
        if self
            .active
            .as_ref()
            .is_some_and(|(kind, _)| *kind == Kind::Step)
            && let Some((_, gain)) = self.active.take()
            && let Some(bank) = &self.bank
        {
            let _ = gain
                .gain()
                .set_target_at_time(0.0, bank.context.current_time(), 0.005);
        }
    }
}

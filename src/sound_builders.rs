use fundsp::hacker::{adsr_live, clamp01, envelope2, moog_q, xerp, AudioUnit64, Net64};

use crate::SharedMidiState;

// The var_fn() version looks better code-wise, but sounds a little worse - a little clipped.
pub fn simple_sound(state: &SharedMidiState, synth: Box<dyn AudioUnit64>) -> Box<dyn AudioUnit64> {
    //let control = state.control_shared();
    let control = state.control_var();
    state.assemble_sound(
        synth,
        Box::new(control >> envelope2(move |_, n| clamp01(n))),
        //Box::new(var_fn(control, clamp01))
    )
}

#[derive(Copy, Clone, Debug)]
pub struct Adsr {
    pub attack: f64,
    pub decay: f64,
    pub sustain: f64,
    pub release: f64,
}

impl Adsr {
    pub fn boxed(&self, state: &SharedMidiState) -> Box<dyn AudioUnit64> {
        let control = state.control_var();
        Box::new(control >> adsr_live(self.attack, self.decay, self.sustain, self.release))
    }

    pub fn net64ed(&self, state: &SharedMidiState) -> Net64 {
        Net64::wrap(self.boxed(state))
    }

    pub fn timed_sound(&self, timed_sound: Box<dyn AudioUnit64>, state: &SharedMidiState) -> Net64 {
        Net64::pipe_op(
            Net64::stack_op(state.bent_pitch(), self.net64ed(state)),
            Net64::wrap(timed_sound),
        )
    }
}

// It works, but I'm trying to avoid macros.
#[allow(unused)]
macro_rules! op {
    ($fn:expr) => {
        envelope2(move |_, n| $fn(n))
    };
}

pub fn adsr_timed_moog(
    state: &SharedMidiState,
    source: Box<dyn AudioUnit64>,
    adsr: Adsr,
) -> Box<dyn AudioUnit64> {
    Box::new(Net64::pipe_op(
        Net64::stack_op(
            Net64::wrap(source),
            Net64::pipe_op(
                adsr.net64ed(state),
                Net64::wrap(Box::new(envelope2(move |_, n| xerp(1100.0, 11000.0, n)))),
            ),
        ),
        Net64::wrap(Box::new(moog_q(0.6))),
    ))
}

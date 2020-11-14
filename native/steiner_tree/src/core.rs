use rustler::Encoder;

use crate::atoms;
use crate::state::State;

pub(crate) enum Ret {
    Ok(Vec<(usize, usize)>),
    Error(Error),
    /// Aborting the computation. state is mutated. The caller should save state for later invocations.
    Yielding,
}

pub(crate) enum Error {
    TooLargeInput(usize),
    InvalidArg(usize, Vec<(usize, usize)>),
}

impl Encoder for Error {
    fn encode<'a>(&self, env: rustler::Env<'a>) -> rustler::Term<'a> {
        match self {
            Error::TooLargeInput(n) => (atoms::too_large_input(), n).encode(env),
            Error::InvalidArg(n, edges) => (atoms::invalid_arg(), (n, edges)).encode(env),
        }
    }
}

pub(crate) fn compute(state: &mut State) -> Ret {
    if state.rem > 0 {
        state.rem -= 1;
        Ret::Yielding
    } else {
        Ret::Error(Error::InvalidArg(state.n, state.edges.clone()))
    }
}

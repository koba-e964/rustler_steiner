use rustler::Encoder;

use crate::atoms;

pub(crate) enum Ret {
    Ok(Vec<(usize, usize)>),
    Error(Error),
    Yielding(State),
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

pub(crate) struct State {
    n: usize,
    edges: Vec<(usize, usize)>,
    rem: usize,
}

impl State {
    pub(crate) fn new(n: usize, edges: Vec<(usize, usize)>) -> State {
        Self { n, edges, rem: 1 }
    }
}

pub(crate) fn compute(mut state: State) -> Ret {
    if state.rem > 0 {
        state.rem -= 1;
        Ret::Yielding(state)
    } else {
        Ret::Error(Error::InvalidArg(state.n, state.edges))
    }
}
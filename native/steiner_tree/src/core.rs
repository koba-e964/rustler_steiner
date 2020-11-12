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
}

impl State {
    pub(crate) fn new(n: usize, edges: Vec<(usize, usize)>) -> State {
        Self { n, edges }
    }
}

pub(crate) fn compute(state: State) -> Ret {
    Ret::Error(Error::InvalidArg(state.n, state.edges))
}

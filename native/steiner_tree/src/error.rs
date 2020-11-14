use crate::atoms;
use rustler::Encoder;

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

use crate::atoms;
use rustler::Encoder;

pub(crate) enum Error {
    TooLargeInput(usize),
    InvalidArg(usize, Vec<(usize, usize)>, Vec<usize>),
    TerminalNotConnected,
}

impl Encoder for Error {
    fn encode<'a>(&self, env: rustler::Env<'a>) -> rustler::Term<'a> {
        match self {
            Error::TooLargeInput(n) => (atoms::too_large_input(), n).encode(env),
            Error::InvalidArg(n, edges, terms) => {
                (atoms::invalid_arg(), (n, edges, terms)).encode(env)
            }
            Error::TerminalNotConnected => atoms::terminal_not_connected().encode(env),
        }
    }
}

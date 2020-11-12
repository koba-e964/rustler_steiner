mod core;

use crate::core::{compute, Ret, State};
use rustler::{Encoder, Env, Error, Term};

mod atoms {
    rustler::rustler_atoms! {
        atom ok;
        atom error;
        atom too_large_input;
        atom invalid_arg;
    }
}

rustler::rustler_export_nifs! {
    "Elixir.SteinerTree",
    [
        ("compute", 2, steiner_tree),
    ],
    None
}

/// Computes a Steiner tree of a given graph.
fn steiner_tree<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let n: usize = args[0].decode()?;
    let edges: Vec<(usize, usize)> = args[1].decode()?;

    let state = State::new(n, edges);
    let result = compute(state);

    match result {
        Ret::Ok(result) => Ok((atoms::ok(), result).encode(env)),
        Ret::Error(e) => Ok((atoms::error(), e).encode(env)),
        Ret::Yielding(state) => todo!(),
    }
}

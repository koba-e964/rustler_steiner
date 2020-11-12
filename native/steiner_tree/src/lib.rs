mod core;

use rustler::codegen_runtime::{NifReturned, NIF_ENV, NIF_TERM};
use rustler::{Encoder, Env, Error, SchedulerFlags, Term};
use std::ffi::CString;

use crate::core::{compute, Ret, State};

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

unsafe extern "C" fn steiner_tree_interrupted(env: NIF_ENV, _: i32, _: *const NIF_TERM) -> NIF_TERM {
    rustler_sys::enif_make_int(env, 42)
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
        Ret::Yielding(state) => unsafe {
            let result = NifReturned::Reschedule {
                fun_name: CString::new("steiner_tree_interrupted").unwrap(),
                flags: SchedulerFlags::DirtyCpu,
                fun: steiner_tree_interrupted,
                args: vec![],
            };
            Ok(result.apply(env).encode(env))
        },
    }
}

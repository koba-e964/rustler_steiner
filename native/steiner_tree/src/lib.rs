mod core;
mod error;
mod state;

use rustler::codegen_runtime::{NifReturned, NIF_ENV, NIF_TERM};
use rustler::{Encoder, Env, Error, SchedulerFlags, Term};
use std::ffi::CString;

use crate::core::{compute, Ret};
use crate::state::{
    create_state, decode_state_ptr_from_NIF_TERM, destroy_state, encode_state_ptr_as_NIF_TERM,
    State,
};

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

unsafe extern "C" fn steiner_tree_interrupted(
    env: NIF_ENV,
    argc: i32,
    argv: *const NIF_TERM,
) -> NIF_TERM {
    assert_eq!(argc, 1);
    let term = *argv;
    let ptr = decode_state_ptr_from_NIF_TERM(term);
    let result = steiner_tree_yielding(Env::new(&term, env), ptr);
    result.as_c_arg()
}

/// Computes a Steiner tree of a given graph.
fn steiner_tree<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let n: usize = args[0].decode()?;
    let edges: Vec<(usize, usize)> = args[1].decode()?;

    let ptr = unsafe { create_state(n, edges) };

    Ok(steiner_tree_yielding(env, ptr))
}

fn steiner_tree_yielding(env: Env<'_>, ptr: *mut State) -> Term<'_> {
    let result;
    unsafe {
        let state = &mut *ptr;
        result = compute(state);
    }

    match result {
        Ret::Ok(result) => {
            // destroy the state
            unsafe {
                destroy_state(ptr);
            }
            (atoms::ok(), result).encode(env)
        }
        Ret::Error(e) => {
            // destroy the state
            unsafe {
                destroy_state(ptr);
            }
            (atoms::error(), e).encode(env)
        }
        Ret::Yielding => {
            let encoded = encode_state_ptr_as_NIF_TERM(ptr);
            let result = NifReturned::Reschedule {
                fun_name: CString::new("steiner_tree_interrupted").unwrap(),
                flags: SchedulerFlags::Normal,
                fun: steiner_tree_interrupted,
                args: vec![encoded],
            };
            // NOTE: result.apply(env).encode(env) won't work here:
            // NIF_TERM is just an alias of usize, so it would return the Erlang representation of an integer 0.
            unsafe { Term::new(env, result.apply(env)) }
        }
    }
}

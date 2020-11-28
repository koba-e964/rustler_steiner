mod core;
mod error;
mod state;
mod subsets;

use rustler::codegen_runtime::{NifReturned, NIF_ENV, NIF_TERM};
use rustler::{Encoder, Env, Error, SchedulerFlags, Term};
use std::ffi::CString;
use std::time::Instant;

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
        atom terminal_not_connected;
    }
}

rustler::rustler_export_nifs! {
    "Elixir.SteinerTree",
    [
        ("compute", 3, steiner_tree),
        ("compute_nonyielding", 3, steiner_tree_nonyielding),
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
    let terms: Vec<usize> = args[2].decode()?;

    let ptr = if let Some(ptr) = create_state(n, edges, terms) {
        ptr
    } else {
        return Err(Error::RaiseAtom("bad_alloc"));
    };

    Ok(unsafe { steiner_tree_yielding(env, ptr) })
}

/// Computes a Steiner tree of a given graph. This version will never yield.
fn steiner_tree_nonyielding<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let n: usize = args[0].decode()?;
    let edges: Vec<(usize, usize)> = args[1].decode()?;
    let terms: Vec<usize> = args[2].decode()?;

    let mut state = State::new(n, edges, terms);

    let result = loop {
        match compute(&mut state) {
            Ret::Ok(ans, picked_edges) => {
                // destroy the state
                break (atoms::ok(), (ans, picked_edges)).encode(env);
            }
            Ret::Error(e) => {
                // destroy the state
                break (atoms::error(), e).encode(env);
            }
            Ret::Yielding => {}
        }
    };
    Ok(result)
}

/// # Safety
/// - ptr should be the pointer created by create_state
/// - ptr must point to the valid State
unsafe fn steiner_tree_yielding(env: Env<'_>, ptr: *mut State) -> Term<'_> {
    let start = Instant::now();
    let result;
    {
        let state = &mut *ptr;
        result = compute(state);
    }

    match result {
        Ret::Ok(ans, picked_edges) => {
            // destroy the state
            destroy_state(ptr);
            (atoms::ok(), (ans, picked_edges)).encode(env)
        }
        Ret::Error(e) => {
            // destroy the state
            destroy_state(ptr);
            (atoms::error(), e).encode(env)
        }
        Ret::Yielding => {
            let elapsed = start.elapsed();
            // We are given 1ms timeslice. This value is hardcoded.
            let elapsed_micro = elapsed.as_micros().min(1000000000) as i32;
            let percentage = (elapsed_micro / 10).max(1).min(100);
            let should_yield = rustler::schedule::consume_timeslice(env, percentage);
            if should_yield {
                let encoded = encode_state_ptr_as_NIF_TERM(ptr);
                let result = NifReturned::Reschedule {
                    fun_name: CString::new("steiner_tree_interrupted").unwrap(),
                    flags: SchedulerFlags::Normal,
                    fun: steiner_tree_interrupted,
                    args: vec![encoded],
                };
                // NOTE: result.apply(env).encode(env) won't work here:
                // NIF_TERM is just an alias of usize, so it would return the Erlang representation of an integer 0.
                Term::new(env, result.apply(env))
            } else {
                steiner_tree_yielding(env, ptr)
            }
        }
    }
}

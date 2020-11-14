use crate::error::Error;
use crate::state::State;

pub(crate) enum Ret {
    Ok(Vec<(usize, usize)>),
    Error(Error),
    /// Aborting the computation. state is mutated. The caller should save state for later invocations.
    Yielding,
}

pub(crate) fn compute(state: &mut State) -> Ret {
    if state.rem > 0 {
        state.rem -= 1;
        Ret::Yielding
    } else {
        Ret::Error(Error::InvalidArg(state.n, state.edges.clone()))
    }
}

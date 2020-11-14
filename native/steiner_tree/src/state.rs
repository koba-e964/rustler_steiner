use rustler::codegen_runtime::NIF_TERM;

#[derive(Debug)]
pub(crate) struct State {
    pub n: usize,
    pub edges: Vec<(usize, usize)>,
    pub rem: usize,
}

impl State {
    pub(crate) fn new(n: usize, edges: Vec<(usize, usize)>) -> State {
        Self { n, edges, rem: 1 }
    }
}

#[allow(non_snake_case)]
pub(crate) fn encode_state_ptr_as_NIF_TERM(ptr: *mut State) -> NIF_TERM {
    // TODO: better representation using resource.
    // We need to register the resource type we use in load/0 beforehand.
    ptr as usize
}

/// # Safety
/// This functions is currently always safe. It is marked unsafe just in case.
#[allow(non_snake_case)]
pub(crate) unsafe fn decode_state_ptr_from_NIF_TERM(term: NIF_TERM) -> *mut State {
    // TODO: better representation using resource.
    term as _
}

/// Creates a state using NIF functions provided by Erlang.
/// Returns None if allocation failed.
pub(crate) fn create_state(n: usize, edges: Vec<(usize, usize)>) -> Option<*mut State> {
    let state = State::new(n, edges);
    // Allocate space for State
    // TODO: better representation using resource.
    let ptr: *mut State;
    unsafe {
        ptr = rustler_sys::enif_alloc(std::mem::size_of::<State>()) as _;
        if ptr.is_null() {
            return None;
        }
        std::ptr::write(ptr, state);
    }
    Some(ptr)
}

/// # Safety
/// - ptr should be the pointer created by create_state
/// - ptr must point to the valid State
pub(crate) unsafe fn destroy_state(ptr: *mut State) {
    std::ptr::drop_in_place(ptr);
    rustler_sys::enif_free(ptr as *mut ::core::ffi::c_void)
}

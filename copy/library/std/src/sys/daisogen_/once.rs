// Very simple Once<> implementation based on the Unix one, available at
// src/sys_common/once/futex.rs

use super::daisogen::futex;
use crate::cell::Cell;
use crate::sync as public;
use crate::sync::atomic::{
    AtomicUsize,
    Ordering::{Acquire, Release},
};
use crate::sync::once::ExclusiveState;

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    // No initialization has run yet
    Incomplete,
    // A thread has attempted to initialize, but it panicked
    Poisoned,
    // Some thread is attempting to initialize
    Running,
    // Initialization has completed
    Complete,
}

impl From<usize> for State {
    fn from(value: usize) -> Self {
        match value {
            0 => State::Incomplete,
            1 => State::Poisoned,
            2 => State::Running,
            3 => State::Complete,
            _ => panic!("Invalid State value: {}", value),
        }
    }
}

pub struct OnceState {
    poisoned: bool,
    set_state_to: Cell<State>,
}

impl OnceState {
    #[inline]
    pub fn is_poisoned(&self) -> bool {
        self.poisoned
    }

    #[inline]
    pub fn poison(&self) {
        self.set_state_to.set(State::Poisoned)
    }
}

// ---

struct CompletionGuard<'a> {
    state: &'a AtomicUsize,
    set_state_on_drop_to: State,
}

impl<'a> Drop for CompletionGuard<'a> {
    fn drop(&mut self) {
        self.state.swap(self.set_state_on_drop_to as usize, Release);
        futex::wake_one(&self.state);
    }
}

// ---

pub struct Once {
    state: AtomicUsize,
}

impl Once {
    #[inline]
    #[rustc_const_stable(feature = "const_once_new", since = "1.32.0")]
    pub const fn new() -> Once {
        Once {
            state: AtomicUsize::new(State::Incomplete as usize),
        }
    }

    #[inline]
    pub fn is_completed(&self) -> bool {
        self.state.load(Acquire) == State::Complete as usize
    }

    #[inline]
    pub(crate) fn state(&mut self) -> ExclusiveState {
        match State::from(self.state.load(Acquire)) {
            State::Incomplete => ExclusiveState::Incomplete,
            State::Poisoned => ExclusiveState::Poisoned,
            State::Complete => ExclusiveState::Complete,
            _ => unreachable!("invalid Once state"),
        }
    }

    #[cold]
    #[track_caller]
    pub fn call(&self, ignore_poisoning: bool, f: &mut impl FnMut(&public::OnceState)) {
        let mut state = self.state.load(Acquire);
        loop {
            match State::from(state) {
                State::Poisoned if !ignore_poisoning => {
                    // Panic to propagate the poison
                    panic!("Once instance has previously been poisoned");
                }
                State::Incomplete | State::Poisoned => {
                    // Try to register the current thread as the one running
                    if let Err(new) = self.state.compare_exchange_weak(
                        state,
                        State::Running as usize,
                        Acquire,
                        Acquire,
                    ) {
                        state = new;
                        continue;
                    }

                    // `guard` will set the new state on drop.
                    let mut guard = CompletionGuard {
                        state: &self.state,
                        set_state_on_drop_to: State::Poisoned,
                    };

                    // Run the function, letting it know if we're poisoned or not.
                    let f_state = public::OnceState {
                        inner: OnceState {
                            poisoned: state == State::Poisoned as usize,
                            set_state_to: Cell::new(State::Complete),
                        },
                    };
                    f(&f_state);
                    guard.set_state_on_drop_to = f_state.inner.set_state_to.get();
                    return;
                }
                State::Running => {
                    // Go to bed
                    futex::wait(&self.state, State::Running as usize);
                    state = self.state.load(Acquire);
                }
                State::Complete => return,
            }
        }
    }
}

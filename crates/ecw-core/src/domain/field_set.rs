use std::collections::HashMap;
use std::hash::Hash;

use super::{Interval, Measure, Measured};

use super::field_state::FieldState;

static EMPTY: FieldState = FieldState::Empty;

/// The fields of one form, parsed once: the state of each, how many are valid, whether any
/// has an error. `K` is the calculator's own key type (an enum of its fields).
pub struct FieldSet<K> {
    states: HashMap<K, FieldState>,
    valid_count: usize,
    has_error: bool,
}

impl<K: Copy + Eq + Hash> FieldSet<K> {
    /// Parses every key of `keys` with `parse` and keeps the results.
    pub fn collect(keys: &[K], mut parse: impl FnMut(K) -> FieldState) -> Self {
        let mut states = HashMap::with_capacity(keys.len());
        let mut valid_count = 0;
        let mut has_error = false;

        for &key in keys {
            let state = parse(key);
            valid_count += usize::from(state.is_ok());
            has_error |= state.has_error();
            states.insert(key, state);
        }

        Self {
            states,
            valid_count,
            has_error,
        }
    }

    /// How many fields hold a valid value.
    pub fn valid_count(&self) -> usize {
        self.valid_count
    }

    /// True if at least one field could not be read.
    pub fn has_error(&self) -> bool {
        self.has_error
    }

    /// The state of a field; a key that was not collected reads as empty.
    pub fn state(&self, key: K) -> &FieldState {
        self.states.get(&key).unwrap_or(&EMPTY)
    }

    /// True if the field holds a valid value.
    pub fn is_ok(&self, key: K) -> bool {
        self.state(key).is_ok()
    }

    /// The parsed value of a field, if it has one.
    pub fn measure(&self, key: K) -> Option<Measure> {
        self.state(key).measure()
    }

    /// The value of a field as an interval of a physical type.
    pub fn interval<T: Measured>(&self, key: K) -> Option<Interval<T>> {
        self.measure(key).map(|m| m.to_interval())
    }

    /// The keys of `keys` that do not hold a valid value yet.
    pub fn unfilled(&self, keys: &[K]) -> Vec<K> {
        keys.iter()
            .copied()
            .filter(|&key| !self.is_ok(key))
            .collect()
    }
}

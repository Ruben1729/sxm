// src/runner.rs
use crate::traits::XMachine;

pub struct MachineRunner<M: XMachine> {
    pub state: M::State,
    pub store: M::Store,
}

impl<M: XMachine> MachineRunner<M> {
    pub fn new() -> Self {
        Self {
            state: M::initial_state(),
            store: M::initial_store(),
        }
    }

    /// Tries to step the machine by finding a valid Phi for the input.
    pub fn step(&mut self, input: M::Input) -> Result<Option<M::Output>, &'static str> {
        // 1. Get all allowed functions for the current state
        let possible_phis = M::get_available_phi(self.state);

        // 2. Try them one by one (Priority based on order in the list)
        for &phi in possible_phis {
            // We pass input by reference so we can reuse it for the next check if this fails
            match M::execute_phi(phi, &mut self.store, &input) {
                Ok(output) => {
                    // 3. Success! Calculate next state
                    let next = M::next_state(self.state, phi);
                    self.state = next;
                    return Ok(output);
                }
                Err(_) => {
                    // Guard failed, continue to next phi...
                    continue;
                }
            }
        }

        // If we get here, no transition was valid for this input (Machine halts/rejects)
        Err("No valid transition found for input")
    }
}
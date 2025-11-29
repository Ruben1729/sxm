// src/traits.rs

/// The core X-Machine Trait.
///
/// In theory, an X-Machine is M = (Sigma, Gamma, Q, M, Phi, F, m0, q0).
/// This trait maps those greek letters to Rust types.
pub trait XMachine {
    /// Sigma (Σ): The Input Alphabet.
    /// What flows into the machine? (e.g., u8, chars, Events)
    type Input;

    /// Gamma (Γ): The Output Alphabet.
    /// What does the machine produce?
    type Output;

    /// Q: The State Set.
    /// Usually an Enum (e.g., State::Idle, State::Processing)
    type State: Copy + Clone + PartialEq + core::fmt::Debug;

    /// M: The Memory (Store).
    /// The data structure holding internal variables.
    type Store;

    /// The Identifier for a Processing Function (Phi).
    /// usually an enum like `Phi::Increment`, `Phi::Reset`.
    type Phi: Copy + Clone + PartialEq + core::fmt::Debug + 'static;

    /// q0: Initial State
    fn initial_state() -> Self::State;

    /// m0: Initial Memory
    fn initial_store() -> Self::Store;

    /// Returns ALL possible states in the machine.
    /// Necessary for Graphviz and Complete Test Coverage.
    fn all_states() -> &'static [Self::State];

    /// 1. The Topology
    /// Returns the list of allowed functions (arcs) from the current state.
    /// Used by both the Runner (to decide what to do) and the Graph Generator.
    fn get_available_phi(state: Self::State) -> &'static [Self::Phi];

    /// 2. The Next State Function (F)
    /// "If I was in `state` and successfully executed `phi`, where am I now?"
    fn next_state(state: Self::State, phi: Self::Phi) -> Self::State;

    /// 3. The Processing Logic
    /// Attempts to execute the function `phi` with the current data.
    ///
    /// Returns:
    /// - Ok(Some(out)): Guard passed, Store updated, Output produced.
    /// - Ok(None): Guard passed, Store updated, No output.
    /// - Err(()): **Guard Failed**. The runner should try the next available Phi.
    fn execute_phi(
        phi: Self::Phi,
        store: &mut Self::Store,
        input: &Self::Input,
    ) -> Result<Option<Self::Output>, ()>;
}
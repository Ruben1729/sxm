/// The core X-Machine Trait.
///
/// In theory, an X-Machine is M = (Sigma, Gamma, Q, M, Phi, F, m0, q0).
/// This trait maps those greek letters to Rust types.
pub trait XMachine {
    /// Sigma (Σ): The Input Alphabet.
    type Input: Clone + core::fmt::Debug + PartialEq + 'static;

    /// Gamma (Γ): The Output Alphabet.
    type Output: Clone + core::fmt::Debug + PartialEq + 'static;

    /// Q: The finite set of states.
    type State: Copy + Clone + PartialEq + core::fmt::Debug + 'static;

    /// M: Possibly infinite set called memory.
    type Memory: Clone;

    /// Phi: The finite set of partial functions.
    type Phi: Copy + Clone + PartialEq + core::fmt::Debug + 'static;

    /// F: Next state partial function
    fn next_state(state: Self::State, phi: Self::Phi) -> Option<Self::State>;

    /// I: Inital states
    fn initial_states() -> &'static [Self::State];

    /// T: Final states
    fn final_states() -> &'static [Self::State];

    /// m0: Initial Memory
    fn initial_store() -> Self::Memory;

    /// Describes how Phi is executed
    fn execute_phi(
        phi: Self::Phi,
        store: &mut Self::Memory,
        input: &Self::Input,
    ) -> Result<Option<Self::Output>, ()>;

    fn all_inputs() -> &'static [Self::Input];
    fn all_outputs() -> &'static [Self::Output];
    
    /// Returns a list of all possible states (Q)
    fn all_states() -> &'static [Self::State];

    /// Returns a list of all possible function symbols (Phi)
    fn all_phis() -> &'static [Self::Phi];

    fn get_phi_for_input(state: Self::State, input: &Self::Input) -> Option<Self::Phi>;
}
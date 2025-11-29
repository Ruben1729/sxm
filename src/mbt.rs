use crate::XMachine;
use std::fmt::Debug;
use std::collections::VecDeque;

/// Represents a generated test vector used to validate the implementation.
///
/// Based on the stream X-machine testing method, a test suite is constructed
/// by traversing the associated finite automaton
#[derive(Debug)]
pub struct TestCase<Input, Output> {
    /// A human-readable identifier for the test scenario.
    pub name: String,

    /// C (State Cover): The sequence of inputs required to reach the state under test.
    /// Derived from the State Cover Set of the associated finite automaton.
    pub setup_sequence: Vec<Input>,

    /// σ (Sigma): The specific input symbol applied to trigger the transition.
    /// Used to exercise a specific processing function φ or test Input-Completeness.
    pub test_input: Input,

    /// γ (Gamma): The expected output symbol produced by the processing function.
    /// Used to satisfy Output-Distinguishability.
    pub expected_output: Option<Output>,

    /// W (Characterization): The sequence of inputs used to verify the resulting state.
    /// Derived from the Characterization Set (W-set) to distinguish the final state.
    pub verification_sequence: Vec<Input>,
}

pub struct SxMTester;

impl SxMTester {
    /// Generates conformance tests (W-Method).
    /// These prove the implementation logic matches the Spec.
    pub fn generate_logic_tests<T: XMachine>(
        distinguishing_sequences: &dyn Fn(T::State) -> Vec<T::Input>,
    ) -> Vec<TestCase<T::Input, T::Output>> {
        let mut tests = Vec::new();

        for &target_state in T::all_states() {
            if let Some(path_to_state) = Self::find_path_to_state::<T>(target_state) {
                for input in T::all_inputs() {
                    if let Some(phi) = T::get_phi_for_input(target_state, input) {
                        if let Some(expected_next_state) = T::next_state(target_state, phi) {
                            let verify_seq = distinguishing_sequences(expected_next_state);
                            let mut dummy_mem = T::initial_store();
                            let expected_out =
                                T::execute_phi(phi, &mut dummy_mem, input).ok().flatten();

                            tests.push(TestCase {
                                name: format!(
                                    "Logic Verify: {:?} + {:?} -> {:?}",
                                    target_state, input, expected_next_state
                                ),
                                setup_sequence: path_to_state.clone(),
                                test_input: input.clone(),
                                expected_output: expected_out,
                                verification_sequence: verify_seq,
                            });
                        }
                    }
                }
            }
        }
        tests
    }

    /// Generates Input-Completeness tests.
    /// These prove the hardware handles invalid inputs safely.
    pub fn generate_robustness_tests<T: XMachine>() -> Vec<TestCase<T::Input, T::Output>> {
        let mut tests = Vec::new();

        for &state in T::all_states() {
            if let Some(path) = Self::find_path_to_state::<T>(state) {
                for input in T::all_inputs() {
                    let is_defined = T::get_phi_for_input(state, input).is_some();

                    if !is_defined {
                        tests.push(TestCase {
                            name: format!("Robustness: {:?} should reject {:?}", state, input),
                            setup_sequence: path.clone(),
                            test_input: input.clone(),
                            expected_output: None,
                            verification_sequence: vec![],
                        });
                    }
                }
            }
        }
        tests
    }

    /// Breadth-First Search to find the shortest input sequence to a target state
    fn find_path_to_state<T: XMachine>(target: T::State) -> Option<Vec<T::Input>> {
        let mut queue: VecDeque<(T::State, Vec<T::Input>)> = VecDeque::new();
        let mut visited: Vec<T::State> = Vec::new();

        for &start in T::initial_states() {
            if start == target {
                return Some(vec![]);
            }
            queue.push_back((start, vec![]));
            visited.push(start);
        }

        while let Some((current_state, path)) = queue.pop_front() {
            for input in T::all_inputs() {
                if let Some(phi) = T::get_phi_for_input(current_state, input) {
                    if let Some(next_state) = T::next_state(current_state, phi) {
                        if next_state == target {
                            let mut full_path = path.clone();
                            full_path.push(input.clone());
                            return Some(full_path);
                        }

                        if !visited.contains(&next_state) {
                            visited.push(next_state);
                            let mut new_path = path.clone();
                            new_path.push(input.clone());
                            queue.push_back((next_state, new_path));
                        }
                    }
                }
            }
        }
        None
    }

    /// Generates tests by finding a path to execute EVERY valid Phi function.
    /// This discovers data-dependent paths (like the PIN code).
    pub fn generate_phi_coverage_tests<T: XMachine>(
        distinguishing_sequences: &dyn Fn(T::State) -> Vec<T::Input>
    ) -> Vec<TestCase<T::Input, T::Output>> {

        let mut tests = Vec::new();
        for &start_state in T::all_states() {
            for input in T::all_inputs() {
                if let Some(target_phi) = T::get_phi_for_input(start_state, input) {
                    if let Some((setup_path, resulting_memory)) = Self::find_path_to_satisfy_phi::<T>(start_state, target_phi, input) {
                        let mut test_mem = resulting_memory.clone();
                        let expected_output = T::execute_phi(target_phi, &mut test_mem, input).ok().flatten();
                        let next_state = T::next_state(start_state, target_phi).unwrap();

                        tests.push(TestCase {
                            name: format!("Phi Verify: {:?} (via {:?})", target_phi, setup_path),
                            setup_sequence: setup_path,
                            test_input: input.clone(),
                            expected_output,
                            verification_sequence: distinguishing_sequences(next_state),
                        });
                    } else {
                        println!("Warning: Could not find data path to execute Phi '{:?}' from State '{:?}'", target_phi, start_state);
                    }
                }
            }
        }
        tests
    }

    /// BFS that tracks Memory to find a path where `execute_phi` succeeds.
    fn find_path_to_satisfy_phi<T: XMachine>(
        target_state: T::State,
        target_phi: T::Phi,
        trigger_input: &T::Input
    ) -> Option<(Vec<T::Input>, T::Memory)> {
        let mut queue = VecDeque::new();
        for &start in T::initial_states() {
            queue.push_back((start, T::initial_store(), Vec::new()));
        }

        let max_depth = 10;
        while let Some((curr_state, curr_mem, path)) = queue.pop_front() {
            if curr_state == target_state {
                let mut check_mem = curr_mem.clone();
                if T::execute_phi(target_phi, &mut check_mem, trigger_input).is_ok() {
                    return Some((path, curr_mem));
                }
            }
            if path.len() >= max_depth {
                continue;
            }

            for input in T::all_inputs() {
                if let Some(phi) = T::get_phi_for_input(curr_state, input) {
                    let mut next_mem = curr_mem.clone();

                    if let Ok(_) = T::execute_phi(phi, &mut next_mem, input) {
                        if let Some(next_state) = T::next_state(curr_state, phi) {
                            let mut new_path = path.clone();
                            new_path.push(input.clone());
                            queue.push_back((next_state, next_mem, new_path));
                        }
                    }
                }
            }
        }
        None
    }
}

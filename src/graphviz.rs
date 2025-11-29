use crate::XMachine;
use std::fmt::{Debug, Write};
use std::convert::TryFrom;

pub fn generate_dot<T: XMachine>(machine_name: &str) -> String {
    let mut output = String::new();
    writeln!(output, "digraph {} {{", machine_name).unwrap();
    writeln!(output, "    rankdir=LR;").unwrap();
    writeln!(output, "    node [shape=circle];").unwrap();
    writeln!(output, "    // Initial States").unwrap();
    for state in T::initial_states() {
        writeln!(output, "    \"_start_{:?}\" [style=invisible, label=\"\", width=0, height=0];", state).unwrap();
        writeln!(output, "    \"_start_{:?}\" -> \"{:?}\" [penwidth=2.0];", state, state).unwrap();
    }

    writeln!(output, "    // Terminal States").unwrap();
    for state in T::final_states() {
        writeln!(output, "    \"{:?}\" [shape=doublecircle];", state).unwrap();
    }

    writeln!(output, "    // Transitions").unwrap();
    for &source in T::all_states() {
        for &phi in T::all_phis() {
            if let Some(target) = T::next_state(source, phi) {
                writeln!(
                    output,
                    "    \"{:?}\" -> \"{:?}\" [label=\"{:?}\"];",
                    source, target, phi
                ).unwrap();
            }
        }
    }

    writeln!(output, "}}").unwrap();
    output
}

pub fn generate_generic_context_dot<MA, MB>() -> String
where
    MA: XMachine,
    MB: XMachine,
    MB::Input: TryFrom<MA::Output>,
    MA::Input: TryFrom<MB::Output>,
    MA::Output: Debug + PartialEq + Clone,
    MB::Output: Debug + PartialEq + Clone,
    MA::Input: Debug + PartialEq + Clone,
    MB::Input: Debug + PartialEq + Clone,
{
    let mut output = String::new();
    let mut internal_a_outputs = Vec::new();
    let mut internal_b_inputs = Vec::new();

    for out in MA::all_outputs() {
        if let Ok(derived_input) = MB::Input::try_from(out.clone()) {
            internal_a_outputs.push(out.clone());
            internal_b_inputs.push(derived_input);
        }
    }

    let mut internal_b_outputs = Vec::new();
    let mut internal_a_inputs = Vec::new();

    for out in MB::all_outputs() {
        if let Ok(derived_input) = MA::Input::try_from(out.clone()) {
            internal_b_outputs.push(out.clone());
            internal_a_inputs.push(derived_input);
        }
    }

    writeln!(output, "digraph GenericContext {{").unwrap();
    writeln!(output, "    rankdir=LR;").unwrap();
    writeln!(output, "    node [fontname=\"Arial\", fontsize=12];").unwrap();
    writeln!(output, "    node [shape=component, style=filled, fillcolor=lightgrey, height=2];").unwrap();
    writeln!(output, "    System [label=\"System\\n(Black Box)\"];").unwrap();
    writeln!(output, "    node [shape=none, style=none, fillcolor=none, height=0.5];").unwrap();
    writeln!(output, "    Environment_In [label=\"Environment\"];").unwrap();

    for input in MA::all_inputs() {
        if !internal_a_inputs.contains(input) {
            writeln!(output, "    Environment_In -> System [label=\"{:?}\"];", input).unwrap();
        }
    }

    for input in MB::all_inputs() {
        if !internal_b_inputs.contains(input) {
            writeln!(output, "    Environment_In -> System [label=\"{:?}\"];", input).unwrap();
        }
    }

    writeln!(output, "    Environment_Out [label=\"Environment\"];").unwrap();

    for out in MA::all_outputs() {
        if !internal_a_outputs.contains(out) {
            writeln!(output, "    System -> Environment_Out [label=\"{:?}\"];", out).unwrap();
        }
    }

    for out in MB::all_outputs() {
        if !internal_b_outputs.contains(out) {
            writeln!(output, "    System -> Environment_Out [label=\"{:?}\"];", out).unwrap();
        }
    }

    writeln!(output, "}}").unwrap();
    output
}

use sxm::XMachine;
use sxm::mbt::SxMTester;
use std::convert::TryFrom;

/// Adapter: Digicode Output -> Door Input
impl TryFrom<DigicodeOutputAlphabet> for DoorInputAlphabet {
    type Error = ();

    fn try_from(output: DigicodeOutputAlphabet) -> Result<Self, Self::Error> {
        match output {
            DigicodeOutputAlphabet::Open => Ok(DoorInputAlphabet::Open),
            _ => Err(()),
        }
    }
}

/// Adapter: Door Output -> Digicode Input
impl TryFrom<DoorOutputAlphabet> for DigicodeInputAlphabet {
    type Error = ();

    fn try_from(output: DoorOutputAlphabet) -> Result<Self, Self::Error> {
        match output {
            DoorOutputAlphabet::DoorCloses => Ok(DigicodeInputAlphabet::DoorCloses),
            _ => Err(()),
        }
    }
}

/// Input Alphabet (Σ)
#[derive(Clone, Debug, PartialEq)]
pub enum DigicodeInputAlphabet {
    OkEnter,
    DoorCloses,
    Digit(u8),
}

/// Output Alphabet (Γ)
#[derive(Clone, Debug, PartialEq)]
pub enum DigicodeOutputAlphabet {
    Digit(u8),
    Open,
    Initialise,
    IgnoreDigit,
    RejectInput,
    None,
}

/// States (Q)
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DigicodeState {
    Ready,
    Accepting,
    CodeEntered,
}

/// Memory (M)
#[derive(Clone, Debug, PartialEq)]
pub struct DigicodeMemory {
    pub current_sequence: Vec<u8>,
    pub valid_code: Vec<u8>,
}

/// Phi (Φ)
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DigicodePhi {
    Reject,
    InputDigit,
    Ignore,
    Finish,
    Lock,
}

pub struct Digicode;

impl XMachine for Digicode {
    type Input = DigicodeInputAlphabet;
    type Output = DigicodeOutputAlphabet;
    type State = DigicodeState;
    type Memory = DigicodeMemory;
    type Phi = DigicodePhi;

    fn next_state(state: Self::State, phi: Self::Phi) -> Option<Self::State> {
        use DigicodeState::*;
        use DigicodePhi::*;

        match (state, phi) {
            (Ready, InputDigit) => Some(Accepting),
            (Ready, Reject) => Some(Ready),
            (Accepting, InputDigit) => Some(Accepting),
            (Accepting, Finish) => Some(CodeEntered),
            (Accepting, Reject) => Some(Ready),
            (Accepting, Ignore) => Some(Accepting),
            (CodeEntered, Lock) => Some(Ready),
            _ => None,
        }
    }

    fn initial_states() -> &'static [Self::State] {
        &[DigicodeState::Ready]
    }

    fn final_states() -> &'static [Self::State] {
        use DigicodeState::*;
        &[Ready, Accepting, CodeEntered]
    }

    fn initial_store() -> Self::Memory {
        DigicodeMemory {
            current_sequence: Vec::new(),
            valid_code: vec![4, 9, 2],
        }
    }

    fn execute_phi(
        phi: Self::Phi,
        store: &mut Self::Memory,
        input: &Self::Input,
    ) -> Result<Option<Self::Output>, ()> {
        use DigicodePhi::*;
        use DigicodeInputAlphabet as In;
        use DigicodeOutputAlphabet as Out;

        match (phi, input) {
            (Reject, In::OkEnter) => {
                if store.current_sequence != store.valid_code {
                    store.current_sequence.clear();
                    Ok(Some(Out::RejectInput))
                } else {
                    Err(())
                }
            }
            (InputDigit, In::Digit(d)) => {
                if store.current_sequence.len() < store.valid_code.len() {
                    store.current_sequence.push(*d);
                    Ok(Some(Out::Digit(*d)))
                } else {
                    Err(())
                }
            }
            (Ignore, In::Digit(_)) => {
                if store.current_sequence.len() == store.valid_code.len() {
                    Ok(Some(Out::IgnoreDigit))
                } else {
                    Err(())
                }
            }
            (Finish, In::OkEnter) => {
                if store.current_sequence == store.valid_code {
                    Ok(Some(Out::Open))
                } else {
                    Err(())
                }
            }
            (Lock, In::DoorCloses) => {
                store.current_sequence.clear();
                Ok(Some(Out::Initialise))
            }
            _ => Err(()),
        }
    }

    fn all_states() -> &'static [Self::State] {
        use DigicodeState::*;
        &[Ready, Accepting, CodeEntered]
    }

    fn all_phis() -> &'static [Self::Phi] {
        use DigicodePhi::*;
        &[Reject, InputDigit, Ignore, Finish, Lock]
    }

    fn all_inputs() -> &'static [Self::Input] {
        use DigicodeInputAlphabet::*;
        &[
            OkEnter,
            DoorCloses,
            Digit(0),
            Digit(1),
            Digit(2),
            Digit(3),
            Digit(4),
            Digit(5),
            Digit(6),
            Digit(7),
            Digit(8),
            Digit(9)
        ]
    }

    fn all_outputs() -> &'static [Self::Output] {
        use DigicodeOutputAlphabet::*;
        &[
            Digit(0),
            Digit(1),
            Digit(2),
            Digit(3),
            Digit(4),
            Digit(5),
            Digit(6),
            Digit(7),
            Digit(8),
            Digit(9),
            Open,
            Initialise,
            IgnoreDigit,
            RejectInput,
        ]
    }

    fn get_phi_for_input(state: Self::State, input: &Self::Input) -> Option<Self::Phi> {
        use DigicodeState::*;
        use DigicodePhi::*;
        use DigicodeInputAlphabet::*;

        match (state, input) {
            (Ready, Digit(_)) => Some(InputDigit),
            (Ready, OkEnter) => Some(Reject),
            (Accepting, Digit(_)) => Some(InputDigit),
            (Accepting, OkEnter) => Some(Finish),
            (CodeEntered, DoorCloses) => Some(Lock),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DoorInputAlphabet {
    Open,
    Close,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DoorOutputAlphabet {
    DoorOpens,
    DoorCloses,
    OpenIgnored,
    CloseIgnored,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DoorState {
    Closed,
    Opened,
}

pub type DoorMemory = u32;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DoorPhi {
    OpenDoor,
    CloseDoor,
    IgnoreOpen,
    IgnoreClose,
}

pub struct Door;

impl XMachine for Door {
    type Input = DoorInputAlphabet;
    type Output = DoorOutputAlphabet;
    type State = DoorState;
    type Memory = DoorMemory;
    type Phi = DoorPhi;

    fn initial_states() -> &'static [Self::State] {
        &[DoorState::Closed]
    }

    fn final_states() -> &'static [Self::State] {
        use DoorState::*;
        &[Closed, Opened]
    }

    fn initial_store() -> Self::Memory {
        0
    }

    fn next_state(state: Self::State, phi: Self::Phi) -> Option<Self::State> {
        use DoorState::*;
        use DoorPhi::*;

        match (state, phi) {
            (Closed, OpenDoor) => Some(Opened),
            (Closed, IgnoreClose) => Some(Closed),
            (Opened, CloseDoor) => Some(Closed),
            (Opened, IgnoreOpen) => Some(Opened),
            _ => None,
        }
    }

    fn execute_phi(
        phi: Self::Phi,
        store: &mut Self::Memory,
        input: &Self::Input,
    ) -> Result<Option<Self::Output>, ()> {
        use DoorPhi::*;
        use DoorInputAlphabet as In;
        use DoorOutputAlphabet as Out;

        match (phi, input) {
            (OpenDoor, In::Open) => {
                *store += 1;
                Ok(Some(Out::DoorOpens))
            }
            (CloseDoor, In::Close) => Ok(Some(Out::DoorCloses)),
            (IgnoreOpen, In::Open) => Ok(Some(Out::OpenIgnored)),
            (IgnoreClose, In::Close) => Ok(Some(Out::CloseIgnored)),
            _ => Err(()),
        }
    }

    fn all_states() -> &'static [Self::State] {
        use DoorState::*;
        &[Closed, Opened]
    }

    fn all_phis() -> &'static [Self::Phi] {
        use DoorPhi::*;
        &[OpenDoor, CloseDoor, IgnoreOpen, IgnoreClose]
    }

    fn all_inputs() -> &'static [Self::Input] {
        use DoorInputAlphabet::*;
        &[
            Open,
            Close
        ]
    }

    fn all_outputs() -> &'static [Self::Output] {
        use DoorOutputAlphabet::*;
        &[
            DoorOpens,
            DoorCloses,
            OpenIgnored,
            CloseIgnored,
        ]
    }

    fn get_phi_for_input(state: Self::State, input: &Self::Input) -> Option<Self::Phi> {
        use DoorState::*;
        use DoorInputAlphabet::*;
        use DoorPhi::*;

        match (state, input) {
            (Closed, Open) => Some(OpenDoor),
            (Closed, Close) => Some(IgnoreClose),
            (Opened, Close) => Some(CloseDoor),
            (Opened, Open) => Some(IgnoreOpen),
        }
    }
}

pub struct SecureDoorSystem {
    pub digicode_mem: <Digicode as XMachine>::Memory,
    pub door_mem: <Door as XMachine>::Memory,
}

impl SecureDoorSystem {
    pub fn new() -> Self {
        Self {
            digicode_mem: Digicode::initial_store(),
            door_mem: Door::initial_store(),
        }
    }

    /// Processes an external input into the system.
    /// This mimics the "Change of Configuration" described in Definition 8.
    pub fn process_input(&mut self, input: DigicodeInputAlphabet) {
        let mut pending_digicode_input = Some(input);
        let mut pending_door_input: Option<DoorInputAlphabet> = None;

        loop {
            let mut internal_activity = false;
            if let Some(inp) = pending_digicode_input.take() {
                for &phi in Digicode::all_phis() {
                    if let Ok(Some(output)) = Digicode::execute_phi(phi, &mut self.digicode_mem, &inp) {
                        println!("  [Digicode] {:?} -> Output: {:?}", phi, output);
                        internal_activity = true;

                        if let Ok(door_inp) = DoorInputAlphabet::try_from(output.clone()) {
                            println!("  [Network] Routing {:?} to Door", output);
                            pending_door_input = Some(door_inp);
                        } else {
                            println!("  [Environment] Output: {:?}", output);
                        }
                        break;
                    }
                }
            }

            if let Some(inp) = pending_door_input.take() {
                for &phi in Door::all_phis() {
                    if let Ok(Some(output)) = Door::execute_phi(phi, &mut self.door_mem, &inp) {
                        println!("  [Door] {:?} -> Output: {:?}", phi, output);
                        internal_activity = true;

                        if let Ok(digi_inp) = DigicodeInputAlphabet::try_from(output.clone()) {
                            println!("  [Network] Routing {:?} to Digicode", output);
                            pending_digicode_input = Some(digi_inp);
                        } else {
                            println!("  [Environment] Output: {:?}", output);
                        }
                        break;
                    }
                }
            }

            if !internal_activity {
                break;
            }
        }
    }
}

fn main() {
    let mut system = SecureDoorSystem::new();

    // 1. Enter the code <4, 9, 2>
    system.process_input(DigicodeInputAlphabet::Digit(4));
    system.process_input(DigicodeInputAlphabet::Digit(9));
    system.process_input(DigicodeInputAlphabet::Digit(2));

    // 2. Press OK. This triggers the chain reaction:
    // Digicode(Finish) -> outputs Open -> Door(OpenDoor) -> outputs DoorOpens
    system.process_input(DigicodeInputAlphabet::OkEnter);

    // Check Memory: Door should have opened once (count = 1)
    println!("Door Memory (open count): {}", system.door_mem);

    // Define the "W" set (Distinguishing Sequences) manually for Digicode
    // "If I am in State X, what input proves it?"
    let identifier_map = |state: DigicodeState| -> Vec<DigicodeInputAlphabet> {
        match state {
            DigicodeState::Ready => vec![DigicodeInputAlphabet::Digit(1)],
            DigicodeState::CodeEntered => vec![DigicodeInputAlphabet::DoorCloses],
            DigicodeState::Accepting => vec![DigicodeInputAlphabet::OkEnter],
        }
    };

    // 1. Generate Logic Tests (Conformance)
    let logic_tests = SxMTester::generate_logic_tests::<Digicode>(&identifier_map);
    println!("--- Logic Tests ({}) ---", logic_tests.len());
    for t in logic_tests {
        println!("{:?}", t.name);
    }

    // 2. Generate Robustness Tests (Input Completeness)
    let robust_tests = SxMTester::generate_robustness_tests::<Digicode>();
    println!("\n--- Robustness Tests ({}) ---", robust_tests.len());
    for t in robust_tests {
        println!("{:?}", t.name);
    }

    // 3. Generate Phi Coverage Tests (Data-Dependent)
    let phi_tests = SxMTester::generate_phi_coverage_tests::<Digicode>(&identifier_map);
    println!("\n--- Phi Tests ({}) ---", phi_tests.len());
    for t in phi_tests {
        println!("{} using Input: {:?}", t.name, t.test_input);
    }
}

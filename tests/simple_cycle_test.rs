// tests/simple_cycle_test.rs
use sxm::{XMachine, MachineRunner};

// --- Definitions ---
#[derive(Debug, Clone, Copy, PartialEq)]
enum SwitchState {
    Off,
    On,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SwitchPhi {
    TurnOn,
    TurnOff,
}

struct SwitchMemory {
    clicks: u32,
}

struct LightSwitch;

// --- Implementation ---
impl XMachine for LightSwitch {
    type Input = u8; // 1 = force action, 0 = ignore
    type Output = String;
    type State = SwitchState;
    type Store = SwitchMemory;
    type Phi = SwitchPhi;

    fn initial_state() -> Self::State {
        SwitchState::Off
    }

    fn initial_store() -> Self::Store {
        SwitchMemory { clicks: 0 }
    }

    // 1. Topology: Define the Graph
    fn get_available_phi(state: Self::State) -> &'static [Self::Phi] {
        match state {
            SwitchState::Off => &[SwitchPhi::TurnOn],
            SwitchState::On => &[SwitchPhi::TurnOff],
        }
    }

    // 2. Control Flow: Define destinations
    fn next_state(state: Self::State, phi: Self::Phi) -> Self::State {
        match (state, phi) {
            (SwitchState::Off, SwitchPhi::TurnOn) => SwitchState::On,
            (SwitchState::On, SwitchPhi::TurnOff) => SwitchState::Off,
            _ => state, // Should be unreachable if topology is correct
        }
    }

    // 3. Data Processing: Guards and Updates
    fn execute_phi(
        phi: Self::Phi,
        store: &mut Self::Store,
        input: &Self::Input,
    ) -> Result<Option<Self::Output>, ()> {

        // GLOBAL GUARD: Only accept input 1 (Simulating a button press)
        if *input != 1 {
            return Err(()); // Guard Failed
        }

        // Processing Logic
        match phi {
            SwitchPhi::TurnOn => {
                store.clicks += 1;
                Ok(Some(format!("ON ({} clicks)", store.clicks)))
            }
            SwitchPhi::TurnOff => {
                store.clicks += 1;
                Ok(Some(format!("OFF ({} clicks)", store.clicks)))
            }
        }
    }
}

#[test]
fn test_separated_logic() {
    let mut machine = MachineRunner::<LightSwitch>::new();

    // 1. Send '0' (Noise) - Should fail guard
    let res = machine.step(0);
    assert!(res.is_err());
    assert_eq!(machine.state, SwitchState::Off);

    // 2. Send '1' - Should transition to ON
    let res = machine.step(1);
    assert!(res.is_ok());
    assert_eq!(machine.state, SwitchState::On);

    // 3. Send '1' - Should transition to OFF
    let res = machine.step(1);
    assert!(res.is_ok());
    assert_eq!(machine.state, SwitchState::Off);
}
use aptos_keyless_common::api::{RequestInput, ProverServiceResponse};
use std::sync::Mutex;
use rust_rapidsnark::{FullProver, ProverInitError};

pub type Bytes = Vec<u8>;
#[derive(Default)]
pub struct Groth16Vk {
    pub alpha_g1: Bytes,
    pub beta_g1: Bytes,
    pub delta_g2: Bytes,
    pub gamma_abc_g1: Vec<Bytes>,
    pub gamma_g2: Bytes,
}

// TODO: Implement
pub struct RapidProver {
    pub prover: Option<FullProver>
}

impl Default for RapidProver {
    fn default() -> Self {
        let prover = FullProver::new(&"Default").ok();
        RapidProver {
            prover
        }
    }
}

#[derive(Default)]
// TODO: Implement
pub struct Config {}
#[derive(Default)]
// TODO: Implement
pub struct CircuitConfig {}

#[derive(Default)]
pub struct State {
    pub prover: Mutex<RapidProver>, // from rust-snark wrapper
    pub groth16_vk: Groth16Vk,
    pub config: Config,
    pub circuit_config: CircuitConfig,
}

impl State {
    fn new() -> Self {
        Default::default()
    }
}

// TODO: Make main Tokio runtime
fn main() {
    //TODO: Add Service to do some sending/receiving of proof requests etc

    //TODO: log service

    //TODO: Add verification key file read

    //TODO: Initialisze State
    //TODO: Config
    //TODO: wrap state in arc pointer to share to threads

    //TODO: Fetch JWK refresh in some interval -- async fn

    //TODO: Some metrics service start perhaps
}

#[test]
fn can_create_state() {
    let test_state = State::new();

    assert!(test_state.groth16_vk.alpha_g1.is_empty());

    let rapid_prover = test_state.prover.try_lock().unwrap();
    assert!(rapid_prover.prover.as_ref().is_none());
}

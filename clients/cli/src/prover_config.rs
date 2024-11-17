// use crate::generated;

use crate::prover_id_manager;

// use crate::analytics::track;
// use crate::connection::{connect_to_orchestrator_with_retry};

use nexus_core::{
    prover::nova::{
        pp::gen_vm_pp, 
        types::{seq, C1, G1, G2, C2, RO, SC, PublicParams}
    },
};


pub struct ProverConfig {
    pub prover_id: String,
    // pub client: WebSocketStream<MaybeTlsStream<TcpStream>>,
    pub k: i32,
    pub ws_addr_string: String,
    pub public_parameters: PublicParams<G1, G2, C1, C2, RO, SC, seq::SetupParams<(G1, G2, C1, C2, RO, SC)>>,
}

pub async fn initialize(hostname: String, port: u16) -> Result<ProverConfig, Box<dyn std::error::Error>> {
    // Construct the WebSocket URL based on the port number
    // Uses secure WebSocket (wss) for port 443, regular WebSocket (ws) otherwise
    let ws_addr_string = format!(
        "{}://{}:{}/prove",
        if port == 443 { "wss" } else { "ws" },
        hostname,
        port
    );

    // Set the constant k value used for proof generation
    // This determines the size/complexity of the proving system
    let k = 4;

    // Retrieve an existing prover ID from storage or generate a new one
    // This ID uniquely identifies this prover instance
    let prover_id = prover_id_manager::get_or_generate_prover_id();

    // Generate the public parameters for the proving system
    let public_parameters: PublicParams<G1, G2, C1, C2, RO, SC, seq::SetupParams<(G1, G2, C1, C2, RO, SC)>> = 
        match gen_vm_pp::<C1, seq::SetupParams<(G1, G2, C1, C2, RO, SC)>>(
            k as usize, 
            &()
        ) {
            Ok(params) => params,
            Err(e) => return Err(format!("Failed to generate public parameters: {}", e).into())
        };

    // Construct and return the ProverConfig with the initialized values
    Ok(ProverConfig {
        ws_addr_string,
        k,
        prover_id,
        public_parameters
    })
}
use coremidi::network::{NetworkConnection, NetworkHost, NetworkSession};

#[test]
fn network_area_exposes_session_metadata() {
    let session = NetworkSession::default();
    let constants = NetworkSession::constants().expect("network constants available");
    assert!(!constants.bonjour_service_type.is_empty());
    assert!(session.connection_policy().is_ok());
    assert!(session.contacts().is_ok());
    assert!(session.connections().is_ok());

    let host = NetworkHost::with_address("example", "127.0.0.1", 5004);
    let connection = NetworkConnection::new(host.clone());
    assert_eq!(connection.host, host);
}

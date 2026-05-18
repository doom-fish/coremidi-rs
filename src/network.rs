use core::ffi::c_char;
use std::ptr;

use serde::{Deserialize, Serialize};

use crate::endpoint::MidiEndpoint;
use crate::error::{MidiError, MidiResult};
use crate::ffi;
use crate::private;

extern "C" {
    fn cmr_network_constants_json() -> *mut c_char;
    fn cmr_network_session_is_enabled() -> bool;
    fn cmr_network_session_set_enabled(enabled: bool);
    fn cmr_network_session_network_port() -> i32;
    fn cmr_network_session_network_name() -> *mut c_char;
    fn cmr_network_session_local_name() -> *mut c_char;
    fn cmr_network_session_connection_policy() -> i32;
    fn cmr_network_session_set_connection_policy(
        raw_value: i32,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_network_session_contacts_json() -> *mut c_char;
    fn cmr_network_session_add_contact_json(
        json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_network_session_remove_contact_json(
        json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_network_session_connections_json() -> *mut c_char;
    fn cmr_network_session_add_connection_json(
        json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_network_session_remove_connection_json(
        json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_network_session_source_endpoint() -> ffi::MIDIEndpointRef;
    fn cmr_network_session_destination_endpoint() -> ffi::MIDIEndpointRef;
    fn cmr_network_activate_bluetooth_connections(error_out: *mut *mut c_char) -> i32;
    fn cmr_network_disconnect_bluetooth(uuid: *const c_char, error_out: *mut *mut c_char) -> i32;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
/// Wraps `MIDINetworkConnectionPolicy` values.
pub enum NetworkConnectionPolicy {
    /// Wraps the CoreMIDI no one case.
    NoOne = 0,
    /// Wraps the CoreMIDI hosts in contact list case.
    HostsInContactList = 1,
    /// Wraps the CoreMIDI anyone case.
    Anyone = 2,
}

impl NetworkConnectionPolicy {
    #[must_use]
    /// Wraps an existing `MIDINetworkConnectionPolicy`.
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::NoOne),
            1 => Some(Self::HostsInContactList),
            2 => Some(Self::Anyone),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
/// Wraps `MIDINetworkHost`.
pub struct NetworkHost {
    /// Mirrors the CoreMIDI name field.
    pub name: String,
    /// Mirrors the CoreMIDI address field.
    pub address: Option<String>,
    /// Mirrors the CoreMIDI port field.
    pub port: u64,
    /// Mirrors the CoreMIDI net service name field.
    pub net_service_name: Option<String>,
    /// Mirrors the CoreMIDI net service domain field.
    pub net_service_domain: Option<String>,
}

impl NetworkHost {
    #[must_use]
    /// Wraps the CoreMIDI with address operation for `NetworkHost`.
    pub fn with_address(name: impl Into<String>, address: impl Into<String>, port: u64) -> Self {
        Self {
            name: name.into(),
            address: Some(address.into()),
            port,
            net_service_name: None,
            net_service_domain: None,
        }
    }

    #[must_use]
    /// Wraps the CoreMIDI with service operation for `NetworkHost`.
    pub fn with_service(
        name: impl Into<String>,
        net_service_name: impl Into<String>,
        net_service_domain: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            address: None,
            port: 0,
            net_service_name: Some(net_service_name.into()),
            net_service_domain: Some(net_service_domain.into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Wraps `MIDINetworkConnection`.
pub struct NetworkConnection {
    /// Mirrors the CoreMIDI host field.
    pub host: NetworkHost,
}

impl NetworkConnection {
    #[must_use]
    /// Wraps the CoreMIDI new operation for `NetworkConnection`.
    pub const fn new(host: NetworkHost) -> Self {
        Self { host }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
/// Wraps `MIDINetworkConnection`.
pub struct NetworkConstants {
    /// Mirrors the CoreMIDI bonjour service type field.
    pub bonjour_service_type: String,
    /// Mirrors the CoreMIDI contacts changed notification field.
    pub contacts_changed_notification: String,
    /// Mirrors the CoreMIDI session changed notification field.
    pub session_changed_notification: String,
}

#[derive(Debug, Default, Clone, Copy)]
/// Wraps `MIDINetworkSession`.
pub struct NetworkSession;

impl NetworkSession {
    #[must_use]
    /// Wraps access to the shared `MIDINetworkSession`.
    pub const fn default() -> Self {
        Self
    }

    /// Wraps the CoreMIDI constants operation for `NetworkSession`.
    pub fn constants() -> MidiResult<NetworkConstants> {
        unsafe { private::take_json(cmr_network_constants_json()) }
    }

    #[must_use]
    /// Wraps the CoreMIDI is enabled operation for `NetworkSession`.
    pub fn is_enabled(self) -> bool {
        unsafe { cmr_network_session_is_enabled() }
    }

    /// Wraps the CoreMIDI set enabled operation for `NetworkSession`.
    pub fn set_enabled(self, enabled: bool) {
        unsafe { cmr_network_session_set_enabled(enabled) };
    }

    #[must_use]
    /// Wraps the CoreMIDI network port operation for `NetworkSession`.
    pub fn network_port(self) -> u64 {
        u64::try_from(unsafe { cmr_network_session_network_port().max(0) }).unwrap_or(0)
    }

    /// Wraps the CoreMIDI network name operation for `NetworkSession`.
    pub fn network_name(self) -> MidiResult<String> {
        Ok(unsafe { private::take_c_string(cmr_network_session_network_name()) })
    }

    /// Wraps the CoreMIDI local name operation for `NetworkSession`.
    pub fn local_name(self) -> MidiResult<String> {
        Ok(unsafe { private::take_c_string(cmr_network_session_local_name()) })
    }

    /// Wraps the CoreMIDI connection policy operation for `NetworkSession`.
    pub fn connection_policy(self) -> MidiResult<NetworkConnectionPolicy> {
        NetworkConnectionPolicy::from_raw(unsafe { cmr_network_session_connection_policy() })
            .ok_or_else(|| MidiError::Bridge("unknown MIDINetworkConnectionPolicy value".into()))
    }

    /// Wraps the CoreMIDI set connection policy operation for `NetworkSession`.
    pub fn set_connection_policy(self, policy: NetworkConnectionPolicy) -> MidiResult<()> {
        let mut error = ptr::null_mut();
        unsafe {
            private::swift_result(
                cmr_network_session_set_connection_policy(policy as i32, &mut error),
                error,
            )
        }
    }

    /// Wraps the CoreMIDI contacts operation for `NetworkSession`.
    pub fn contacts(self) -> MidiResult<Vec<NetworkHost>> {
        unsafe { private::take_json(cmr_network_session_contacts_json()) }
    }

    /// Wraps the CoreMIDI add contact operation for `NetworkSession`.
    pub fn add_contact(self, host: &NetworkHost) -> MidiResult<()> {
        let payload = private::encode_json(host)?;
        let mut error = ptr::null_mut();
        unsafe {
            private::swift_result(
                cmr_network_session_add_contact_json(payload.as_ptr(), &mut error),
                error,
            )
        }
    }

    /// Wraps the CoreMIDI remove contact operation for `NetworkSession`.
    pub fn remove_contact(self, host: &NetworkHost) -> MidiResult<()> {
        let payload = private::encode_json(host)?;
        let mut error = ptr::null_mut();
        unsafe {
            private::swift_result(
                cmr_network_session_remove_contact_json(payload.as_ptr(), &mut error),
                error,
            )
        }
    }

    /// Wraps the CoreMIDI connections operation for `NetworkSession`.
    pub fn connections(self) -> MidiResult<Vec<NetworkConnection>> {
        unsafe { private::take_json(cmr_network_session_connections_json()) }
    }

    /// Wraps the CoreMIDI add connection operation for `NetworkSession`.
    pub fn add_connection(self, connection: &NetworkConnection) -> MidiResult<()> {
        let payload = private::encode_json(connection)?;
        let mut error = ptr::null_mut();
        unsafe {
            private::swift_result(
                cmr_network_session_add_connection_json(payload.as_ptr(), &mut error),
                error,
            )
        }
    }

    /// Wraps the CoreMIDI remove connection operation for `NetworkSession`.
    pub fn remove_connection(self, connection: &NetworkConnection) -> MidiResult<()> {
        let payload = private::encode_json(connection)?;
        let mut error = ptr::null_mut();
        unsafe {
            private::swift_result(
                cmr_network_session_remove_connection_json(payload.as_ptr(), &mut error),
                error,
            )
        }
    }

    #[must_use]
    /// Wraps the CoreMIDI source endpoint operation for `NetworkSession`.
    pub fn source_endpoint(self) -> MidiEndpoint {
        unsafe { MidiEndpoint::from_raw(cmr_network_session_source_endpoint()) }
    }

    #[must_use]
    /// Wraps the CoreMIDI destination endpoint operation for `NetworkSession`.
    pub fn destination_endpoint(self) -> MidiEndpoint {
        unsafe { MidiEndpoint::from_raw(cmr_network_session_destination_endpoint()) }
    }
}

/// Wraps the CoreMIDI activate bluetooth connections operation for `NetworkSession`.
pub fn activate_bluetooth_connections() -> MidiResult<()> {
    let mut error = ptr::null_mut();
    unsafe {
        private::swift_result(
            cmr_network_activate_bluetooth_connections(&mut error),
            error,
        )
    }
}

/// Wraps the CoreMIDI disconnect bluetooth operation for `NetworkSession`.
pub fn disconnect_bluetooth(uuid: &str) -> MidiResult<()> {
    let uuid = private::to_cstring(uuid)?;
    let mut error = ptr::null_mut();
    unsafe {
        private::swift_result(
            cmr_network_disconnect_bluetooth(uuid.as_ptr(), &mut error),
            error,
        )
    }
}

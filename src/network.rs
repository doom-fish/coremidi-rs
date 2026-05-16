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
    fn cmr_network_session_set_connection_policy(raw_value: i32, error_out: *mut *mut c_char) -> i32;
    fn cmr_network_session_contacts_json() -> *mut c_char;
    fn cmr_network_session_add_contact_json(json: *const c_char, error_out: *mut *mut c_char) -> i32;
    fn cmr_network_session_remove_contact_json(json: *const c_char, error_out: *mut *mut c_char) -> i32;
    fn cmr_network_session_connections_json() -> *mut c_char;
    fn cmr_network_session_add_connection_json(json: *const c_char, error_out: *mut *mut c_char) -> i32;
    fn cmr_network_session_remove_connection_json(json: *const c_char, error_out: *mut *mut c_char) -> i32;
    fn cmr_network_session_source_endpoint() -> ffi::MIDIEndpointRef;
    fn cmr_network_session_destination_endpoint() -> ffi::MIDIEndpointRef;
    fn cmr_network_activate_bluetooth_connections(error_out: *mut *mut c_char) -> i32;
    fn cmr_network_disconnect_bluetooth(uuid: *const c_char, error_out: *mut *mut c_char) -> i32;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum NetworkConnectionPolicy {
    NoOne = 0,
    HostsInContactList = 1,
    Anyone = 2,
}

impl NetworkConnectionPolicy {
    #[must_use]
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
pub struct NetworkHost {
    pub name: String,
    pub address: Option<String>,
    pub port: u64,
    pub net_service_name: Option<String>,
    pub net_service_domain: Option<String>,
}

impl NetworkHost {
    #[must_use]
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
pub struct NetworkConnection {
    pub host: NetworkHost,
}

impl NetworkConnection {
    #[must_use]
    pub const fn new(host: NetworkHost) -> Self {
        Self { host }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct NetworkConstants {
    pub bonjour_service_type: String,
    pub contacts_changed_notification: String,
    pub session_changed_notification: String,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct NetworkSession;

impl NetworkSession {
    #[must_use]
    pub const fn default() -> Self {
        Self
    }

    pub fn constants() -> MidiResult<NetworkConstants> {
        unsafe { private::take_json(cmr_network_constants_json()) }
    }

    #[must_use]
    pub fn is_enabled(self) -> bool {
        unsafe { cmr_network_session_is_enabled() }
    }

    pub fn set_enabled(self, enabled: bool) {
        unsafe { cmr_network_session_set_enabled(enabled) };
    }

    #[must_use]
    pub fn network_port(self) -> u64 {
        u64::try_from(unsafe { cmr_network_session_network_port().max(0) }).unwrap_or(0)
    }

    pub fn network_name(self) -> MidiResult<String> {
        Ok(unsafe { private::take_c_string(cmr_network_session_network_name()) })
    }

    pub fn local_name(self) -> MidiResult<String> {
        Ok(unsafe { private::take_c_string(cmr_network_session_local_name()) })
    }

    pub fn connection_policy(self) -> MidiResult<NetworkConnectionPolicy> {
        NetworkConnectionPolicy::from_raw(unsafe { cmr_network_session_connection_policy() })
            .ok_or_else(|| MidiError::Bridge("unknown MIDINetworkConnectionPolicy value".into()))
    }

    pub fn set_connection_policy(self, policy: NetworkConnectionPolicy) -> MidiResult<()> {
        let mut error = ptr::null_mut();
        unsafe {
            private::swift_result(
                cmr_network_session_set_connection_policy(policy as i32, &mut error),
                error,
            )
        }
    }

    pub fn contacts(self) -> MidiResult<Vec<NetworkHost>> {
        unsafe { private::take_json(cmr_network_session_contacts_json()) }
    }

    pub fn add_contact(self, host: &NetworkHost) -> MidiResult<()> {
        let payload = private::encode_json(host)?;
        let mut error = ptr::null_mut();
        unsafe { private::swift_result(cmr_network_session_add_contact_json(payload.as_ptr(), &mut error), error) }
    }

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

    pub fn connections(self) -> MidiResult<Vec<NetworkConnection>> {
        unsafe { private::take_json(cmr_network_session_connections_json()) }
    }

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
    pub fn source_endpoint(self) -> MidiEndpoint {
        unsafe { MidiEndpoint::from_raw(cmr_network_session_source_endpoint()) }
    }

    #[must_use]
    pub fn destination_endpoint(self) -> MidiEndpoint {
        unsafe { MidiEndpoint::from_raw(cmr_network_session_destination_endpoint()) }
    }
}

pub fn activate_bluetooth_connections() -> MidiResult<()> {
    let mut error = ptr::null_mut();
    unsafe { private::swift_result(cmr_network_activate_bluetooth_connections(&mut error), error) }
}

pub fn disconnect_bluetooth(uuid: &str) -> MidiResult<()> {
    let uuid = private::to_cstring(uuid)?;
    let mut error = ptr::null_mut();
    unsafe { private::swift_result(cmr_network_disconnect_bluetooth(uuid.as_ptr(), &mut error), error) }
}

use core::fmt;

use crate::ffi;

pub type MidiResult<T> = Result<T, MidiError>;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MidiError {
    InvalidArgument(String),
    CoreFoundation(String),
    Bridge(String),
    Serialization(String),
    BufferTooSmall { requested: usize, available: usize },
    Unsupported(String),
    Status(MidiStatus),
}

impl fmt::Display for MidiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(message) => write!(f, "invalid argument: {message}"),
            Self::CoreFoundation(message) => write!(f, "CoreFoundation error: {message}"),
            Self::Bridge(message) => write!(f, "CoreMIDI bridge error: {message}"),
            Self::Serialization(message) => write!(f, "serialization error: {message}"),
            Self::BufferTooSmall {
                requested,
                available,
            } => {
                write!(
                    f,
                    "buffer too small: requested {requested} bytes, available {available} bytes"
                )
            }
            Self::Unsupported(message) => write!(f, "unsupported operation: {message}"),
            Self::Status(status) => write!(f, "{status}"),
        }
    }
}

impl std::error::Error for MidiError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MidiStatus {
    InvalidClient,
    InvalidPort,
    WrongEndpointType,
    NoConnection,
    UnknownEndpoint,
    UnknownProperty,
    WrongPropertyType,
    NoCurrentSetup,
    MessageSend,
    ServerStart,
    SetupFormat,
    WrongThread,
    ObjectNotFound,
    IdNotUnique,
    NotPermitted,
    UnknownError,
    OsStatus(ffi::OSStatus),
}

impl MidiStatus {
    #[must_use]
    pub const fn from_raw(status: ffi::OSStatus) -> Self {
        match status {
            ffi::kMIDIInvalidClient => Self::InvalidClient,
            ffi::kMIDIInvalidPort => Self::InvalidPort,
            ffi::kMIDIWrongEndpointType => Self::WrongEndpointType,
            ffi::kMIDINoConnection => Self::NoConnection,
            ffi::kMIDIUnknownEndpoint => Self::UnknownEndpoint,
            ffi::kMIDIUnknownProperty => Self::UnknownProperty,
            ffi::kMIDIWrongPropertyType => Self::WrongPropertyType,
            ffi::kMIDINoCurrentSetup => Self::NoCurrentSetup,
            ffi::kMIDIMessageSendErr => Self::MessageSend,
            ffi::kMIDIServerStartErr => Self::ServerStart,
            ffi::kMIDISetupFormatErr => Self::SetupFormat,
            ffi::kMIDIWrongThread => Self::WrongThread,
            ffi::kMIDIObjectNotFound => Self::ObjectNotFound,
            ffi::kMIDIIDNotUnique => Self::IdNotUnique,
            ffi::kMIDINotPermitted => Self::NotPermitted,
            ffi::kMIDIUnknownError => Self::UnknownError,
            other => Self::OsStatus(other),
        }
    }

    #[must_use]
    pub const fn raw(self) -> ffi::OSStatus {
        match self {
            Self::InvalidClient => ffi::kMIDIInvalidClient,
            Self::InvalidPort => ffi::kMIDIInvalidPort,
            Self::WrongEndpointType => ffi::kMIDIWrongEndpointType,
            Self::NoConnection => ffi::kMIDINoConnection,
            Self::UnknownEndpoint => ffi::kMIDIUnknownEndpoint,
            Self::UnknownProperty => ffi::kMIDIUnknownProperty,
            Self::WrongPropertyType => ffi::kMIDIWrongPropertyType,
            Self::NoCurrentSetup => ffi::kMIDINoCurrentSetup,
            Self::MessageSend => ffi::kMIDIMessageSendErr,
            Self::ServerStart => ffi::kMIDIServerStartErr,
            Self::SetupFormat => ffi::kMIDISetupFormatErr,
            Self::WrongThread => ffi::kMIDIWrongThread,
            Self::ObjectNotFound => ffi::kMIDIObjectNotFound,
            Self::IdNotUnique => ffi::kMIDIIDNotUnique,
            Self::NotPermitted => ffi::kMIDINotPermitted,
            Self::UnknownError => ffi::kMIDIUnknownError,
            Self::OsStatus(status) => status,
        }
    }
}

impl fmt::Display for MidiStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidClient => write!(f, "invalid MIDI client"),
            Self::InvalidPort => write!(f, "invalid MIDI port"),
            Self::WrongEndpointType => write!(f, "wrong endpoint type"),
            Self::NoConnection => write!(f, "no such MIDI connection"),
            Self::UnknownEndpoint => write!(f, "unknown MIDI endpoint"),
            Self::UnknownProperty => write!(f, "unknown MIDI property"),
            Self::WrongPropertyType => write!(f, "wrong MIDI property type"),
            Self::NoCurrentSetup => write!(f, "no current MIDI setup"),
            Self::MessageSend => write!(f, "CoreMIDI message send error"),
            Self::ServerStart => write!(f, "CoreMIDI server start error"),
            Self::SetupFormat => write!(f, "CoreMIDI setup format error"),
            Self::WrongThread => write!(f, "CoreMIDI API called from the wrong thread"),
            Self::ObjectNotFound => write!(f, "CoreMIDI object not found"),
            Self::IdNotUnique => write!(f, "CoreMIDI unique ID collision"),
            Self::NotPermitted => write!(f, "CoreMIDI operation not permitted"),
            Self::UnknownError => write!(f, "unknown CoreMIDI error"),
            Self::OsStatus(status) => write!(f, "CoreMIDI returned OSStatus {status}"),
        }
    }
}

pub(crate) fn result_from_status(status: ffi::OSStatus) -> MidiResult<()> {
    if status == ffi::noErr {
        Ok(())
    } else {
        Err(MidiError::Status(MidiStatus::from_raw(status)))
    }
}

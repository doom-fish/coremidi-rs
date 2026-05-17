use serde::Deserialize;

use crate::error::{MidiError, MidiResult};
use crate::ffi;
use crate::property::MidiObjectType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum NotificationMessageId {
    SetupChanged = ffi::kMIDIMsgSetupChanged,
    ObjectAdded = ffi::kMIDIMsgObjectAdded,
    ObjectRemoved = ffi::kMIDIMsgObjectRemoved,
    PropertyChanged = ffi::kMIDIMsgPropertyChanged,
    ThruConnectionsChanged = ffi::kMIDIMsgThruConnectionsChanged,
    SerialPortOwnerChanged = ffi::kMIDIMsgSerialPortOwnerChanged,
    IoError = ffi::kMIDIMsgIOError,
    InternalStart = ffi::kMIDIMsgInternalStart,
}

impl NotificationMessageId {
    #[must_use]
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            ffi::kMIDIMsgSetupChanged => Some(Self::SetupChanged),
            ffi::kMIDIMsgObjectAdded => Some(Self::ObjectAdded),
            ffi::kMIDIMsgObjectRemoved => Some(Self::ObjectRemoved),
            ffi::kMIDIMsgPropertyChanged => Some(Self::PropertyChanged),
            ffi::kMIDIMsgThruConnectionsChanged => Some(Self::ThruConnectionsChanged),
            ffi::kMIDIMsgSerialPortOwnerChanged => Some(Self::SerialPortOwnerChanged),
            ffi::kMIDIMsgIOError => Some(Self::IoError),
            ffi::kMIDIMsgInternalStart => Some(Self::InternalStart),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Notification {
    SetupChanged,
    ObjectAdded {
        parent: Option<ffi::MIDIObjectRef>,
        parent_type: Option<MidiObjectType>,
        child: ffi::MIDIObjectRef,
        child_type: MidiObjectType,
    },
    ObjectRemoved {
        parent: Option<ffi::MIDIObjectRef>,
        parent_type: Option<MidiObjectType>,
        child: ffi::MIDIObjectRef,
        child_type: MidiObjectType,
    },
    PropertyChanged {
        object: ffi::MIDIObjectRef,
        object_type: MidiObjectType,
        property_name: String,
    },
    ThruConnectionsChanged,
    SerialPortOwnerChanged,
    IoError {
        driver_device: ffi::MIDIDeviceRef,
        error_code: ffi::OSStatus,
    },
    Unknown {
        message_id: i32,
        message_size: u32,
    },
}

#[derive(Debug, Deserialize)]
struct NotificationPayload {
    message_id: i32,
    message_size: u32,
    parent: Option<ffi::MIDIObjectRef>,
    parent_type: Option<i32>,
    child: Option<ffi::MIDIObjectRef>,
    child_type: Option<i32>,
    object: Option<ffi::MIDIObjectRef>,
    object_type: Option<i32>,
    property_name: Option<String>,
    driver_device: Option<ffi::MIDIDeviceRef>,
    error_code: Option<ffi::OSStatus>,
}

impl Notification {
    pub fn from_json_str(payload: &str) -> MidiResult<Self> {
        let payload: NotificationPayload = serde_json::from_str(payload)
            .map_err(|error| MidiError::Serialization(error.to_string()))?;
        Self::from_payload(payload)
    }

    #[allow(clippy::cast_ptr_alignment)]
    pub unsafe fn from_raw_ptr(message: *const ffi::MIDINotification) -> MidiResult<Self> {
        if message.is_null() {
            return Err(MidiError::InvalidArgument(
                "notification pointer must not be null".into(),
            ));
        }

        let message_ref = &*message;
        match NotificationMessageId::from_raw(message_ref.messageID) {
            Some(NotificationMessageId::SetupChanged) => Ok(Self::SetupChanged),
            Some(NotificationMessageId::ObjectAdded) => {
                let typed = &*(message.cast::<ffi::MIDIObjectAddRemoveNotification>());
                Ok(Self::ObjectAdded {
                    parent: (typed.parent != 0).then_some(typed.parent),
                    parent_type: (typed.parent != 0)
                        .then(|| MidiObjectType::from_raw(typed.parentType)),
                    child: typed.child,
                    child_type: MidiObjectType::from_raw(typed.childType),
                })
            }
            Some(NotificationMessageId::ObjectRemoved) => {
                let typed = &*(message.cast::<ffi::MIDIObjectAddRemoveNotification>());
                Ok(Self::ObjectRemoved {
                    parent: (typed.parent != 0).then_some(typed.parent),
                    parent_type: (typed.parent != 0)
                        .then(|| MidiObjectType::from_raw(typed.parentType)),
                    child: typed.child,
                    child_type: MidiObjectType::from_raw(typed.childType),
                })
            }
            Some(NotificationMessageId::PropertyChanged) => {
                let typed = &*(message.cast::<ffi::MIDIObjectPropertyChangeNotification>());
                let property_name = crate::cf::string_from_cfstring(typed.propertyName)?;
                Ok(Self::PropertyChanged {
                    object: typed.object,
                    object_type: MidiObjectType::from_raw(typed.objectType),
                    property_name,
                })
            }
            Some(NotificationMessageId::ThruConnectionsChanged) => Ok(Self::ThruConnectionsChanged),
            Some(NotificationMessageId::SerialPortOwnerChanged) => Ok(Self::SerialPortOwnerChanged),
            Some(NotificationMessageId::IoError) => {
                let typed = &*(message.cast::<ffi::MIDIIOErrorNotification>());
                Ok(Self::IoError {
                    driver_device: typed.driverDevice,
                    error_code: typed.errorCode,
                })
            }
            Some(NotificationMessageId::InternalStart) | None => Ok(Self::Unknown {
                message_id: message_ref.messageID,
                message_size: message_ref.messageSize,
            }),
        }
    }

    fn from_payload(payload: NotificationPayload) -> MidiResult<Self> {
        match NotificationMessageId::from_raw(payload.message_id) {
            Some(NotificationMessageId::SetupChanged) => Ok(Self::SetupChanged),
            Some(NotificationMessageId::ObjectAdded) => Ok(Self::ObjectAdded {
                parent: payload.parent,
                parent_type: payload.parent_type.map(MidiObjectType::from_raw),
                child: payload.child.ok_or_else(|| {
                    MidiError::Serialization("missing child object in notification payload".into())
                })?,
                child_type: MidiObjectType::from_raw(payload.child_type.ok_or_else(|| {
                    MidiError::Serialization("missing child type in notification payload".into())
                })?),
            }),
            Some(NotificationMessageId::ObjectRemoved) => Ok(Self::ObjectRemoved {
                parent: payload.parent,
                parent_type: payload.parent_type.map(MidiObjectType::from_raw),
                child: payload.child.ok_or_else(|| {
                    MidiError::Serialization("missing child object in notification payload".into())
                })?,
                child_type: MidiObjectType::from_raw(payload.child_type.ok_or_else(|| {
                    MidiError::Serialization("missing child type in notification payload".into())
                })?),
            }),
            Some(NotificationMessageId::PropertyChanged) => Ok(Self::PropertyChanged {
                object: payload.object.ok_or_else(|| {
                    MidiError::Serialization("missing object in property change payload".into())
                })?,
                object_type: MidiObjectType::from_raw(payload.object_type.ok_or_else(|| {
                    MidiError::Serialization(
                        "missing object type in property change payload".into(),
                    )
                })?),
                property_name: payload.property_name.unwrap_or_default(),
            }),
            Some(NotificationMessageId::ThruConnectionsChanged) => Ok(Self::ThruConnectionsChanged),
            Some(NotificationMessageId::SerialPortOwnerChanged) => Ok(Self::SerialPortOwnerChanged),
            Some(NotificationMessageId::IoError) => Ok(Self::IoError {
                driver_device: payload.driver_device.unwrap_or_default(),
                error_code: payload.error_code.unwrap_or_default(),
            }),
            Some(NotificationMessageId::InternalStart) | None => Ok(Self::Unknown {
                message_id: payload.message_id,
                message_size: payload.message_size,
            }),
        }
    }
}

use core::mem::MaybeUninit;
use std::ptr;

use crate::error::{MidiError, MidiResult};
use crate::ffi;
use crate::private;

extern "C" {
    fn cmr_thru_connection_create(
        owner_id: *const core::ffi::c_char,
        bytes: *const u8,
        length: usize,
        out_connection: *mut ffi::MIDIThruConnectionRef,
        error_out: *mut *mut core::ffi::c_char,
    ) -> i32;
    fn cmr_thru_connection_dispose(
        connection: ffi::MIDIThruConnectionRef,
        error_out: *mut *mut core::ffi::c_char,
    ) -> i32;
    fn cmr_thru_connection_get_params(
        connection: ffi::MIDIThruConnectionRef,
        out_bytes: *mut *mut u8,
        out_len: *mut usize,
        error_out: *mut *mut core::ffi::c_char,
    ) -> i32;
    fn cmr_thru_connection_set_params(
        connection: ffi::MIDIThruConnectionRef,
        bytes: *const u8,
        length: usize,
        error_out: *mut *mut core::ffi::c_char,
    ) -> i32;
    fn cmr_thru_connection_find(
        owner_id: *const core::ffi::c_char,
        out_bytes: *mut *mut u8,
        out_len: *mut usize,
        error_out: *mut *mut core::ffi::c_char,
    ) -> i32;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum MidiTransformKind {
    None = ffi::kMIDITransform_None,
    FilterOut = ffi::kMIDITransform_FilterOut,
    MapControl = ffi::kMIDITransform_MapControl,
    Add = ffi::kMIDITransform_Add,
    Scale = ffi::kMIDITransform_Scale,
    MinValue = ffi::kMIDITransform_MinValue,
    MaxValue = ffi::kMIDITransform_MaxValue,
    MapValue = ffi::kMIDITransform_MapValue,
}

impl MidiTransformKind {
    #[must_use]
    pub const fn from_raw(raw: u16) -> Option<Self> {
        match raw {
            ffi::kMIDITransform_None => Some(Self::None),
            ffi::kMIDITransform_FilterOut => Some(Self::FilterOut),
            ffi::kMIDITransform_MapControl => Some(Self::MapControl),
            ffi::kMIDITransform_Add => Some(Self::Add),
            ffi::kMIDITransform_Scale => Some(Self::Scale),
            ffi::kMIDITransform_MinValue => Some(Self::MinValue),
            ffi::kMIDITransform_MaxValue => Some(Self::MaxValue),
            ffi::kMIDITransform_MapValue => Some(Self::MapValue),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MidiControlType {
    SevenBit = ffi::kMIDIControlType_7Bit,
    FourteenBit = ffi::kMIDIControlType_14Bit,
    SevenBitRpn = ffi::kMIDIControlType_7BitRPN,
    FourteenBitRpn = ffi::kMIDIControlType_14BitRPN,
    SevenBitNrpn = ffi::kMIDIControlType_7BitNRPN,
    FourteenBitNrpn = ffi::kMIDIControlType_14BitNRPN,
}

impl MidiControlType {
    #[must_use]
    pub const fn from_raw(raw: u8) -> Option<Self> {
        match raw {
            ffi::kMIDIControlType_7Bit => Some(Self::SevenBit),
            ffi::kMIDIControlType_14Bit => Some(Self::FourteenBit),
            ffi::kMIDIControlType_7BitRPN => Some(Self::SevenBitRpn),
            ffi::kMIDIControlType_14BitRPN => Some(Self::FourteenBitRpn),
            ffi::kMIDIControlType_7BitNRPN => Some(Self::SevenBitNrpn),
            ffi::kMIDIControlType_14BitNRPN => Some(Self::FourteenBitNrpn),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MidiTransform {
    pub kind: MidiTransformKind,
    pub param: i16,
}

impl MidiTransform {
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            kind: MidiTransformKind::None,
            param: 0,
        }
    }

    #[must_use]
    pub const fn into_raw(self) -> ffi::MIDITransform {
        ffi::MIDITransform {
            transform: self.kind as u16,
            param: self.param,
        }
    }

    pub fn from_raw(raw: ffi::MIDITransform) -> MidiResult<Self> {
        Ok(Self {
            kind: MidiTransformKind::from_raw(raw.transform)
                .ok_or_else(|| MidiError::Bridge(format!("unknown MIDITransform kind {}", raw.transform)))?,
            param: raw.param,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MidiControlTransform {
    pub control_type: MidiControlType,
    pub remapped_control_type: MidiControlType,
    pub control_number: u16,
    pub transform: MidiTransform,
}

impl MidiControlTransform {
    pub fn from_raw(raw: ffi::MIDIControlTransform) -> MidiResult<Self> {
        Ok(Self {
            control_type: MidiControlType::from_raw(raw.controlType)
                .ok_or_else(|| MidiError::Bridge(format!("unknown MIDIControlType {}", raw.controlType)))?,
            remapped_control_type: MidiControlType::from_raw(raw.remappedControlType).ok_or_else(|| {
                MidiError::Bridge(format!("unknown MIDIControlType {}", raw.remappedControlType))
            })?,
            control_number: raw.controlNumber,
            transform: MidiTransform::from_raw(ffi::MIDITransform {
                transform: raw.transform,
                param: raw.param,
            })?,
        })
    }

    #[must_use]
    pub const fn into_raw(self) -> ffi::MIDIControlTransform {
        ffi::MIDIControlTransform {
            controlType: self.control_type as u8,
            remappedControlType: self.remapped_control_type as u8,
            controlNumber: self.control_number,
            transform: self.transform.kind as u16,
            param: self.transform.param,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThruConnectionEndpoint {
    pub endpoint_ref: ffi::MIDIEndpointRef,
    pub unique_id: ffi::MIDIUniqueID,
}

impl ThruConnectionEndpoint {
    #[must_use]
    pub const fn from_raw(raw: ffi::MIDIThruConnectionEndpoint) -> Self {
        Self {
            endpoint_ref: raw.endpointRef,
            unique_id: raw.uniqueID,
        }
    }

    #[must_use]
    pub const fn into_raw(self) -> ffi::MIDIThruConnectionEndpoint {
        ffi::MIDIThruConnectionEndpoint {
            endpointRef: self.endpoint_ref,
            uniqueID: self.unique_id,
        }
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThruConnectionParams {
    pub version: u32,
    pub sources: Vec<ThruConnectionEndpoint>,
    pub destinations: Vec<ThruConnectionEndpoint>,
    pub channel_map: [u8; 16],
    pub low_velocity: u8,
    pub high_velocity: u8,
    pub low_note: u8,
    pub high_note: u8,
    pub note_number: MidiTransform,
    pub velocity: MidiTransform,
    pub key_pressure: MidiTransform,
    pub channel_pressure: MidiTransform,
    pub program_change: MidiTransform,
    pub pitch_bend: MidiTransform,
    pub filter_out_sysex: bool,
    pub filter_out_mtc: bool,
    pub filter_out_beat_clock: bool,
    pub filter_out_tune_request: bool,
    pub filter_out_all_controls: bool,
    pub control_transforms: Vec<MidiControlTransform>,
    pub maps: Vec<u16>,
}

impl Default for ThruConnectionParams {
    fn default() -> Self {
        let mut raw = MaybeUninit::<ffi::MIDIThruConnectionParams>::zeroed();
        unsafe {
            ffi::MIDIThruConnectionParamsInitialize(raw.as_mut_ptr());
            Self::from_bytes(std::slice::from_raw_parts(
                raw.as_ptr().cast::<u8>(),
                core::mem::size_of::<ffi::MIDIThruConnectionParams>(),
            ))
            .expect("MIDIThruConnectionParamsInitialize produced invalid params")
        }
    }
}

impl ThruConnectionParams {
    pub fn from_bytes(bytes: &[u8]) -> MidiResult<Self> {
        if bytes.len() < core::mem::size_of::<ffi::MIDIThruConnectionParams>() {
            return Err(MidiError::Bridge("thru connection parameter blob is too small".into()));
        }

        let mut base = MaybeUninit::<ffi::MIDIThruConnectionParams>::zeroed();
        unsafe {
            ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                base.as_mut_ptr().cast::<u8>(),
                core::mem::size_of::<ffi::MIDIThruConnectionParams>(),
            );
            let base = base.assume_init();

            let source_count = usize::try_from(base.numSources)
                .unwrap_or(usize::MAX)
                .min(ffi::kMIDIThruConnection_MaxEndpoints);
            let destination_count = usize::try_from(base.numDestinations)
                .unwrap_or(usize::MAX)
                .min(ffi::kMIDIThruConnection_MaxEndpoints);
            let control_count = usize::from(base.numControlTransforms);
            let map_count = usize::from(base.numMaps);

            let base_size = core::mem::size_of::<ffi::MIDIThruConnectionParams>();
            let tail = &bytes[base_size..];
            let control_size = core::mem::size_of::<ffi::MIDIControlTransform>();
            let control_bytes = control_count
                .checked_mul(control_size)
                .ok_or_else(|| MidiError::Bridge("control transform count overflow".into()))?;
            let map_size = core::mem::size_of::<u16>();
            let map_bytes = map_count
                .checked_mul(map_size)
                .ok_or_else(|| MidiError::Bridge("map count overflow".into()))?;
            if bytes.len() < base_size + control_bytes + map_bytes {
                return Err(MidiError::Bridge("thru connection parameter blob is truncated".into()));
            }

            let raw_controls = tail[..control_bytes]
                .chunks_exact(control_size)
                .map(|chunk| {
                    let mut raw = MaybeUninit::<ffi::MIDIControlTransform>::zeroed();
                    ptr::copy_nonoverlapping(chunk.as_ptr(), raw.as_mut_ptr().cast::<u8>(), control_size);
                    raw.assume_init()
                })
                .collect::<Vec<_>>();
            let raw_maps = tail[control_bytes..control_bytes + map_bytes]
                .chunks_exact(map_size)
                .map(|chunk| {
                    let mut raw = 0_u16;
                    ptr::copy_nonoverlapping(
                        chunk.as_ptr(),
                        std::ptr::addr_of_mut!(raw).cast::<u8>(),
                        map_size,
                    );
                    raw
                })
                .collect::<Vec<_>>();

            Ok(Self {
                version: base.version,
                sources: base.sources[..source_count]
                    .iter()
                    .copied()
                    .map(ThruConnectionEndpoint::from_raw)
                    .collect(),
                destinations: base.destinations[..destination_count]
                    .iter()
                    .copied()
                    .map(ThruConnectionEndpoint::from_raw)
                    .collect(),
                channel_map: base.channelMap,
                low_velocity: base.lowVelocity,
                high_velocity: base.highVelocity,
                low_note: base.lowNote,
                high_note: base.highNote,
                note_number: MidiTransform::from_raw(base.noteNumber)?,
                velocity: MidiTransform::from_raw(base.velocity)?,
                key_pressure: MidiTransform::from_raw(base.keyPressure)?,
                channel_pressure: MidiTransform::from_raw(base.channelPressure)?,
                program_change: MidiTransform::from_raw(base.programChange)?,
                pitch_bend: MidiTransform::from_raw(base.pitchBend)?,
                filter_out_sysex: base.filterOutSysEx != 0,
                filter_out_mtc: base.filterOutMTC != 0,
                filter_out_beat_clock: base.filterOutBeatClock != 0,
                filter_out_tune_request: base.filterOutTuneRequest != 0,
                filter_out_all_controls: base.filterOutAllControls != 0,
                control_transforms: raw_controls
                    .iter()
                    .copied()
                    .map(MidiControlTransform::from_raw)
                    .collect::<MidiResult<_>>()?,
                maps: raw_maps,
            })
        }
    }

    pub fn to_bytes(&self) -> MidiResult<Vec<u8>> {
        if self.sources.len() > ffi::kMIDIThruConnection_MaxEndpoints {
            return Err(MidiError::Bridge("too many thru connection sources".into()));
        }
        if self.destinations.len() > ffi::kMIDIThruConnection_MaxEndpoints {
            return Err(MidiError::Bridge("too many thru connection destinations".into()));
        }

        let num_sources = u32::try_from(self.sources.len())
            .map_err(|_| MidiError::Bridge("too many thru connection sources".into()))?;
        let num_destinations = u32::try_from(self.destinations.len())
            .map_err(|_| MidiError::Bridge("too many thru connection destinations".into()))?;
        let num_control_transforms = u16::try_from(self.control_transforms.len())
            .map_err(|_| MidiError::Bridge("too many thru control transforms".into()))?;
        let num_maps = u16::try_from(self.maps.len())
            .map_err(|_| MidiError::Bridge("too many thru maps".into()))?;

        let mut raw = MaybeUninit::<ffi::MIDIThruConnectionParams>::zeroed();
        unsafe {
            ffi::MIDIThruConnectionParamsInitialize(raw.as_mut_ptr());
            let raw = &mut *raw.as_mut_ptr();
            raw.version = self.version;
            raw.numSources = num_sources;
            raw.numDestinations = num_destinations;
            for (slot, source) in raw.sources.iter_mut().zip(self.sources.iter().copied()) {
                *slot = source.into_raw();
            }
            for (slot, destination) in raw.destinations.iter_mut().zip(self.destinations.iter().copied()) {
                *slot = destination.into_raw();
            }
            raw.channelMap = self.channel_map;
            raw.lowVelocity = self.low_velocity;
            raw.highVelocity = self.high_velocity;
            raw.lowNote = self.low_note;
            raw.highNote = self.high_note;
            raw.noteNumber = self.note_number.into_raw();
            raw.velocity = self.velocity.into_raw();
            raw.keyPressure = self.key_pressure.into_raw();
            raw.channelPressure = self.channel_pressure.into_raw();
            raw.programChange = self.program_change.into_raw();
            raw.pitchBend = self.pitch_bend.into_raw();
            raw.filterOutSysEx = u8::from(self.filter_out_sysex);
            raw.filterOutMTC = u8::from(self.filter_out_mtc);
            raw.filterOutBeatClock = u8::from(self.filter_out_beat_clock);
            raw.filterOutTuneRequest = u8::from(self.filter_out_tune_request);
            raw.filterOutAllControls = u8::from(self.filter_out_all_controls);
            raw.numControlTransforms = num_control_transforms;
            raw.numMaps = num_maps;

            let base_size = core::mem::size_of::<ffi::MIDIThruConnectionParams>();
            let mut bytes = Vec::with_capacity(
                base_size
                    + self.control_transforms.len() * core::mem::size_of::<ffi::MIDIControlTransform>()
                    + self.maps.len() * core::mem::size_of::<u16>(),
            );
            bytes.extend_from_slice(std::slice::from_raw_parts(
                std::ptr::from_mut(raw).cast::<u8>(),
                base_size,
            ));
            for transform in self.control_transforms.iter().copied() {
                let raw_transform = transform.into_raw();
                bytes.extend_from_slice(std::slice::from_raw_parts(
                    std::ptr::addr_of!(raw_transform).cast::<u8>(),
                    core::mem::size_of::<ffi::MIDIControlTransform>(),
                ));
            }
            for map in &self.maps {
                bytes.extend_from_slice(std::slice::from_raw_parts(
                    std::ptr::from_ref(map).cast::<u8>(),
                    core::mem::size_of::<u16>(),
                ));
            }
            Ok(bytes)
        }
    }
}

#[derive(Debug)]
pub struct ThruConnection {
    raw: ffi::MIDIThruConnectionRef,
}

impl ThruConnection {
    pub fn create(owner_id: Option<&str>, params: &ThruConnectionParams) -> MidiResult<Self> {
        let owner_id = owner_id.map(private::to_cstring).transpose()?;
        let bytes = params.to_bytes()?;
        let mut raw = 0;
        let mut error = ptr::null_mut();
        unsafe {
            private::swift_result(
                cmr_thru_connection_create(
                    owner_id.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
                    bytes.as_ptr(),
                    bytes.len(),
                    &mut raw,
                    &mut error,
                ),
                error,
            )?;
        }
        Ok(Self { raw })
    }

    pub fn params(&self) -> MidiResult<ThruConnectionParams> {
        let mut out_bytes = ptr::null_mut();
        let mut out_len = 0;
        let mut error = ptr::null_mut();
        unsafe {
            private::swift_result(cmr_thru_connection_get_params(self.raw, &mut out_bytes, &mut out_len, &mut error), error)?;
            let bytes = private::take_bytes(out_bytes, out_len);
            ThruConnectionParams::from_bytes(&bytes)
        }
    }

    pub fn set_params(&self, params: &ThruConnectionParams) -> MidiResult<()> {
        let bytes = params.to_bytes()?;
        let mut error = ptr::null_mut();
        unsafe {
            private::swift_result(
                cmr_thru_connection_set_params(self.raw, bytes.as_ptr(), bytes.len(), &mut error),
                error,
            )
        }
    }

    pub fn find(owner_id: &str) -> MidiResult<Vec<ffi::MIDIThruConnectionRef>> {
        let owner_id = private::to_cstring(owner_id)?;
        let mut out_bytes = ptr::null_mut();
        let mut out_len = 0;
        let mut error = ptr::null_mut();
        unsafe {
            private::swift_result(
                cmr_thru_connection_find(owner_id.as_ptr(), &mut out_bytes, &mut out_len, &mut error),
                error,
            )?;
            let bytes = private::take_bytes(out_bytes, out_len);
            if bytes.len() % core::mem::size_of::<ffi::MIDIThruConnectionRef>() != 0 {
                return Err(MidiError::Bridge("thru connection find blob is not connection-ref aligned".into()));
            }
            Ok(bytes
                .chunks_exact(core::mem::size_of::<ffi::MIDIThruConnectionRef>())
                .map(|chunk| {
                    let mut raw = 0;
                    ptr::copy_nonoverlapping(
                        chunk.as_ptr(),
                        std::ptr::addr_of_mut!(raw).cast::<u8>(),
                        core::mem::size_of::<ffi::MIDIThruConnectionRef>(),
                    );
                    raw
                })
                .collect())
        }
    }

    #[must_use]
    pub const fn raw(&self) -> ffi::MIDIThruConnectionRef {
        self.raw
    }
}

impl Drop for ThruConnection {
    fn drop(&mut self) {
        let mut error = ptr::null_mut();
        let _ = unsafe { private::swift_result(cmr_thru_connection_dispose(self.raw, &mut error), error) };
    }
}

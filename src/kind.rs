//! Defines the common used enums.

mod camera_info;
mod color_scheme;
mod extension;
mod format;
mod frame_metadata;
mod hole_filling;
mod option;
mod persistence_control;
mod prelude;
mod stream_kind;
mod timestamp_domain;

pub use camera_info::Rs2CameraInfo;
pub use color_scheme::ColorScheme;
pub use extension::{
    Rs2Extension, DEVICE_EXTENSIONS, FILTER_EXTENSIONS, FRAME_EXTENSIONS, MISC_EXTENSIONS,
    PROFILE_EXTENSIONS, SENSOR_EXTENSIONS,
};
pub use format::Rs2Format;
pub use frame_metadata::Rs2FrameMetadata;
pub use hole_filling::HoleFillingMode;
pub use option::{OptionSetError, Rs2Option, Rs2OptionRange};
pub use persistence_control::PersistenceControl;
pub use prelude::*;
pub use stream_kind::Rs2StreamKind;
pub use timestamp_domain::Rs2TimestampDomain;

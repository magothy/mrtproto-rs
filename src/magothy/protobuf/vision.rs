// @generated
// This file is @generated by prost-build.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Color {
    /// \[0, 1\]
    #[prost(float, tag="1")]
    pub red: f32,
    /// \[0, 1\]
    #[prost(float, tag="2")]
    pub green: f32,
    /// \[0, 1\]
    #[prost(float, tag="3")]
    pub blue: f32,
    /// \[0, 1\], unset = 1
    #[prost(message, optional, tag="4")]
    pub alpha: ::core::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FramePrediction {
    /// top left
    #[prost(float, tag="1")]
    pub x: f32,
    /// top left
    #[prost(float, tag="2")]
    pub y: f32,
    #[prost(float, tag="3")]
    pub width: f32,
    #[prost(float, tag="4")]
    pub height: f32,
    #[prost(float, tag="5")]
    pub confidence: f32,
    #[prost(int32, tag="6")]
    pub class_id: i32,
    #[prost(string, tag="7")]
    pub class_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="8")]
    pub class_color: ::core::option::Option<Color>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Frame {
    #[prost(uint64, tag="1")]
    pub frame_number: u64,
    #[prost(message, optional, tag="2")]
    pub ttag_system: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub ttag_steady_ns: u64,
    #[prost(uint32, tag="4")]
    pub prediction_duration_ms: u32,
    #[prost(message, repeated, tag="5")]
    pub predictions: ::prost::alloc::vec::Vec<FramePrediction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OccupancyMap {
    #[prost(message, optional, tag="1")]
    pub ttag_system: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="2")]
    pub ttag_steady_ns: u64,
    #[prost(enumeration="occupancy_map::Type", tag="3")]
    pub r#type: i32,
    /// center of map
    #[prost(double, tag="4")]
    pub lat_deg: f64,
    /// center of map
    #[prost(double, tag="5")]
    pub lon_deg: f64,
    #[prost(double, tag="6")]
    pub width_m: f64,
    #[prost(double, tag="7")]
    pub height_m: f64,
    #[prost(uint32, tag="8")]
    pub width_px: u32,
    #[prost(uint32, tag="9")]
    pub height_px: u32,
    /// size == width_px * height_px, row-major order
    /// first pixel is top-left
    /// 1 byte per pixel, 0-255 belief
    /// 0 = no belief, 255 = full belief
    #[prost(bytes="vec", tag="10")]
    pub grid: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `OccupancyMap`.
pub mod occupancy_map {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Unknown = 0,
        Empty = 1,
        Occupied = 2,
        Merged = 3,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unknown => "UNKNOWN",
                Type::Empty => "EMPTY",
                Type::Occupied => "OCCUPIED",
                Type::Merged => "MERGED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "EMPTY" => Some(Self::Empty),
                "OCCUPIED" => Some(Self::Occupied),
                "MERGED" => Some(Self::Merged),
                _ => None,
            }
        }
    }
}
// @@protoc_insertion_point(module)

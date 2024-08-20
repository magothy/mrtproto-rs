// @generated
// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Spokes {
    #[prost(uint32, tag="1")]
    pub num_spoke: u32,
    #[prost(uint32, tag="2")]
    pub first_spoke_index: u32,
    #[prost(float, tag="3")]
    pub range_m: f32,
    #[prost(bytes="vec", repeated, tag="4")]
    pub spokes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Info {
    #[prost(message, optional, tag="1")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(enumeration="State", tag="2")]
    pub state: i32,
    #[prost(int32, tag="3")]
    pub gain: i32,
    #[prost(int32, tag="4")]
    pub rain: i32,
    #[prost(int32, tag="5")]
    pub sea: i32,
    #[prost(int32, tag="6")]
    pub range: i32,
    #[prost(int32, tag="7")]
    pub scan_speed: i32,
    #[prost(message, optional, tag="8")]
    pub spokes: ::core::option::Option<Spokes>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Command {
    #[prost(enumeration="CommandType", tag="1")]
    pub command: i32,
    #[prost(int64, tag="2")]
    pub value: i64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
    Off = 0,
    Standby = 1,
    WarmingUp = 2,
    TimedIdle = 3,
    Stopping = 4,
    SpinningDown = 5,
    Starting = 6,
    SpinningUp = 7,
    Transmit = 8,
}
impl State {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            State::Off => "STATE_OFF",
            State::Standby => "STATE_STANDBY",
            State::WarmingUp => "STATE_WARMING_UP",
            State::TimedIdle => "STATE_TIMED_IDLE",
            State::Stopping => "STATE_STOPPING",
            State::SpinningDown => "STATE_SPINNING_DOWN",
            State::Starting => "STATE_STARTING",
            State::SpinningUp => "STATE_SPINNING_UP",
            State::Transmit => "STATE_TRANSMIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATE_OFF" => Some(Self::Off),
            "STATE_STANDBY" => Some(Self::Standby),
            "STATE_WARMING_UP" => Some(Self::WarmingUp),
            "STATE_TIMED_IDLE" => Some(Self::TimedIdle),
            "STATE_STOPPING" => Some(Self::Stopping),
            "STATE_SPINNING_DOWN" => Some(Self::SpinningDown),
            "STATE_STARTING" => Some(Self::Starting),
            "STATE_SPINNING_UP" => Some(Self::SpinningUp),
            "STATE_TRANSMIT" => Some(Self::Transmit),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CommandType {
    CommandNone = 0,
    CommandTurnOn = 1,
    CommandTurnOff = 2,
    CommandSetRange = 3,
    CommandSetGain = 4,
}
impl CommandType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CommandType::CommandNone => "COMMAND_NONE",
            CommandType::CommandTurnOn => "COMMAND_TURN_ON",
            CommandType::CommandTurnOff => "COMMAND_TURN_OFF",
            CommandType::CommandSetRange => "COMMAND_SET_RANGE",
            CommandType::CommandSetGain => "COMMAND_SET_GAIN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COMMAND_NONE" => Some(Self::CommandNone),
            "COMMAND_TURN_ON" => Some(Self::CommandTurnOn),
            "COMMAND_TURN_OFF" => Some(Self::CommandTurnOff),
            "COMMAND_SET_RANGE" => Some(Self::CommandSetRange),
            "COMMAND_SET_GAIN" => Some(Self::CommandSetGain),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)

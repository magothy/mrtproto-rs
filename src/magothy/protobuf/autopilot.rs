// @generated
// This file is @generated by prost-build.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(double, tag="1")]
    pub x: f64,
    #[prost(double, tag="2")]
    pub y: f64,
    #[prost(double, tag="3")]
    pub z: f64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Position {
    #[prost(double, tag="1")]
    pub latitude_deg: f64,
    #[prost(double, tag="2")]
    pub longitude_deg: f64,
    /// x = lon, y = lat, z = alt
    #[prost(message, optional, tag="3")]
    pub error_m: ::core::option::Option<Error>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct AngularRate {
    #[prost(double, tag="1")]
    pub x_dps: f64,
    #[prost(double, tag="2")]
    pub y_dps: f64,
    #[prost(double, tag="3")]
    pub z_dps: f64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Acceleration {
    #[prost(double, tag="1")]
    pub x_mps2: f64,
    #[prost(double, tag="2")]
    pub y_mps2: f64,
    #[prost(double, tag="3")]
    pub z_mps2: f64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MagneticField {
    #[prost(double, tag="1")]
    pub x_gauss: f64,
    #[prost(double, tag="2")]
    pub y_gauss: f64,
    #[prost(double, tag="3")]
    pub z_gauss: f64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Euler {
    #[prost(double, tag="1")]
    pub roll_deg: f64,
    #[prost(double, tag="2")]
    pub pitch_deg: f64,
    #[prost(double, tag="3")]
    pub heading_deg: f64,
    /// x = roll, y = pitch, z = heading
    #[prost(message, optional, tag="4")]
    pub error_deg: ::core::option::Option<Error>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct BodyVelocity {
    #[prost(double, tag="1")]
    pub x_mps: f64,
    #[prost(double, tag="2")]
    pub y_mps: f64,
    #[prost(double, tag="3")]
    pub z_mps: f64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct InertialVelocity {
    #[prost(double, tag="1")]
    pub north_mps: f64,
    #[prost(double, tag="2")]
    pub east_mps: f64,
    #[prost(double, tag="3")]
    pub down_mps: f64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct WaterCurrent {
    #[prost(double, tag="1")]
    pub speed_mps: f64,
    #[prost(double, tag="2")]
    pub direction_deg: f64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct FuelGauge {
    #[prost(double, tag="1")]
    pub voltage_v: f64,
    #[prost(double, tag="2")]
    pub current_a: f64,
    #[prost(double, tag="3")]
    pub state_of_charge_percent: f64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct VehicleData {
    #[prost(message, optional, tag="1")]
    pub position: ::core::option::Option<Position>,
    #[prost(enumeration="vehicle_data::DataSource", tag="2")]
    pub position_source: i32,
    #[prost(uint64, tag="3")]
    pub position_ttag_ns_steady: u64,
    #[prost(message, optional, tag="4")]
    pub attitude: ::core::option::Option<Euler>,
    #[prost(enumeration="vehicle_data::DataSource", tag="5")]
    pub attitude_source: i32,
    #[prost(uint64, tag="6")]
    pub attitude_ttag_ns_steady: u64,
    #[prost(message, optional, tag="7")]
    pub angular_rate: ::core::option::Option<AngularRate>,
    #[prost(enumeration="vehicle_data::DataSource", tag="8")]
    pub angular_rate_source: i32,
    #[prost(uint64, tag="9")]
    pub angular_rate_ttag_ns_steady: u64,
    #[prost(message, optional, tag="10")]
    pub acceleration: ::core::option::Option<Acceleration>,
    #[prost(enumeration="vehicle_data::DataSource", tag="11")]
    pub acceleration_source: i32,
    #[prost(uint64, tag="12")]
    pub acceleration_ttag_ns_steady: u64,
    #[prost(message, optional, tag="13")]
    pub magnetic_field: ::core::option::Option<MagneticField>,
    #[prost(enumeration="vehicle_data::DataSource", tag="14")]
    pub magnetic_field_source: i32,
    #[prost(uint64, tag="15")]
    pub magnetic_field_ttag_ns_steady: u64,
    #[prost(double, optional, tag="16")]
    pub depth_m: ::core::option::Option<f64>,
    #[prost(enumeration="vehicle_data::DataSource", tag="17")]
    pub depth_source: i32,
    #[prost(uint64, tag="18")]
    pub depth_ttag_ns_steady: u64,
    #[prost(double, optional, tag="19")]
    pub altitude_m: ::core::option::Option<f64>,
    #[prost(enumeration="vehicle_data::DataSource", tag="20")]
    pub altitude_source: i32,
    #[prost(uint64, tag="21")]
    pub altitude_ttag_ns_steady: u64,
    #[prost(double, optional, tag="22")]
    pub speed_mps: ::core::option::Option<f64>,
    #[prost(enumeration="vehicle_data::DataSource", tag="23")]
    pub speed_source: i32,
    #[prost(uint64, tag="24")]
    pub speed_ttag_ns_steady: u64,
    #[prost(double, optional, tag="25")]
    pub course_deg: ::core::option::Option<f64>,
    #[prost(enumeration="vehicle_data::DataSource", tag="26")]
    pub course_source: i32,
    #[prost(uint64, tag="27")]
    pub course_ttag_ns_steady: u64,
    #[prost(message, optional, tag="28")]
    pub body_velocity: ::core::option::Option<BodyVelocity>,
    #[prost(enumeration="vehicle_data::DataSource", tag="29")]
    pub body_velocity_source: i32,
    #[prost(uint64, tag="30")]
    pub body_velocity_ttag_ns_steady: u64,
    #[prost(message, optional, tag="31")]
    pub water_current: ::core::option::Option<WaterCurrent>,
    #[prost(enumeration="vehicle_data::DataSource", tag="32")]
    pub water_current_source: i32,
    #[prost(uint64, tag="33")]
    pub water_current_ttag_ns_steady: u64,
    #[prost(double, optional, tag="34")]
    pub water_relative_speed_mps: ::core::option::Option<f64>,
    #[prost(enumeration="vehicle_data::DataSource", tag="35")]
    pub water_relative_speed_source: i32,
    #[prost(uint64, tag="36")]
    pub water_relative_speed_ttag_ns_steady: u64,
    #[prost(message, optional, tag="37")]
    pub fuel_gauge: ::core::option::Option<FuelGauge>,
    #[prost(enumeration="vehicle_data::DataSource", tag="38")]
    pub fuel_gauge_source: i32,
    #[prost(uint64, tag="39")]
    pub fuel_gauge_ttag_ns_steady: u64,
}
/// Nested message and enum types in `VehicleData`.
pub mod vehicle_data {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DataSource {
        /// no data for a particular field
        DsNodata = 0,
        /// Inertial Measurement Unit
        DsImu = 1,
        /// Attitude and Heading Reference System
        DsAhrs = 2,
        /// GNSS (GPS, GLONASS, etc.)
        DsGps = 3,
        /// depth sensor
        DsDepth = 4,
        /// altitude sensor
        DsAltitude = 5,
        /// speed sensor
        DsSpeed = 6,
        /// estimation algorithm such as Kalman filter
        DsEstimation = 7,
        /// such as heading rate computed by differentiating heading
        DsComputation = 8,
        ///
        DsSimulation = 9,
        /// Inertial Navigation System
        DsIns = 10,
    }
    impl DataSource {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::DsNodata => "DS_NODATA",
                Self::DsImu => "DS_IMU",
                Self::DsAhrs => "DS_AHRS",
                Self::DsGps => "DS_GPS",
                Self::DsDepth => "DS_DEPTH",
                Self::DsAltitude => "DS_ALTITUDE",
                Self::DsSpeed => "DS_SPEED",
                Self::DsEstimation => "DS_ESTIMATION",
                Self::DsComputation => "DS_COMPUTATION",
                Self::DsSimulation => "DS_SIMULATION",
                Self::DsIns => "DS_INS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DS_NODATA" => Some(Self::DsNodata),
                "DS_IMU" => Some(Self::DsImu),
                "DS_AHRS" => Some(Self::DsAhrs),
                "DS_GPS" => Some(Self::DsGps),
                "DS_DEPTH" => Some(Self::DsDepth),
                "DS_ALTITUDE" => Some(Self::DsAltitude),
                "DS_SPEED" => Some(Self::DsSpeed),
                "DS_ESTIMATION" => Some(Self::DsEstimation),
                "DS_COMPUTATION" => Some(Self::DsComputation),
                "DS_SIMULATION" => Some(Self::DsSimulation),
                "DS_INS" => Some(Self::DsIns),
                _ => None,
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct AhrsIf {
    #[prost(message, optional, tag="1")]
    pub angular_rate: ::core::option::Option<AngularRate>,
    #[prost(message, optional, tag="2")]
    pub acceleration: ::core::option::Option<Acceleration>,
    #[prost(message, optional, tag="3")]
    pub magfield: ::core::option::Option<MagneticField>,
    #[prost(message, optional, tag="4")]
    pub euler: ::core::option::Option<Euler>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct AltitudeIf {
    /// use NaN to represent unset
    #[prost(double, optional, tag="1")]
    pub altitude_m: ::core::option::Option<f64>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct BatteryIf {
    /// use NaN to represent unset
    #[prost(double, optional, tag="1")]
    pub voltage_v: ::core::option::Option<f64>,
    /// use NaN to represent unset
    #[prost(double, optional, tag="2")]
    pub current_a: ::core::option::Option<f64>,
    /// use < 0 to represent unset
    #[prost(int32, optional, tag="3")]
    pub state_of_charge_percent: ::core::option::Option<i32>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DepthIf {
    /// use NaN to represent unset
    #[prost(double, optional, tag="1")]
    pub depth_m: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EffectorIf {
    #[prost(double, repeated, tag="1")]
    pub command: ::prost::alloc::vec::Vec<f64>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct InsIf {
    #[prost(message, optional, tag="1")]
    pub angular_rate: ::core::option::Option<AngularRate>,
    #[prost(message, optional, tag="2")]
    pub acceleration: ::core::option::Option<Acceleration>,
    #[prost(message, optional, tag="3")]
    pub magfield: ::core::option::Option<MagneticField>,
    #[prost(message, optional, tag="4")]
    pub euler: ::core::option::Option<Euler>,
    #[prost(message, optional, tag="5")]
    pub body_velocity: ::core::option::Option<BodyVelocity>,
    #[prost(message, optional, tag="6")]
    pub position: ::core::option::Option<Position>,
    #[prost(message, optional, tag="7")]
    pub inertial_velocity: ::core::option::Option<InertialVelocity>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GpsIf {
    #[prost(message, optional, tag="1")]
    pub rmc_data: ::core::option::Option<gps_if::RmcData>,
    #[prost(message, optional, tag="2")]
    pub gga_data: ::core::option::Option<gps_if::GgaData>,
    #[prost(message, optional, tag="3")]
    pub gst_data: ::core::option::Option<gps_if::GstData>,
}
/// Nested message and enum types in `GpsIf`.
pub mod gps_if {
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct RmcData {
        /// use NaN to represent unset
        #[prost(double, optional, tag="1")]
        pub latitude_deg: ::core::option::Option<f64>,
        /// use NaN to represent unset
        #[prost(double, optional, tag="2")]
        pub longitude_deg: ::core::option::Option<f64>,
        /// use NaN to represent unset
        #[prost(double, optional, tag="3")]
        pub ground_speed_kt: ::core::option::Option<f64>,
        /// use NaN to represent unset
        #[prost(double, optional, tag="4")]
        pub course_true_deg: ::core::option::Option<f64>,
    }
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct GgaData {
        /// use NaN to represent unset
        #[prost(double, optional, tag="1")]
        pub latitude_deg: ::core::option::Option<f64>,
        /// use NaN to represent unset
        #[prost(double, optional, tag="2")]
        pub longitude_deg: ::core::option::Option<f64>,
        /// use NaN to represent unset
        #[prost(double, optional, tag="3")]
        pub altitude_m: ::core::option::Option<f64>,
        /// use < 0 to represent unset
        #[prost(int32, optional, tag="4")]
        pub num_satellite: ::core::option::Option<i32>,
        #[prost(enumeration="gga_data::FixQuality", tag="5")]
        pub fix_quality: i32,
    }
    /// Nested message and enum types in `GgaData`.
    pub mod gga_data {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum FixQuality {
            FqInvalid = 0,
            FqGps = 1,
            FqDgps = 2,
            FqPps = 3,
            FqRtkFixed = 4,
            FqRtkFloat = 5,
            FqEstimated = 6,
            FqManual = 7,
            FqSimulation = 8,
        }
        impl FixQuality {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Self::FqInvalid => "FQ_INVALID",
                    Self::FqGps => "FQ_GPS",
                    Self::FqDgps => "FQ_DGPS",
                    Self::FqPps => "FQ_PPS",
                    Self::FqRtkFixed => "FQ_RTK_FIXED",
                    Self::FqRtkFloat => "FQ_RTK_FLOAT",
                    Self::FqEstimated => "FQ_ESTIMATED",
                    Self::FqManual => "FQ_MANUAL",
                    Self::FqSimulation => "FQ_SIMULATION",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "FQ_INVALID" => Some(Self::FqInvalid),
                    "FQ_GPS" => Some(Self::FqGps),
                    "FQ_DGPS" => Some(Self::FqDgps),
                    "FQ_PPS" => Some(Self::FqPps),
                    "FQ_RTK_FIXED" => Some(Self::FqRtkFixed),
                    "FQ_RTK_FLOAT" => Some(Self::FqRtkFloat),
                    "FQ_ESTIMATED" => Some(Self::FqEstimated),
                    "FQ_MANUAL" => Some(Self::FqManual),
                    "FQ_SIMULATION" => Some(Self::FqSimulation),
                    _ => None,
                }
            }
        }
    }
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct GstData {
        /// use NaN to represent unset
        #[prost(double, optional, tag="1")]
        pub std_latitude_error_m: ::core::option::Option<f64>,
        /// use NaN to represent unset
        #[prost(double, optional, tag="2")]
        pub std_longitude_error_m: ::core::option::Option<f64>,
        /// use NaN to represent unset
        #[prost(double, optional, tag="3")]
        pub std_altitude_error_m: ::core::option::Option<f64>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObstacleIf {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration="obstacle_if::ZoneType", tag="4")]
    pub zone_type: i32,
    #[prost(bool, tag="5")]
    pub is_stationary: bool,
    #[prost(double, tag="6")]
    pub lifespan_s: f64,
    #[prost(double, optional, tag="7")]
    pub course_deg: ::core::option::Option<f64>,
    #[prost(double, optional, tag="8")]
    pub speed_mps: ::core::option::Option<f64>,
    #[prost(message, optional, tag="9")]
    pub point_of_interest: ::core::option::Option<Position>,
    #[prost(oneof="obstacle_if::ObstacleOneOf", tags="2, 3")]
    pub obstacle_one_of: ::core::option::Option<obstacle_if::ObstacleOneOf>,
}
/// Nested message and enum types in `ObstacleIf`.
pub mod obstacle_if {
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Circle {
        #[prost(message, optional, tag="1")]
        pub origin: ::core::option::Option<super::Position>,
        #[prost(double, tag="2")]
        pub radius_m: f64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Polygon {
        #[prost(message, repeated, tag="1")]
        pub vertices: ::prost::alloc::vec::Vec<super::Position>,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ZoneType {
        ZoneKeepOut = 0,
        ZoneKeepIn = 1,
    }
    impl ZoneType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::ZoneKeepOut => "ZONE_KEEP_OUT",
                Self::ZoneKeepIn => "ZONE_KEEP_IN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ZONE_KEEP_OUT" => Some(Self::ZoneKeepOut),
                "ZONE_KEEP_IN" => Some(Self::ZoneKeepIn),
                _ => None,
            }
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ObstacleOneOf {
        #[prost(message, tag="2")]
        Circle(Circle),
        #[prost(message, tag="3")]
        Polygon(Polygon),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Obstacles {
    #[prost(message, optional, tag="1")]
    pub ttag_system: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="2")]
    pub ttag_steady_ns: u64,
    #[prost(message, repeated, tag="3")]
    pub obstacles: ::prost::alloc::vec::Vec<ObstacleIf>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Path {
    #[prost(message, optional, tag="1")]
    pub ttag_system: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="2")]
    pub ttag_steady_ns: u64,
    /// local path
    #[prost(message, repeated, tag="3")]
    pub path: ::prost::alloc::vec::Vec<Position>,
    #[prost(message, repeated, tag="4")]
    pub obstacles: ::prost::alloc::vec::Vec<ObstacleIf>,
    #[prost(double, optional, tag="5")]
    pub speed_mps: ::core::option::Option<f64>,
    #[prost(message, optional, tag="6")]
    pub start: ::core::option::Option<Position>,
    #[prost(message, optional, tag="7")]
    pub end_local: ::core::option::Option<Position>,
    #[prost(message, optional, tag="8")]
    pub end_global: ::core::option::Option<Position>,
    #[prost(message, repeated, tag="9")]
    pub global_path: ::prost::alloc::vec::Vec<Position>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VehicleStateIf {
    #[prost(uint64, tag="1")]
    pub ttag_ns: u64,
    #[prost(message, optional, tag="2")]
    pub vehicle_data: ::core::option::Option<VehicleData>,
    #[prost(enumeration="vehicle_state_if::Mode", tag="3")]
    pub mode: i32,
    #[prost(message, repeated, tag="4")]
    pub health_items: ::prost::alloc::vec::Vec<vehicle_state_if::HealthItem>,
    #[prost(message, optional, tag="5")]
    pub fault_response: ::core::option::Option<vehicle_state_if::FaultResponse>,
}
/// Nested message and enum types in `VehicleStateIf`.
pub mod vehicle_state_if {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HealthItem {
        /// also priority - smaller number is higher priority
        #[prost(uint32, tag="1")]
        pub index: u32,
        /// unique identifier
        #[prost(string, tag="2")]
        pub uid: ::prost::alloc::string::String,
        /// status id for mavlink
        #[prost(uint32, tag="3")]
        pub sensor_id: u32,
        /// true if properly configured, false otherwise
        #[prost(bool, tag="4")]
        pub is_valid: bool,
        /// true if enabled
        #[prost(bool, tag="5")]
        pub enabled: bool,
        /// true on healthy, false on faulted
        #[prost(bool, tag="6")]
        pub is_healthy: bool,
    }
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct FaultResponse {
        #[prost(enumeration="FaultResponseType", tag="1")]
        pub response_type: i32,
        #[prost(uint32, tag="2")]
        pub health_item_index: u32,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Mode {
        VsStandby = 0,
        VsManual = 1,
        VsHealthyMission = 2,
        VsUnhealthyMission = 3,
        VsLoiter = 4,
        VsMissionPlanning = 5,
        VsUnhealthyMissionPlanning = 6,
    }
    impl Mode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::VsStandby => "VS_STANDBY",
                Self::VsManual => "VS_MANUAL",
                Self::VsHealthyMission => "VS_HEALTHY_MISSION",
                Self::VsUnhealthyMission => "VS_UNHEALTHY_MISSION",
                Self::VsLoiter => "VS_LOITER",
                Self::VsMissionPlanning => "VS_MISSION_PLANNING",
                Self::VsUnhealthyMissionPlanning => "VS_UNHEALTHY_MISSION_PLANNING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VS_STANDBY" => Some(Self::VsStandby),
                "VS_MANUAL" => Some(Self::VsManual),
                "VS_HEALTHY_MISSION" => Some(Self::VsHealthyMission),
                "VS_UNHEALTHY_MISSION" => Some(Self::VsUnhealthyMission),
                "VS_LOITER" => Some(Self::VsLoiter),
                "VS_MISSION_PLANNING" => Some(Self::VsMissionPlanning),
                "VS_UNHEALTHY_MISSION_PLANNING" => Some(Self::VsUnhealthyMissionPlanning),
                _ => None,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FaultResponseType {
        FrIgnore = 0,
        FrHalt = 1,
        FrLoiter = 2,
        FrGoRally = 3,
        FrGoFirst = 4,
        FrGoLast = 5,
        FrGoLaunch = 6,
        FrCustom = 7,
    }
    impl FaultResponseType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::FrIgnore => "FR_IGNORE",
                Self::FrHalt => "FR_HALT",
                Self::FrLoiter => "FR_LOITER",
                Self::FrGoRally => "FR_GO_RALLY",
                Self::FrGoFirst => "FR_GO_FIRST",
                Self::FrGoLast => "FR_GO_LAST",
                Self::FrGoLaunch => "FR_GO_LAUNCH",
                Self::FrCustom => "FR_CUSTOM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FR_IGNORE" => Some(Self::FrIgnore),
                "FR_HALT" => Some(Self::FrHalt),
                "FR_LOITER" => Some(Self::FrLoiter),
                "FR_GO_RALLY" => Some(Self::FrGoRally),
                "FR_GO_FIRST" => Some(Self::FrGoFirst),
                "FR_GO_LAST" => Some(Self::FrGoLast),
                "FR_GO_LAUNCH" => Some(Self::FrGoLaunch),
                "FR_CUSTOM" => Some(Self::FrCustom),
                _ => None,
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MavlinkMissionItemInt {
    #[prost(uint32, tag="1")]
    pub target_system: u32,
    #[prost(uint32, tag="2")]
    pub target_component: u32,
    #[prost(uint32, tag="3")]
    pub mission_type: u32,
    #[prost(uint32, tag="4")]
    pub seq: u32,
    #[prost(uint32, tag="5")]
    pub command: u32,
    #[prost(uint32, tag="6")]
    pub frame: u32,
    #[prost(bool, tag="7")]
    pub current: bool,
    #[prost(bool, tag="8")]
    pub autocontinue: bool,
    #[prost(float, tag="9")]
    pub param1: f32,
    #[prost(float, tag="10")]
    pub param2: f32,
    #[prost(float, tag="11")]
    pub param3: f32,
    #[prost(float, tag="12")]
    pub param4: f32,
    #[prost(int32, tag="13")]
    pub x: i32,
    #[prost(int32, tag="14")]
    pub y: i32,
    #[prost(float, tag="15")]
    pub z: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MavlinkMission {
    #[prost(message, repeated, tag="1")]
    pub mission_items: ::prost::alloc::vec::Vec<MavlinkMissionItemInt>,
    #[prost(message, repeated, tag="2")]
    pub fence_items: ::prost::alloc::vec::Vec<MavlinkMissionItemInt>,
    #[prost(message, repeated, tag="3")]
    pub rally_items: ::prost::alloc::vec::Vec<MavlinkMissionItemInt>,
}
// @@protoc_insertion_point(module)

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to unitree_go__msg__AudioData

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioData {

    // This member is not documented.
    #[allow(missing_docs)]
    pub time_frame: u64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: Vec<u8>,

}



impl Default for AudioData {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::AudioData::default())
  }
}

impl rosidl_runtime_rs::Message for AudioData {
  type RmwMsg = super::msg::rmw::AudioData;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        time_frame: msg.time_frame,
        data: msg.data.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      time_frame: msg.time_frame,
        data: msg.data.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      time_frame: msg.time_frame,
      data: msg.data
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to unitree_go__msg__BmsCmd

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BmsCmd {

    // This member is not documented.
    #[allow(missing_docs)]
    pub off: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub reserve: [u8; 3],

}



impl Default for BmsCmd {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BmsCmd::default())
  }
}

impl rosidl_runtime_rs::Message for BmsCmd {
  type RmwMsg = super::msg::rmw::BmsCmd;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        off: msg.off,
        reserve: msg.reserve,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      off: msg.off,
        reserve: msg.reserve,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      off: msg.off,
      reserve: msg.reserve,
    }
  }
}


// Corresponds to unitree_go__msg__BmsState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BmsState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub version_high: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub version_low: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub soc: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cycle: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub bq_ntc: [i8; 2],


    // This member is not documented.
    #[allow(missing_docs)]
    pub mcu_ntc: [i8; 2],


    // This member is not documented.
    #[allow(missing_docs)]
    pub cell_vol: [u16; 15],

}



impl Default for BmsState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BmsState::default())
  }
}

impl rosidl_runtime_rs::Message for BmsState {
  type RmwMsg = super::msg::rmw::BmsState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        version_high: msg.version_high,
        version_low: msg.version_low,
        status: msg.status,
        soc: msg.soc,
        current: msg.current,
        cycle: msg.cycle,
        bq_ntc: msg.bq_ntc,
        mcu_ntc: msg.mcu_ntc,
        cell_vol: msg.cell_vol,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      version_high: msg.version_high,
      version_low: msg.version_low,
      status: msg.status,
      soc: msg.soc,
      current: msg.current,
      cycle: msg.cycle,
        bq_ntc: msg.bq_ntc,
        mcu_ntc: msg.mcu_ntc,
        cell_vol: msg.cell_vol,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      version_high: msg.version_high,
      version_low: msg.version_low,
      status: msg.status,
      soc: msg.soc,
      current: msg.current,
      cycle: msg.cycle,
      bq_ntc: msg.bq_ntc,
      mcu_ntc: msg.mcu_ntc,
      cell_vol: msg.cell_vol,
    }
  }
}


// Corresponds to unitree_go__msg__Error

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Error {

    // This member is not documented.
    #[allow(missing_docs)]
    pub source: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub state: u32,

}



impl Default for Error {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Error::default())
  }
}

impl rosidl_runtime_rs::Message for Error {
  type RmwMsg = super::msg::rmw::Error;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        source: msg.source,
        state: msg.state,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      source: msg.source,
      state: msg.state,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      source: msg.source,
      state: msg.state,
    }
  }
}


// Corresponds to unitree_go__msg__Go2FrontVideoData

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Go2FrontVideoData {

    // This member is not documented.
    #[allow(missing_docs)]
    pub time_frame: u64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub video720p: Vec<u8>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub video360p: Vec<u8>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub video180p: Vec<u8>,

}



impl Default for Go2FrontVideoData {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Go2FrontVideoData::default())
  }
}

impl rosidl_runtime_rs::Message for Go2FrontVideoData {
  type RmwMsg = super::msg::rmw::Go2FrontVideoData;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        time_frame: msg.time_frame,
        video720p: msg.video720p.into(),
        video360p: msg.video360p.into(),
        video180p: msg.video180p.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      time_frame: msg.time_frame,
        video720p: msg.video720p.as_slice().into(),
        video360p: msg.video360p.as_slice().into(),
        video180p: msg.video180p.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      time_frame: msg.time_frame,
      video720p: msg.video720p
          .into_iter()
          .collect(),
      video360p: msg.video360p
          .into_iter()
          .collect(),
      video180p: msg.video180p
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to unitree_go__msg__HeightMap
/// Header

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HeightMap {
    /// timestamp
    pub stamp: f64,

    /// world frame id
    pub frame_id: std::string::String,

    /// Map info
    /// The map resolution
    pub resolution: f32,

    /// Map width along x-axis
    pub width: u32,

    /// Map height alonge y-axis
    pub height: u32,

    /// Map frame origin xy-position, the xyz-axis direction of map frame is aligned with the world frame
    pub origin: [f32; 2],

    /// Map data, in x-major order, starting with [0,0], ending with [width, height]
    /// For a cell whose 2d-array-index is [ix, iy]，
    ///    its position in world frame is: [ix * resolution + origin[0], iy * resolution + origin[1]]
    ///    its cell value is: data[width * iy + ix]
    pub data: Vec<f32>,

}



impl Default for HeightMap {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::HeightMap::default())
  }
}

impl rosidl_runtime_rs::Message for HeightMap {
  type RmwMsg = super::msg::rmw::HeightMap;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        stamp: msg.stamp,
        frame_id: msg.frame_id.as_str().into(),
        resolution: msg.resolution,
        width: msg.width,
        height: msg.height,
        origin: msg.origin,
        data: msg.data.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      stamp: msg.stamp,
        frame_id: msg.frame_id.as_str().into(),
      resolution: msg.resolution,
      width: msg.width,
      height: msg.height,
        origin: msg.origin,
        data: msg.data.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      stamp: msg.stamp,
      frame_id: msg.frame_id.to_string(),
      resolution: msg.resolution,
      width: msg.width,
      height: msg.height,
      origin: msg.origin,
      data: msg.data
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to unitree_go__msg__IMUState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IMUState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub quaternion: [f32; 4],


    // This member is not documented.
    #[allow(missing_docs)]
    pub gyroscope: [f32; 3],


    // This member is not documented.
    #[allow(missing_docs)]
    pub accelerometer: [f32; 3],


    // This member is not documented.
    #[allow(missing_docs)]
    pub rpy: [f32; 3],


    // This member is not documented.
    #[allow(missing_docs)]
    pub temperature: i8,

}



impl Default for IMUState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::IMUState::default())
  }
}

impl rosidl_runtime_rs::Message for IMUState {
  type RmwMsg = super::msg::rmw::IMUState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        quaternion: msg.quaternion,
        gyroscope: msg.gyroscope,
        accelerometer: msg.accelerometer,
        rpy: msg.rpy,
        temperature: msg.temperature,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        quaternion: msg.quaternion,
        gyroscope: msg.gyroscope,
        accelerometer: msg.accelerometer,
        rpy: msg.rpy,
      temperature: msg.temperature,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      quaternion: msg.quaternion,
      gyroscope: msg.gyroscope,
      accelerometer: msg.accelerometer,
      rpy: msg.rpy,
      temperature: msg.temperature,
    }
  }
}


// Corresponds to unitree_go__msg__InterfaceConfig

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InterfaceConfig {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub reserve: [u8; 2],

}



impl Default for InterfaceConfig {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::InterfaceConfig::default())
  }
}

impl rosidl_runtime_rs::Message for InterfaceConfig {
  type RmwMsg = super::msg::rmw::InterfaceConfig;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
        value: msg.value,
        reserve: msg.reserve,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      value: msg.value,
        reserve: msg.reserve,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
      value: msg.value,
      reserve: msg.reserve,
    }
  }
}


// Corresponds to unitree_go__msg__LidarState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LidarState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub firmware_version: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub software_version: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sdk_version: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sys_rotation_speed: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub com_rotation_speed: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_state: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cloud_frequency: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cloud_packet_loss_rate: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cloud_size: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cloud_scan_num: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub imu_frequency: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub imu_packet_loss_rate: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub imu_rpy: [f32; 3],


    // This member is not documented.
    #[allow(missing_docs)]
    pub serial_recv_stamp: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub serial_buffer_size: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub serial_buffer_read: u32,

}



impl Default for LidarState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LidarState::default())
  }
}

impl rosidl_runtime_rs::Message for LidarState {
  type RmwMsg = super::msg::rmw::LidarState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        stamp: msg.stamp,
        firmware_version: msg.firmware_version.as_str().into(),
        software_version: msg.software_version.as_str().into(),
        sdk_version: msg.sdk_version.as_str().into(),
        sys_rotation_speed: msg.sys_rotation_speed,
        com_rotation_speed: msg.com_rotation_speed,
        error_state: msg.error_state,
        cloud_frequency: msg.cloud_frequency,
        cloud_packet_loss_rate: msg.cloud_packet_loss_rate,
        cloud_size: msg.cloud_size,
        cloud_scan_num: msg.cloud_scan_num,
        imu_frequency: msg.imu_frequency,
        imu_packet_loss_rate: msg.imu_packet_loss_rate,
        imu_rpy: msg.imu_rpy,
        serial_recv_stamp: msg.serial_recv_stamp,
        serial_buffer_size: msg.serial_buffer_size,
        serial_buffer_read: msg.serial_buffer_read,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      stamp: msg.stamp,
        firmware_version: msg.firmware_version.as_str().into(),
        software_version: msg.software_version.as_str().into(),
        sdk_version: msg.sdk_version.as_str().into(),
      sys_rotation_speed: msg.sys_rotation_speed,
      com_rotation_speed: msg.com_rotation_speed,
      error_state: msg.error_state,
      cloud_frequency: msg.cloud_frequency,
      cloud_packet_loss_rate: msg.cloud_packet_loss_rate,
      cloud_size: msg.cloud_size,
      cloud_scan_num: msg.cloud_scan_num,
      imu_frequency: msg.imu_frequency,
      imu_packet_loss_rate: msg.imu_packet_loss_rate,
        imu_rpy: msg.imu_rpy,
      serial_recv_stamp: msg.serial_recv_stamp,
      serial_buffer_size: msg.serial_buffer_size,
      serial_buffer_read: msg.serial_buffer_read,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      stamp: msg.stamp,
      firmware_version: msg.firmware_version.to_string(),
      software_version: msg.software_version.to_string(),
      sdk_version: msg.sdk_version.to_string(),
      sys_rotation_speed: msg.sys_rotation_speed,
      com_rotation_speed: msg.com_rotation_speed,
      error_state: msg.error_state,
      cloud_frequency: msg.cloud_frequency,
      cloud_packet_loss_rate: msg.cloud_packet_loss_rate,
      cloud_size: msg.cloud_size,
      cloud_scan_num: msg.cloud_scan_num,
      imu_frequency: msg.imu_frequency,
      imu_packet_loss_rate: msg.imu_packet_loss_rate,
      imu_rpy: msg.imu_rpy,
      serial_recv_stamp: msg.serial_recv_stamp,
      serial_buffer_size: msg.serial_buffer_size,
      serial_buffer_read: msg.serial_buffer_read,
    }
  }
}


// Corresponds to unitree_go__msg__LowCmd

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LowCmd {

    // This member is not documented.
    #[allow(missing_docs)]
    pub head: [u8; 2],


    // This member is not documented.
    #[allow(missing_docs)]
    pub level_flag: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub frame_reserve: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sn: [u32; 2],


    // This member is not documented.
    #[allow(missing_docs)]
    pub version: [u32; 2],


    // This member is not documented.
    #[allow(missing_docs)]
    pub bandwidth: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_cmd: [super::msg::MotorCmd; 20],


    // This member is not documented.
    #[allow(missing_docs)]
    pub bms_cmd: super::msg::BmsCmd,


    // This member is not documented.
    #[allow(missing_docs)]
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub wireless_remote: [u8; 40],


    // This member is not documented.
    #[allow(missing_docs)]
    pub led: [u8; 12],


    // This member is not documented.
    #[allow(missing_docs)]
    pub fan: [u8; 2],


    // This member is not documented.
    #[allow(missing_docs)]
    pub gpio: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub reserve: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub crc: u32,

}



impl Default for LowCmd {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LowCmd::default())
  }
}

impl rosidl_runtime_rs::Message for LowCmd {
  type RmwMsg = super::msg::rmw::LowCmd;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        head: msg.head,
        level_flag: msg.level_flag,
        frame_reserve: msg.frame_reserve,
        sn: msg.sn,
        version: msg.version,
        bandwidth: msg.bandwidth,
        motor_cmd: msg.motor_cmd
          .map(|elem| super::msg::MotorCmd::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned()),
        bms_cmd: super::msg::BmsCmd::into_rmw_message(std::borrow::Cow::Owned(msg.bms_cmd)).into_owned(),
        wireless_remote: msg.wireless_remote,
        led: msg.led,
        fan: msg.fan,
        gpio: msg.gpio,
        reserve: msg.reserve,
        crc: msg.crc,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        head: msg.head,
      level_flag: msg.level_flag,
      frame_reserve: msg.frame_reserve,
        sn: msg.sn,
        version: msg.version,
      bandwidth: msg.bandwidth,
        motor_cmd: msg.motor_cmd
          .iter()
          .map(|elem| super::msg::MotorCmd::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect::<Vec<_>>()
          .try_into()
          .unwrap(),
        bms_cmd: super::msg::BmsCmd::into_rmw_message(std::borrow::Cow::Borrowed(&msg.bms_cmd)).into_owned(),
        wireless_remote: msg.wireless_remote,
        led: msg.led,
        fan: msg.fan,
      gpio: msg.gpio,
      reserve: msg.reserve,
      crc: msg.crc,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      head: msg.head,
      level_flag: msg.level_flag,
      frame_reserve: msg.frame_reserve,
      sn: msg.sn,
      version: msg.version,
      bandwidth: msg.bandwidth,
      motor_cmd: msg.motor_cmd
        .map(super::msg::MotorCmd::from_rmw_message),
      bms_cmd: super::msg::BmsCmd::from_rmw_message(msg.bms_cmd),
      wireless_remote: msg.wireless_remote,
      led: msg.led,
      fan: msg.fan,
      gpio: msg.gpio,
      reserve: msg.reserve,
      crc: msg.crc,
    }
  }
}


// Corresponds to unitree_go__msg__LowState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LowState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub head: [u8; 2],


    // This member is not documented.
    #[allow(missing_docs)]
    pub level_flag: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub frame_reserve: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sn: [u32; 2],


    // This member is not documented.
    #[allow(missing_docs)]
    pub version: [u32; 2],


    // This member is not documented.
    #[allow(missing_docs)]
    pub bandwidth: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub imu_state: super::msg::IMUState,


    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_state: [super::msg::MotorState; 20],


    // This member is not documented.
    #[allow(missing_docs)]
    pub bms_state: super::msg::BmsState,


    // This member is not documented.
    #[allow(missing_docs)]
    pub foot_force: [i16; 4],


    // This member is not documented.
    #[allow(missing_docs)]
    pub foot_force_est: [i16; 4],


    // This member is not documented.
    #[allow(missing_docs)]
    pub tick: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub wireless_remote: [u8; 40],


    // This member is not documented.
    #[allow(missing_docs)]
    pub bit_flag: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub adc_reel: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub temperature_ntc1: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub temperature_ntc2: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub power_v: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub power_a: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub fan_frequency: [u16; 4],


    // This member is not documented.
    #[allow(missing_docs)]
    pub reserve: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub crc: u32,

}



impl Default for LowState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LowState::default())
  }
}

impl rosidl_runtime_rs::Message for LowState {
  type RmwMsg = super::msg::rmw::LowState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        head: msg.head,
        level_flag: msg.level_flag,
        frame_reserve: msg.frame_reserve,
        sn: msg.sn,
        version: msg.version,
        bandwidth: msg.bandwidth,
        imu_state: super::msg::IMUState::into_rmw_message(std::borrow::Cow::Owned(msg.imu_state)).into_owned(),
        motor_state: msg.motor_state
          .map(|elem| super::msg::MotorState::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned()),
        bms_state: super::msg::BmsState::into_rmw_message(std::borrow::Cow::Owned(msg.bms_state)).into_owned(),
        foot_force: msg.foot_force,
        foot_force_est: msg.foot_force_est,
        tick: msg.tick,
        wireless_remote: msg.wireless_remote,
        bit_flag: msg.bit_flag,
        adc_reel: msg.adc_reel,
        temperature_ntc1: msg.temperature_ntc1,
        temperature_ntc2: msg.temperature_ntc2,
        power_v: msg.power_v,
        power_a: msg.power_a,
        fan_frequency: msg.fan_frequency,
        reserve: msg.reserve,
        crc: msg.crc,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        head: msg.head,
      level_flag: msg.level_flag,
      frame_reserve: msg.frame_reserve,
        sn: msg.sn,
        version: msg.version,
      bandwidth: msg.bandwidth,
        imu_state: super::msg::IMUState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.imu_state)).into_owned(),
        motor_state: msg.motor_state
          .iter()
          .map(|elem| super::msg::MotorState::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect::<Vec<_>>()
          .try_into()
          .unwrap(),
        bms_state: super::msg::BmsState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.bms_state)).into_owned(),
        foot_force: msg.foot_force,
        foot_force_est: msg.foot_force_est,
      tick: msg.tick,
        wireless_remote: msg.wireless_remote,
      bit_flag: msg.bit_flag,
      adc_reel: msg.adc_reel,
      temperature_ntc1: msg.temperature_ntc1,
      temperature_ntc2: msg.temperature_ntc2,
      power_v: msg.power_v,
      power_a: msg.power_a,
        fan_frequency: msg.fan_frequency,
      reserve: msg.reserve,
      crc: msg.crc,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      head: msg.head,
      level_flag: msg.level_flag,
      frame_reserve: msg.frame_reserve,
      sn: msg.sn,
      version: msg.version,
      bandwidth: msg.bandwidth,
      imu_state: super::msg::IMUState::from_rmw_message(msg.imu_state),
      motor_state: msg.motor_state
        .map(super::msg::MotorState::from_rmw_message),
      bms_state: super::msg::BmsState::from_rmw_message(msg.bms_state),
      foot_force: msg.foot_force,
      foot_force_est: msg.foot_force_est,
      tick: msg.tick,
      wireless_remote: msg.wireless_remote,
      bit_flag: msg.bit_flag,
      adc_reel: msg.adc_reel,
      temperature_ntc1: msg.temperature_ntc1,
      temperature_ntc2: msg.temperature_ntc2,
      power_v: msg.power_v,
      power_a: msg.power_a,
      fan_frequency: msg.fan_frequency,
      reserve: msg.reserve,
      crc: msg.crc,
    }
  }
}


// Corresponds to unitree_go__msg__MotorCmd

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotorCmd {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub q: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub dq: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tau: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub kp: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub kd: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub reserve: [u32; 3],

}



impl Default for MotorCmd {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MotorCmd::default())
  }
}

impl rosidl_runtime_rs::Message for MotorCmd {
  type RmwMsg = super::msg::rmw::MotorCmd;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
        q: msg.q,
        dq: msg.dq,
        tau: msg.tau,
        kp: msg.kp,
        kd: msg.kd,
        reserve: msg.reserve,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      q: msg.q,
      dq: msg.dq,
      tau: msg.tau,
      kp: msg.kp,
      kd: msg.kd,
        reserve: msg.reserve,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
      q: msg.q,
      dq: msg.dq,
      tau: msg.tau,
      kp: msg.kp,
      kd: msg.kd,
      reserve: msg.reserve,
    }
  }
}


// Corresponds to unitree_go__msg__MotorCmds

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotorCmds {

    // This member is not documented.
    #[allow(missing_docs)]
    pub cmds: Vec<super::msg::MotorCmd>,

}



impl Default for MotorCmds {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MotorCmds::default())
  }
}

impl rosidl_runtime_rs::Message for MotorCmds {
  type RmwMsg = super::msg::rmw::MotorCmds;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        cmds: msg.cmds
          .into_iter()
          .map(|elem| super::msg::MotorCmd::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        cmds: msg.cmds
          .iter()
          .map(|elem| super::msg::MotorCmd::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      cmds: msg.cmds
          .into_iter()
          .map(super::msg::MotorCmd::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to unitree_go__msg__MotorState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotorState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub q: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub dq: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ddq: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tau_est: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub q_raw: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub dq_raw: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ddq_raw: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub temperature: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lost: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub reserve: [u32; 2],

}



impl Default for MotorState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MotorState::default())
  }
}

impl rosidl_runtime_rs::Message for MotorState {
  type RmwMsg = super::msg::rmw::MotorState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
        q: msg.q,
        dq: msg.dq,
        ddq: msg.ddq,
        tau_est: msg.tau_est,
        q_raw: msg.q_raw,
        dq_raw: msg.dq_raw,
        ddq_raw: msg.ddq_raw,
        temperature: msg.temperature,
        lost: msg.lost,
        reserve: msg.reserve,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      q: msg.q,
      dq: msg.dq,
      ddq: msg.ddq,
      tau_est: msg.tau_est,
      q_raw: msg.q_raw,
      dq_raw: msg.dq_raw,
      ddq_raw: msg.ddq_raw,
      temperature: msg.temperature,
      lost: msg.lost,
        reserve: msg.reserve,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
      q: msg.q,
      dq: msg.dq,
      ddq: msg.ddq,
      tau_est: msg.tau_est,
      q_raw: msg.q_raw,
      dq_raw: msg.dq_raw,
      ddq_raw: msg.ddq_raw,
      temperature: msg.temperature,
      lost: msg.lost,
      reserve: msg.reserve,
    }
  }
}


// Corresponds to unitree_go__msg__MotorStates

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotorStates {

    // This member is not documented.
    #[allow(missing_docs)]
    pub states: Vec<super::msg::MotorState>,

}



impl Default for MotorStates {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MotorStates::default())
  }
}

impl rosidl_runtime_rs::Message for MotorStates {
  type RmwMsg = super::msg::rmw::MotorStates;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        states: msg.states
          .into_iter()
          .map(|elem| super::msg::MotorState::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        states: msg.states
          .iter()
          .map(|elem| super::msg::MotorState::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      states: msg.states
          .into_iter()
          .map(super::msg::MotorState::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to unitree_go__msg__PathPoint

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PathPoint {

    // This member is not documented.
    #[allow(missing_docs)]
    pub t_from_start: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub vx: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub vy: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub vyaw: f32,

}



impl Default for PathPoint {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PathPoint::default())
  }
}

impl rosidl_runtime_rs::Message for PathPoint {
  type RmwMsg = super::msg::rmw::PathPoint;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        t_from_start: msg.t_from_start,
        x: msg.x,
        y: msg.y,
        yaw: msg.yaw,
        vx: msg.vx,
        vy: msg.vy,
        vyaw: msg.vyaw,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      t_from_start: msg.t_from_start,
      x: msg.x,
      y: msg.y,
      yaw: msg.yaw,
      vx: msg.vx,
      vy: msg.vy,
      vyaw: msg.vyaw,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      t_from_start: msg.t_from_start,
      x: msg.x,
      y: msg.y,
      yaw: msg.yaw,
      vx: msg.vx,
      vy: msg.vy,
      vyaw: msg.vyaw,
    }
  }
}


// Corresponds to unitree_go__msg__Req

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Req {

    // This member is not documented.
    #[allow(missing_docs)]
    pub uuid: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub body: std::string::String,

}



impl Default for Req {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Req::default())
  }
}

impl rosidl_runtime_rs::Message for Req {
  type RmwMsg = super::msg::rmw::Req;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        uuid: msg.uuid.as_str().into(),
        body: msg.body.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        uuid: msg.uuid.as_str().into(),
        body: msg.body.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      uuid: msg.uuid.to_string(),
      body: msg.body.to_string(),
    }
  }
}


// Corresponds to unitree_go__msg__Res

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Res {

    // This member is not documented.
    #[allow(missing_docs)]
    pub uuid: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: Vec<u8>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub body: std::string::String,

}



impl Default for Res {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Res::default())
  }
}

impl rosidl_runtime_rs::Message for Res {
  type RmwMsg = super::msg::rmw::Res;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        uuid: msg.uuid.as_str().into(),
        data: msg.data.into(),
        body: msg.body.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        uuid: msg.uuid.as_str().into(),
        data: msg.data.as_slice().into(),
        body: msg.body.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      uuid: msg.uuid.to_string(),
      data: msg.data
          .into_iter()
          .collect(),
      body: msg.body.to_string(),
    }
  }
}


// Corresponds to unitree_go__msg__SportModeCmd

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SportModeCmd {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub gait_type: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub speed_level: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub foot_raise_height: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub body_height: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position: [f32; 2],


    // This member is not documented.
    #[allow(missing_docs)]
    pub euler: [f32; 3],


    // This member is not documented.
    #[allow(missing_docs)]
    pub velocity: [f32; 2],


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw_speed: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub bms_cmd: super::msg::BmsCmd,


    // This member is not documented.
    #[allow(missing_docs)]
    pub path_point: [super::msg::PathPoint; 30],

}



impl Default for SportModeCmd {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SportModeCmd::default())
  }
}

impl rosidl_runtime_rs::Message for SportModeCmd {
  type RmwMsg = super::msg::rmw::SportModeCmd;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
        gait_type: msg.gait_type,
        speed_level: msg.speed_level,
        foot_raise_height: msg.foot_raise_height,
        body_height: msg.body_height,
        position: msg.position,
        euler: msg.euler,
        velocity: msg.velocity,
        yaw_speed: msg.yaw_speed,
        bms_cmd: super::msg::BmsCmd::into_rmw_message(std::borrow::Cow::Owned(msg.bms_cmd)).into_owned(),
        path_point: msg.path_point
          .map(|elem| super::msg::PathPoint::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned()),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      gait_type: msg.gait_type,
      speed_level: msg.speed_level,
      foot_raise_height: msg.foot_raise_height,
      body_height: msg.body_height,
        position: msg.position,
        euler: msg.euler,
        velocity: msg.velocity,
      yaw_speed: msg.yaw_speed,
        bms_cmd: super::msg::BmsCmd::into_rmw_message(std::borrow::Cow::Borrowed(&msg.bms_cmd)).into_owned(),
        path_point: msg.path_point
          .iter()
          .map(|elem| super::msg::PathPoint::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect::<Vec<_>>()
          .try_into()
          .unwrap(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
      gait_type: msg.gait_type,
      speed_level: msg.speed_level,
      foot_raise_height: msg.foot_raise_height,
      body_height: msg.body_height,
      position: msg.position,
      euler: msg.euler,
      velocity: msg.velocity,
      yaw_speed: msg.yaw_speed,
      bms_cmd: super::msg::BmsCmd::from_rmw_message(msg.bms_cmd),
      path_point: msg.path_point
        .map(super::msg::PathPoint::from_rmw_message),
    }
  }
}


// Corresponds to unitree_go__msg__SportModeState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SportModeState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: super::msg::TimeSpec,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_code: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub imu_state: super::msg::IMUState,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub progress: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub gait_type: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub foot_raise_height: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position: [f32; 3],


    // This member is not documented.
    #[allow(missing_docs)]
    pub body_height: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub velocity: [f32; 3],


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw_speed: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub range_obstacle: [f32; 4],


    // This member is not documented.
    #[allow(missing_docs)]
    pub foot_force: [i16; 4],


    // This member is not documented.
    #[allow(missing_docs)]
    pub foot_position_body: [f32; 12],


    // This member is not documented.
    #[allow(missing_docs)]
    pub foot_speed_body: [f32; 12],

}



impl Default for SportModeState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SportModeState::default())
  }
}

impl rosidl_runtime_rs::Message for SportModeState {
  type RmwMsg = super::msg::rmw::SportModeState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        stamp: super::msg::TimeSpec::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
        error_code: msg.error_code,
        imu_state: super::msg::IMUState::into_rmw_message(std::borrow::Cow::Owned(msg.imu_state)).into_owned(),
        mode: msg.mode,
        progress: msg.progress,
        gait_type: msg.gait_type,
        foot_raise_height: msg.foot_raise_height,
        position: msg.position,
        body_height: msg.body_height,
        velocity: msg.velocity,
        yaw_speed: msg.yaw_speed,
        range_obstacle: msg.range_obstacle,
        foot_force: msg.foot_force,
        foot_position_body: msg.foot_position_body,
        foot_speed_body: msg.foot_speed_body,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        stamp: super::msg::TimeSpec::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      error_code: msg.error_code,
        imu_state: super::msg::IMUState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.imu_state)).into_owned(),
      mode: msg.mode,
      progress: msg.progress,
      gait_type: msg.gait_type,
      foot_raise_height: msg.foot_raise_height,
        position: msg.position,
      body_height: msg.body_height,
        velocity: msg.velocity,
      yaw_speed: msg.yaw_speed,
        range_obstacle: msg.range_obstacle,
        foot_force: msg.foot_force,
        foot_position_body: msg.foot_position_body,
        foot_speed_body: msg.foot_speed_body,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      stamp: super::msg::TimeSpec::from_rmw_message(msg.stamp),
      error_code: msg.error_code,
      imu_state: super::msg::IMUState::from_rmw_message(msg.imu_state),
      mode: msg.mode,
      progress: msg.progress,
      gait_type: msg.gait_type,
      foot_raise_height: msg.foot_raise_height,
      position: msg.position,
      body_height: msg.body_height,
      velocity: msg.velocity,
      yaw_speed: msg.yaw_speed,
      range_obstacle: msg.range_obstacle,
      foot_force: msg.foot_force,
      foot_position_body: msg.foot_position_body,
      foot_speed_body: msg.foot_speed_body,
    }
  }
}


// Corresponds to unitree_go__msg__TimeSpec
/// Time indicates a specific point in time, relative to a clock's 0 point.
/// The seconds component, valid over all int32 values.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TimeSpec {

    // This member is not documented.
    #[allow(missing_docs)]
    pub sec: i32,

    /// The nanoseconds component, valid in the range [0, 10e9).
    pub nanosec: u32,

}



impl Default for TimeSpec {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::TimeSpec::default())
  }
}

impl rosidl_runtime_rs::Message for TimeSpec {
  type RmwMsg = super::msg::rmw::TimeSpec;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        sec: msg.sec,
        nanosec: msg.nanosec,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      sec: msg.sec,
      nanosec: msg.nanosec,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      sec: msg.sec,
      nanosec: msg.nanosec,
    }
  }
}


// Corresponds to unitree_go__msg__UwbState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UwbState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub version: [u8; 2],


    // This member is not documented.
    #[allow(missing_docs)]
    pub channel: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joy_mode: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub orientation_est: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pitch_est: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub distance_est: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw_est: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tag_roll: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tag_pitch: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tag_yaw: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub base_roll: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub base_pitch: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub base_yaw: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joystick: [f32; 2],


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_state: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub buttons: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub enabled_from_app: u8,

}



impl Default for UwbState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::UwbState::default())
  }
}

impl rosidl_runtime_rs::Message for UwbState {
  type RmwMsg = super::msg::rmw::UwbState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        version: msg.version,
        channel: msg.channel,
        joy_mode: msg.joy_mode,
        orientation_est: msg.orientation_est,
        pitch_est: msg.pitch_est,
        distance_est: msg.distance_est,
        yaw_est: msg.yaw_est,
        tag_roll: msg.tag_roll,
        tag_pitch: msg.tag_pitch,
        tag_yaw: msg.tag_yaw,
        base_roll: msg.base_roll,
        base_pitch: msg.base_pitch,
        base_yaw: msg.base_yaw,
        joystick: msg.joystick,
        error_state: msg.error_state,
        buttons: msg.buttons,
        enabled_from_app: msg.enabled_from_app,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        version: msg.version,
      channel: msg.channel,
      joy_mode: msg.joy_mode,
      orientation_est: msg.orientation_est,
      pitch_est: msg.pitch_est,
      distance_est: msg.distance_est,
      yaw_est: msg.yaw_est,
      tag_roll: msg.tag_roll,
      tag_pitch: msg.tag_pitch,
      tag_yaw: msg.tag_yaw,
      base_roll: msg.base_roll,
      base_pitch: msg.base_pitch,
      base_yaw: msg.base_yaw,
        joystick: msg.joystick,
      error_state: msg.error_state,
      buttons: msg.buttons,
      enabled_from_app: msg.enabled_from_app,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      version: msg.version,
      channel: msg.channel,
      joy_mode: msg.joy_mode,
      orientation_est: msg.orientation_est,
      pitch_est: msg.pitch_est,
      distance_est: msg.distance_est,
      yaw_est: msg.yaw_est,
      tag_roll: msg.tag_roll,
      tag_pitch: msg.tag_pitch,
      tag_yaw: msg.tag_yaw,
      base_roll: msg.base_roll,
      base_pitch: msg.base_pitch,
      base_yaw: msg.base_yaw,
      joystick: msg.joystick,
      error_state: msg.error_state,
      buttons: msg.buttons,
      enabled_from_app: msg.enabled_from_app,
    }
  }
}


// Corresponds to unitree_go__msg__UwbSwitch

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UwbSwitch {

    // This member is not documented.
    #[allow(missing_docs)]
    pub enabled: u8,

}



impl Default for UwbSwitch {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::UwbSwitch::default())
  }
}

impl rosidl_runtime_rs::Message for UwbSwitch {
  type RmwMsg = super::msg::rmw::UwbSwitch;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        enabled: msg.enabled,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      enabled: msg.enabled,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      enabled: msg.enabled,
    }
  }
}


// Corresponds to unitree_go__msg__WirelessController

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WirelessController {

    // This member is not documented.
    #[allow(missing_docs)]
    pub lx: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ly: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rx: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ry: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub keys: u16,

}



impl Default for WirelessController {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::WirelessController::default())
  }
}

impl rosidl_runtime_rs::Message for WirelessController {
  type RmwMsg = super::msg::rmw::WirelessController;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        lx: msg.lx,
        ly: msg.ly,
        rx: msg.rx,
        ry: msg.ry,
        keys: msg.keys,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      lx: msg.lx,
      ly: msg.ly,
      rx: msg.rx,
      ry: msg.ry,
      keys: msg.keys,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      lx: msg.lx,
      ly: msg.ly,
      rx: msg.rx,
      ry: msg.ry,
      keys: msg.keys,
    }
  }
}



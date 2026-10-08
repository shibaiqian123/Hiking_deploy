#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to unitree_hg__msg__BmsCmd

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BmsCmd {

    // This member is not documented.
    #[allow(missing_docs)]
    pub cmd: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub reserve: [u8; 40],

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
        cmd: msg.cmd,
        reserve: msg.reserve,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      cmd: msg.cmd,
        reserve: msg.reserve,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      cmd: msg.cmd,
      reserve: msg.reserve,
    }
  }
}


// Corresponds to unitree_hg__msg__BmsState

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
    pub fn_: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub cell_vol: [u16; 40],


    // This member is not documented.
    #[allow(missing_docs)]
    pub bmsvoltage: [u32; 3],


    // This member is not documented.
    #[allow(missing_docs)]
    pub current: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub soc: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub soh: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub temperature: [i16; 12],


    // This member is not documented.
    #[allow(missing_docs)]
    pub cycle: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub manufacturer_date: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub bmsstate: [u32; 5],


    // This member is not documented.
    #[allow(missing_docs)]
    pub reserve: [u32; 3],

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
        fn_: msg.fn_,
        cell_vol: msg.cell_vol,
        bmsvoltage: msg.bmsvoltage,
        current: msg.current,
        soc: msg.soc,
        soh: msg.soh,
        temperature: msg.temperature,
        cycle: msg.cycle,
        manufacturer_date: msg.manufacturer_date,
        bmsstate: msg.bmsstate,
        reserve: msg.reserve,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      version_high: msg.version_high,
      version_low: msg.version_low,
      fn_: msg.fn_,
        cell_vol: msg.cell_vol,
        bmsvoltage: msg.bmsvoltage,
      current: msg.current,
      soc: msg.soc,
      soh: msg.soh,
        temperature: msg.temperature,
      cycle: msg.cycle,
      manufacturer_date: msg.manufacturer_date,
        bmsstate: msg.bmsstate,
        reserve: msg.reserve,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      version_high: msg.version_high,
      version_low: msg.version_low,
      fn_: msg.fn_,
      cell_vol: msg.cell_vol,
      bmsvoltage: msg.bmsvoltage,
      current: msg.current,
      soc: msg.soc,
      soh: msg.soh,
      temperature: msg.temperature,
      cycle: msg.cycle,
      manufacturer_date: msg.manufacturer_date,
      bmsstate: msg.bmsstate,
      reserve: msg.reserve,
    }
  }
}


// Corresponds to unitree_hg__msg__HandCmd

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HandCmd {

    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_cmd: Vec<super::msg::MotorCmd>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub reserve: [u32; 4],

}



impl Default for HandCmd {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::HandCmd::default())
  }
}

impl rosidl_runtime_rs::Message for HandCmd {
  type RmwMsg = super::msg::rmw::HandCmd;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        motor_cmd: msg.motor_cmd
          .into_iter()
          .map(|elem| super::msg::MotorCmd::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        reserve: msg.reserve,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        motor_cmd: msg.motor_cmd
          .iter()
          .map(|elem| super::msg::MotorCmd::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        reserve: msg.reserve,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      motor_cmd: msg.motor_cmd
          .into_iter()
          .map(super::msg::MotorCmd::from_rmw_message)
          .collect(),
      reserve: msg.reserve,
    }
  }
}


// Corresponds to unitree_hg__msg__HandState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HandState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_state: Vec<super::msg::MotorState>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub press_sensor_state: Vec<super::msg::PressSensorState>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub imu_state: super::msg::IMUState,


    // This member is not documented.
    #[allow(missing_docs)]
    pub power_v: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub power_a: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub system_v: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub device_v: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error: [u32; 2],


    // This member is not documented.
    #[allow(missing_docs)]
    pub reserve: [u32; 2],

}



impl Default for HandState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::HandState::default())
  }
}

impl rosidl_runtime_rs::Message for HandState {
  type RmwMsg = super::msg::rmw::HandState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        motor_state: msg.motor_state
          .into_iter()
          .map(|elem| super::msg::MotorState::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        press_sensor_state: msg.press_sensor_state
          .into_iter()
          .map(|elem| super::msg::PressSensorState::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        imu_state: super::msg::IMUState::into_rmw_message(std::borrow::Cow::Owned(msg.imu_state)).into_owned(),
        power_v: msg.power_v,
        power_a: msg.power_a,
        system_v: msg.system_v,
        device_v: msg.device_v,
        error: msg.error,
        reserve: msg.reserve,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        motor_state: msg.motor_state
          .iter()
          .map(|elem| super::msg::MotorState::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        press_sensor_state: msg.press_sensor_state
          .iter()
          .map(|elem| super::msg::PressSensorState::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        imu_state: super::msg::IMUState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.imu_state)).into_owned(),
      power_v: msg.power_v,
      power_a: msg.power_a,
      system_v: msg.system_v,
      device_v: msg.device_v,
        error: msg.error,
        reserve: msg.reserve,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      motor_state: msg.motor_state
          .into_iter()
          .map(super::msg::MotorState::from_rmw_message)
          .collect(),
      press_sensor_state: msg.press_sensor_state
          .into_iter()
          .map(super::msg::PressSensorState::from_rmw_message)
          .collect(),
      imu_state: super::msg::IMUState::from_rmw_message(msg.imu_state),
      power_v: msg.power_v,
      power_a: msg.power_a,
      system_v: msg.system_v,
      device_v: msg.device_v,
      error: msg.error,
      reserve: msg.reserve,
    }
  }
}


// Corresponds to unitree_hg__msg__IMUState

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
    pub temperature: i16,

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


// Corresponds to unitree_hg__msg__LowCmd

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LowCmd {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode_pr: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mode_machine: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub motor_cmd: [super::msg::MotorCmd; 35],


    // This member is not documented.
    #[allow(missing_docs)]
    pub reserve: [u32; 4],


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
        mode_pr: msg.mode_pr,
        mode_machine: msg.mode_machine,
        motor_cmd: msg.motor_cmd
          .map(|elem| super::msg::MotorCmd::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned()),
        reserve: msg.reserve,
        crc: msg.crc,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode_pr: msg.mode_pr,
      mode_machine: msg.mode_machine,
        motor_cmd: msg.motor_cmd
          .iter()
          .map(|elem| super::msg::MotorCmd::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect::<Vec<_>>()
          .try_into()
          .unwrap(),
        reserve: msg.reserve,
      crc: msg.crc,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode_pr: msg.mode_pr,
      mode_machine: msg.mode_machine,
      motor_cmd: msg.motor_cmd
        .map(super::msg::MotorCmd::from_rmw_message),
      reserve: msg.reserve,
      crc: msg.crc,
    }
  }
}


// Corresponds to unitree_hg__msg__LowState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LowState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub version: [u32; 2],


    // This member is not documented.
    #[allow(missing_docs)]
    pub mode_pr: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mode_machine: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tick: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub imu_state: super::msg::IMUState,


    // This member is not documented.
    #[allow(missing_docs)]
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub motor_state: [super::msg::MotorState; 35],


    // This member is not documented.
    #[allow(missing_docs)]
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub wireless_remote: [u8; 40],


    // This member is not documented.
    #[allow(missing_docs)]
    pub reserve: [u32; 4],


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
        version: msg.version,
        mode_pr: msg.mode_pr,
        mode_machine: msg.mode_machine,
        tick: msg.tick,
        imu_state: super::msg::IMUState::into_rmw_message(std::borrow::Cow::Owned(msg.imu_state)).into_owned(),
        motor_state: msg.motor_state
          .map(|elem| super::msg::MotorState::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned()),
        wireless_remote: msg.wireless_remote,
        reserve: msg.reserve,
        crc: msg.crc,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        version: msg.version,
      mode_pr: msg.mode_pr,
      mode_machine: msg.mode_machine,
      tick: msg.tick,
        imu_state: super::msg::IMUState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.imu_state)).into_owned(),
        motor_state: msg.motor_state
          .iter()
          .map(|elem| super::msg::MotorState::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect::<Vec<_>>()
          .try_into()
          .unwrap(),
        wireless_remote: msg.wireless_remote,
        reserve: msg.reserve,
      crc: msg.crc,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      version: msg.version,
      mode_pr: msg.mode_pr,
      mode_machine: msg.mode_machine,
      tick: msg.tick,
      imu_state: super::msg::IMUState::from_rmw_message(msg.imu_state),
      motor_state: msg.motor_state
        .map(super::msg::MotorState::from_rmw_message),
      wireless_remote: msg.wireless_remote,
      reserve: msg.reserve,
      crc: msg.crc,
    }
  }
}


// Corresponds to unitree_hg__msg__MainBoardState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MainBoardState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub fan_state: [u16; 6],


    // This member is not documented.
    #[allow(missing_docs)]
    pub temperature: [i16; 6],


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: [f32; 6],


    // This member is not documented.
    #[allow(missing_docs)]
    pub state: [u32; 6],

}



impl Default for MainBoardState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MainBoardState::default())
  }
}

impl rosidl_runtime_rs::Message for MainBoardState {
  type RmwMsg = super::msg::rmw::MainBoardState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        fan_state: msg.fan_state,
        temperature: msg.temperature,
        value: msg.value,
        state: msg.state,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        fan_state: msg.fan_state,
        temperature: msg.temperature,
        value: msg.value,
        state: msg.state,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      fan_state: msg.fan_state,
      temperature: msg.temperature,
      value: msg.value,
      state: msg.state,
    }
  }
}


// Corresponds to unitree_hg__msg__MotorCmd

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
    pub reserve: u32,

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


// Corresponds to unitree_hg__msg__MotorState

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
    pub temperature: [i16; 2],


    // This member is not documented.
    #[allow(missing_docs)]
    pub vol: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sensor: [u32; 2],


    // This member is not documented.
    #[allow(missing_docs)]
    pub motorstate: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub reserve: [u32; 4],

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
        temperature: msg.temperature,
        vol: msg.vol,
        sensor: msg.sensor,
        motorstate: msg.motorstate,
        reserve: msg.reserve,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      q: msg.q,
      dq: msg.dq,
      ddq: msg.ddq,
      tau_est: msg.tau_est,
        temperature: msg.temperature,
      vol: msg.vol,
        sensor: msg.sensor,
      motorstate: msg.motorstate,
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
      temperature: msg.temperature,
      vol: msg.vol,
      sensor: msg.sensor,
      motorstate: msg.motorstate,
      reserve: msg.reserve,
    }
  }
}


// Corresponds to unitree_hg__msg__PressSensorState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PressSensorState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pressure: [f32; 12],


    // This member is not documented.
    #[allow(missing_docs)]
    pub temperature: [f32; 12],


    // This member is not documented.
    #[allow(missing_docs)]
    pub lost: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub reserve: u32,

}



impl Default for PressSensorState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PressSensorState::default())
  }
}

impl rosidl_runtime_rs::Message for PressSensorState {
  type RmwMsg = super::msg::rmw::PressSensorState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pressure: msg.pressure,
        temperature: msg.temperature,
        lost: msg.lost,
        reserve: msg.reserve,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pressure: msg.pressure,
        temperature: msg.temperature,
      lost: msg.lost,
      reserve: msg.reserve,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      pressure: msg.pressure,
      temperature: msg.temperature,
      lost: msg.lost,
      reserve: msg.reserve,
    }
  }
}



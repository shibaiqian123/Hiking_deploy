#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "unitree_hg__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_hg__msg__BmsCmd() -> *const std::ffi::c_void;
}

#[link(name = "unitree_hg__rosidl_generator_c")]
extern "C" {
    fn unitree_hg__msg__BmsCmd__init(msg: *mut BmsCmd) -> bool;
    fn unitree_hg__msg__BmsCmd__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BmsCmd>, size: usize) -> bool;
    fn unitree_hg__msg__BmsCmd__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BmsCmd>);
    fn unitree_hg__msg__BmsCmd__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BmsCmd>, out_seq: *mut rosidl_runtime_rs::Sequence<BmsCmd>) -> bool;
}

// Corresponds to unitree_hg__msg__BmsCmd
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_hg__msg__BmsCmd__init(&mut msg as *mut _) {
        panic!("Call to unitree_hg__msg__BmsCmd__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BmsCmd {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__BmsCmd__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__BmsCmd__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__BmsCmd__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BmsCmd {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BmsCmd where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_hg/msg/BmsCmd";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_hg__msg__BmsCmd() }
  }
}


#[link(name = "unitree_hg__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_hg__msg__BmsState() -> *const std::ffi::c_void;
}

#[link(name = "unitree_hg__rosidl_generator_c")]
extern "C" {
    fn unitree_hg__msg__BmsState__init(msg: *mut BmsState) -> bool;
    fn unitree_hg__msg__BmsState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BmsState>, size: usize) -> bool;
    fn unitree_hg__msg__BmsState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BmsState>);
    fn unitree_hg__msg__BmsState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BmsState>, out_seq: *mut rosidl_runtime_rs::Sequence<BmsState>) -> bool;
}

// Corresponds to unitree_hg__msg__BmsState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_hg__msg__BmsState__init(&mut msg as *mut _) {
        panic!("Call to unitree_hg__msg__BmsState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BmsState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__BmsState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__BmsState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__BmsState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BmsState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BmsState where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_hg/msg/BmsState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_hg__msg__BmsState() }
  }
}


#[link(name = "unitree_hg__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_hg__msg__HandCmd() -> *const std::ffi::c_void;
}

#[link(name = "unitree_hg__rosidl_generator_c")]
extern "C" {
    fn unitree_hg__msg__HandCmd__init(msg: *mut HandCmd) -> bool;
    fn unitree_hg__msg__HandCmd__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HandCmd>, size: usize) -> bool;
    fn unitree_hg__msg__HandCmd__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HandCmd>);
    fn unitree_hg__msg__HandCmd__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HandCmd>, out_seq: *mut rosidl_runtime_rs::Sequence<HandCmd>) -> bool;
}

// Corresponds to unitree_hg__msg__HandCmd
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HandCmd {

    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_cmd: rosidl_runtime_rs::Sequence<super::super::msg::rmw::MotorCmd>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub reserve: [u32; 4],

}



impl Default for HandCmd {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_hg__msg__HandCmd__init(&mut msg as *mut _) {
        panic!("Call to unitree_hg__msg__HandCmd__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HandCmd {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__HandCmd__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__HandCmd__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__HandCmd__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HandCmd {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HandCmd where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_hg/msg/HandCmd";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_hg__msg__HandCmd() }
  }
}


#[link(name = "unitree_hg__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_hg__msg__HandState() -> *const std::ffi::c_void;
}

#[link(name = "unitree_hg__rosidl_generator_c")]
extern "C" {
    fn unitree_hg__msg__HandState__init(msg: *mut HandState) -> bool;
    fn unitree_hg__msg__HandState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HandState>, size: usize) -> bool;
    fn unitree_hg__msg__HandState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HandState>);
    fn unitree_hg__msg__HandState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HandState>, out_seq: *mut rosidl_runtime_rs::Sequence<HandState>) -> bool;
}

// Corresponds to unitree_hg__msg__HandState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HandState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_state: rosidl_runtime_rs::Sequence<super::super::msg::rmw::MotorState>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub press_sensor_state: rosidl_runtime_rs::Sequence<super::super::msg::rmw::PressSensorState>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub imu_state: super::super::msg::rmw::IMUState,


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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_hg__msg__HandState__init(&mut msg as *mut _) {
        panic!("Call to unitree_hg__msg__HandState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HandState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__HandState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__HandState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__HandState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HandState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HandState where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_hg/msg/HandState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_hg__msg__HandState() }
  }
}


#[link(name = "unitree_hg__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_hg__msg__IMUState() -> *const std::ffi::c_void;
}

#[link(name = "unitree_hg__rosidl_generator_c")]
extern "C" {
    fn unitree_hg__msg__IMUState__init(msg: *mut IMUState) -> bool;
    fn unitree_hg__msg__IMUState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<IMUState>, size: usize) -> bool;
    fn unitree_hg__msg__IMUState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<IMUState>);
    fn unitree_hg__msg__IMUState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<IMUState>, out_seq: *mut rosidl_runtime_rs::Sequence<IMUState>) -> bool;
}

// Corresponds to unitree_hg__msg__IMUState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_hg__msg__IMUState__init(&mut msg as *mut _) {
        panic!("Call to unitree_hg__msg__IMUState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for IMUState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__IMUState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__IMUState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__IMUState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for IMUState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for IMUState where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_hg/msg/IMUState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_hg__msg__IMUState() }
  }
}


#[link(name = "unitree_hg__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_hg__msg__LowCmd() -> *const std::ffi::c_void;
}

#[link(name = "unitree_hg__rosidl_generator_c")]
extern "C" {
    fn unitree_hg__msg__LowCmd__init(msg: *mut LowCmd) -> bool;
    fn unitree_hg__msg__LowCmd__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LowCmd>, size: usize) -> bool;
    fn unitree_hg__msg__LowCmd__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LowCmd>);
    fn unitree_hg__msg__LowCmd__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LowCmd>, out_seq: *mut rosidl_runtime_rs::Sequence<LowCmd>) -> bool;
}

// Corresponds to unitree_hg__msg__LowCmd
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    pub motor_cmd: [super::super::msg::rmw::MotorCmd; 35],


    // This member is not documented.
    #[allow(missing_docs)]
    pub reserve: [u32; 4],


    // This member is not documented.
    #[allow(missing_docs)]
    pub crc: u32,

}



impl Default for LowCmd {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_hg__msg__LowCmd__init(&mut msg as *mut _) {
        panic!("Call to unitree_hg__msg__LowCmd__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LowCmd {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__LowCmd__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__LowCmd__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__LowCmd__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LowCmd {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LowCmd where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_hg/msg/LowCmd";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_hg__msg__LowCmd() }
  }
}


#[link(name = "unitree_hg__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_hg__msg__LowState() -> *const std::ffi::c_void;
}

#[link(name = "unitree_hg__rosidl_generator_c")]
extern "C" {
    fn unitree_hg__msg__LowState__init(msg: *mut LowState) -> bool;
    fn unitree_hg__msg__LowState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LowState>, size: usize) -> bool;
    fn unitree_hg__msg__LowState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LowState>);
    fn unitree_hg__msg__LowState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LowState>, out_seq: *mut rosidl_runtime_rs::Sequence<LowState>) -> bool;
}

// Corresponds to unitree_hg__msg__LowState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    pub imu_state: super::super::msg::rmw::IMUState,


    // This member is not documented.
    #[allow(missing_docs)]
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub motor_state: [super::super::msg::rmw::MotorState; 35],


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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_hg__msg__LowState__init(&mut msg as *mut _) {
        panic!("Call to unitree_hg__msg__LowState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LowState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__LowState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__LowState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__LowState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LowState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LowState where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_hg/msg/LowState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_hg__msg__LowState() }
  }
}


#[link(name = "unitree_hg__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_hg__msg__MainBoardState() -> *const std::ffi::c_void;
}

#[link(name = "unitree_hg__rosidl_generator_c")]
extern "C" {
    fn unitree_hg__msg__MainBoardState__init(msg: *mut MainBoardState) -> bool;
    fn unitree_hg__msg__MainBoardState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MainBoardState>, size: usize) -> bool;
    fn unitree_hg__msg__MainBoardState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MainBoardState>);
    fn unitree_hg__msg__MainBoardState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MainBoardState>, out_seq: *mut rosidl_runtime_rs::Sequence<MainBoardState>) -> bool;
}

// Corresponds to unitree_hg__msg__MainBoardState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_hg__msg__MainBoardState__init(&mut msg as *mut _) {
        panic!("Call to unitree_hg__msg__MainBoardState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MainBoardState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__MainBoardState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__MainBoardState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__MainBoardState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MainBoardState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MainBoardState where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_hg/msg/MainBoardState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_hg__msg__MainBoardState() }
  }
}


#[link(name = "unitree_hg__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_hg__msg__MotorCmd() -> *const std::ffi::c_void;
}

#[link(name = "unitree_hg__rosidl_generator_c")]
extern "C" {
    fn unitree_hg__msg__MotorCmd__init(msg: *mut MotorCmd) -> bool;
    fn unitree_hg__msg__MotorCmd__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotorCmd>, size: usize) -> bool;
    fn unitree_hg__msg__MotorCmd__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotorCmd>);
    fn unitree_hg__msg__MotorCmd__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotorCmd>, out_seq: *mut rosidl_runtime_rs::Sequence<MotorCmd>) -> bool;
}

// Corresponds to unitree_hg__msg__MotorCmd
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_hg__msg__MotorCmd__init(&mut msg as *mut _) {
        panic!("Call to unitree_hg__msg__MotorCmd__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotorCmd {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__MotorCmd__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__MotorCmd__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__MotorCmd__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotorCmd {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotorCmd where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_hg/msg/MotorCmd";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_hg__msg__MotorCmd() }
  }
}


#[link(name = "unitree_hg__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_hg__msg__MotorState() -> *const std::ffi::c_void;
}

#[link(name = "unitree_hg__rosidl_generator_c")]
extern "C" {
    fn unitree_hg__msg__MotorState__init(msg: *mut MotorState) -> bool;
    fn unitree_hg__msg__MotorState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotorState>, size: usize) -> bool;
    fn unitree_hg__msg__MotorState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotorState>);
    fn unitree_hg__msg__MotorState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotorState>, out_seq: *mut rosidl_runtime_rs::Sequence<MotorState>) -> bool;
}

// Corresponds to unitree_hg__msg__MotorState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_hg__msg__MotorState__init(&mut msg as *mut _) {
        panic!("Call to unitree_hg__msg__MotorState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotorState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__MotorState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__MotorState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__MotorState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotorState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotorState where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_hg/msg/MotorState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_hg__msg__MotorState() }
  }
}


#[link(name = "unitree_hg__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_hg__msg__PressSensorState() -> *const std::ffi::c_void;
}

#[link(name = "unitree_hg__rosidl_generator_c")]
extern "C" {
    fn unitree_hg__msg__PressSensorState__init(msg: *mut PressSensorState) -> bool;
    fn unitree_hg__msg__PressSensorState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PressSensorState>, size: usize) -> bool;
    fn unitree_hg__msg__PressSensorState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PressSensorState>);
    fn unitree_hg__msg__PressSensorState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PressSensorState>, out_seq: *mut rosidl_runtime_rs::Sequence<PressSensorState>) -> bool;
}

// Corresponds to unitree_hg__msg__PressSensorState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_hg__msg__PressSensorState__init(&mut msg as *mut _) {
        panic!("Call to unitree_hg__msg__PressSensorState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PressSensorState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__PressSensorState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__PressSensorState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_hg__msg__PressSensorState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PressSensorState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PressSensorState where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_hg/msg/PressSensorState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_hg__msg__PressSensorState() }
  }
}



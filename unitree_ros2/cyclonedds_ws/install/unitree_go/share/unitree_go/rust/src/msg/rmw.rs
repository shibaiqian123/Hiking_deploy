#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__AudioData() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__AudioData__init(msg: *mut AudioData) -> bool;
    fn unitree_go__msg__AudioData__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AudioData>, size: usize) -> bool;
    fn unitree_go__msg__AudioData__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AudioData>);
    fn unitree_go__msg__AudioData__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AudioData>, out_seq: *mut rosidl_runtime_rs::Sequence<AudioData>) -> bool;
}

// Corresponds to unitree_go__msg__AudioData
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioData {

    // This member is not documented.
    #[allow(missing_docs)]
    pub time_frame: u64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: rosidl_runtime_rs::Sequence<u8>,

}



impl Default for AudioData {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__AudioData__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__AudioData__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AudioData {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__AudioData__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__AudioData__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__AudioData__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AudioData {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AudioData where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/AudioData";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__AudioData() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__BmsCmd() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__BmsCmd__init(msg: *mut BmsCmd) -> bool;
    fn unitree_go__msg__BmsCmd__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BmsCmd>, size: usize) -> bool;
    fn unitree_go__msg__BmsCmd__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BmsCmd>);
    fn unitree_go__msg__BmsCmd__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BmsCmd>, out_seq: *mut rosidl_runtime_rs::Sequence<BmsCmd>) -> bool;
}

// Corresponds to unitree_go__msg__BmsCmd
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__BmsCmd__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__BmsCmd__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BmsCmd {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__BmsCmd__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__BmsCmd__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__BmsCmd__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BmsCmd {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BmsCmd where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/BmsCmd";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__BmsCmd() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__BmsState() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__BmsState__init(msg: *mut BmsState) -> bool;
    fn unitree_go__msg__BmsState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BmsState>, size: usize) -> bool;
    fn unitree_go__msg__BmsState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BmsState>);
    fn unitree_go__msg__BmsState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BmsState>, out_seq: *mut rosidl_runtime_rs::Sequence<BmsState>) -> bool;
}

// Corresponds to unitree_go__msg__BmsState
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__BmsState__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__BmsState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BmsState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__BmsState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__BmsState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__BmsState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BmsState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BmsState where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/BmsState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__BmsState() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__Error() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__Error__init(msg: *mut Error) -> bool;
    fn unitree_go__msg__Error__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Error>, size: usize) -> bool;
    fn unitree_go__msg__Error__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Error>);
    fn unitree_go__msg__Error__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Error>, out_seq: *mut rosidl_runtime_rs::Sequence<Error>) -> bool;
}

// Corresponds to unitree_go__msg__Error
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__Error__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__Error__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Error {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__Error__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__Error__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__Error__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Error {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Error where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/Error";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__Error() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__Go2FrontVideoData() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__Go2FrontVideoData__init(msg: *mut Go2FrontVideoData) -> bool;
    fn unitree_go__msg__Go2FrontVideoData__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Go2FrontVideoData>, size: usize) -> bool;
    fn unitree_go__msg__Go2FrontVideoData__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Go2FrontVideoData>);
    fn unitree_go__msg__Go2FrontVideoData__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Go2FrontVideoData>, out_seq: *mut rosidl_runtime_rs::Sequence<Go2FrontVideoData>) -> bool;
}

// Corresponds to unitree_go__msg__Go2FrontVideoData
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Go2FrontVideoData {

    // This member is not documented.
    #[allow(missing_docs)]
    pub time_frame: u64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub video720p: rosidl_runtime_rs::Sequence<u8>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub video360p: rosidl_runtime_rs::Sequence<u8>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub video180p: rosidl_runtime_rs::Sequence<u8>,

}



impl Default for Go2FrontVideoData {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__Go2FrontVideoData__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__Go2FrontVideoData__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Go2FrontVideoData {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__Go2FrontVideoData__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__Go2FrontVideoData__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__Go2FrontVideoData__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Go2FrontVideoData {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Go2FrontVideoData where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/Go2FrontVideoData";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__Go2FrontVideoData() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__HeightMap() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__HeightMap__init(msg: *mut HeightMap) -> bool;
    fn unitree_go__msg__HeightMap__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HeightMap>, size: usize) -> bool;
    fn unitree_go__msg__HeightMap__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HeightMap>);
    fn unitree_go__msg__HeightMap__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HeightMap>, out_seq: *mut rosidl_runtime_rs::Sequence<HeightMap>) -> bool;
}

// Corresponds to unitree_go__msg__HeightMap
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Header

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HeightMap {
    /// timestamp
    pub stamp: f64,

    /// world frame id
    pub frame_id: rosidl_runtime_rs::String,

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
    pub data: rosidl_runtime_rs::Sequence<f32>,

}



impl Default for HeightMap {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__HeightMap__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__HeightMap__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HeightMap {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__HeightMap__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__HeightMap__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__HeightMap__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HeightMap {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HeightMap where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/HeightMap";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__HeightMap() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__IMUState() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__IMUState__init(msg: *mut IMUState) -> bool;
    fn unitree_go__msg__IMUState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<IMUState>, size: usize) -> bool;
    fn unitree_go__msg__IMUState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<IMUState>);
    fn unitree_go__msg__IMUState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<IMUState>, out_seq: *mut rosidl_runtime_rs::Sequence<IMUState>) -> bool;
}

// Corresponds to unitree_go__msg__IMUState
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
    pub temperature: i8,

}



impl Default for IMUState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__IMUState__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__IMUState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for IMUState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__IMUState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__IMUState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__IMUState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for IMUState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for IMUState where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/IMUState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__IMUState() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__InterfaceConfig() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__InterfaceConfig__init(msg: *mut InterfaceConfig) -> bool;
    fn unitree_go__msg__InterfaceConfig__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<InterfaceConfig>, size: usize) -> bool;
    fn unitree_go__msg__InterfaceConfig__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<InterfaceConfig>);
    fn unitree_go__msg__InterfaceConfig__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<InterfaceConfig>, out_seq: *mut rosidl_runtime_rs::Sequence<InterfaceConfig>) -> bool;
}

// Corresponds to unitree_go__msg__InterfaceConfig
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__InterfaceConfig__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__InterfaceConfig__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for InterfaceConfig {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__InterfaceConfig__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__InterfaceConfig__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__InterfaceConfig__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for InterfaceConfig {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for InterfaceConfig where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/InterfaceConfig";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__InterfaceConfig() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__LidarState() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__LidarState__init(msg: *mut LidarState) -> bool;
    fn unitree_go__msg__LidarState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LidarState>, size: usize) -> bool;
    fn unitree_go__msg__LidarState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LidarState>);
    fn unitree_go__msg__LidarState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LidarState>, out_seq: *mut rosidl_runtime_rs::Sequence<LidarState>) -> bool;
}

// Corresponds to unitree_go__msg__LidarState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LidarState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub firmware_version: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub software_version: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sdk_version: rosidl_runtime_rs::String,


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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__LidarState__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__LidarState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LidarState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__LidarState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__LidarState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__LidarState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LidarState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LidarState where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/LidarState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__LidarState() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__LowCmd() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__LowCmd__init(msg: *mut LowCmd) -> bool;
    fn unitree_go__msg__LowCmd__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LowCmd>, size: usize) -> bool;
    fn unitree_go__msg__LowCmd__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LowCmd>);
    fn unitree_go__msg__LowCmd__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LowCmd>, out_seq: *mut rosidl_runtime_rs::Sequence<LowCmd>) -> bool;
}

// Corresponds to unitree_go__msg__LowCmd
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    pub motor_cmd: [super::super::msg::rmw::MotorCmd; 20],


    // This member is not documented.
    #[allow(missing_docs)]
    pub bms_cmd: super::super::msg::rmw::BmsCmd,


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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__LowCmd__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__LowCmd__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LowCmd {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__LowCmd__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__LowCmd__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__LowCmd__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LowCmd {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LowCmd where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/LowCmd";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__LowCmd() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__LowState() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__LowState__init(msg: *mut LowState) -> bool;
    fn unitree_go__msg__LowState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LowState>, size: usize) -> bool;
    fn unitree_go__msg__LowState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LowState>);
    fn unitree_go__msg__LowState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LowState>, out_seq: *mut rosidl_runtime_rs::Sequence<LowState>) -> bool;
}

// Corresponds to unitree_go__msg__LowState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    pub imu_state: super::super::msg::rmw::IMUState,


    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_state: [super::super::msg::rmw::MotorState; 20],


    // This member is not documented.
    #[allow(missing_docs)]
    pub bms_state: super::super::msg::rmw::BmsState,


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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__LowState__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__LowState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LowState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__LowState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__LowState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__LowState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LowState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LowState where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/LowState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__LowState() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__MotorCmd() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__MotorCmd__init(msg: *mut MotorCmd) -> bool;
    fn unitree_go__msg__MotorCmd__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotorCmd>, size: usize) -> bool;
    fn unitree_go__msg__MotorCmd__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotorCmd>);
    fn unitree_go__msg__MotorCmd__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotorCmd>, out_seq: *mut rosidl_runtime_rs::Sequence<MotorCmd>) -> bool;
}

// Corresponds to unitree_go__msg__MotorCmd
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
    pub reserve: [u32; 3],

}



impl Default for MotorCmd {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__MotorCmd__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__MotorCmd__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotorCmd {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__MotorCmd__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__MotorCmd__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__MotorCmd__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotorCmd {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotorCmd where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/MotorCmd";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__MotorCmd() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__MotorCmds() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__MotorCmds__init(msg: *mut MotorCmds) -> bool;
    fn unitree_go__msg__MotorCmds__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotorCmds>, size: usize) -> bool;
    fn unitree_go__msg__MotorCmds__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotorCmds>);
    fn unitree_go__msg__MotorCmds__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotorCmds>, out_seq: *mut rosidl_runtime_rs::Sequence<MotorCmds>) -> bool;
}

// Corresponds to unitree_go__msg__MotorCmds
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotorCmds {

    // This member is not documented.
    #[allow(missing_docs)]
    pub cmds: rosidl_runtime_rs::Sequence<super::super::msg::rmw::MotorCmd>,

}



impl Default for MotorCmds {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__MotorCmds__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__MotorCmds__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotorCmds {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__MotorCmds__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__MotorCmds__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__MotorCmds__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotorCmds {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotorCmds where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/MotorCmds";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__MotorCmds() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__MotorState() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__MotorState__init(msg: *mut MotorState) -> bool;
    fn unitree_go__msg__MotorState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotorState>, size: usize) -> bool;
    fn unitree_go__msg__MotorState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotorState>);
    fn unitree_go__msg__MotorState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotorState>, out_seq: *mut rosidl_runtime_rs::Sequence<MotorState>) -> bool;
}

// Corresponds to unitree_go__msg__MotorState
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__MotorState__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__MotorState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotorState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__MotorState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__MotorState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__MotorState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotorState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotorState where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/MotorState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__MotorState() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__MotorStates() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__MotorStates__init(msg: *mut MotorStates) -> bool;
    fn unitree_go__msg__MotorStates__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotorStates>, size: usize) -> bool;
    fn unitree_go__msg__MotorStates__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotorStates>);
    fn unitree_go__msg__MotorStates__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotorStates>, out_seq: *mut rosidl_runtime_rs::Sequence<MotorStates>) -> bool;
}

// Corresponds to unitree_go__msg__MotorStates
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotorStates {

    // This member is not documented.
    #[allow(missing_docs)]
    pub states: rosidl_runtime_rs::Sequence<super::super::msg::rmw::MotorState>,

}



impl Default for MotorStates {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__MotorStates__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__MotorStates__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotorStates {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__MotorStates__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__MotorStates__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__MotorStates__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotorStates {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotorStates where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/MotorStates";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__MotorStates() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__PathPoint() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__PathPoint__init(msg: *mut PathPoint) -> bool;
    fn unitree_go__msg__PathPoint__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PathPoint>, size: usize) -> bool;
    fn unitree_go__msg__PathPoint__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PathPoint>);
    fn unitree_go__msg__PathPoint__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PathPoint>, out_seq: *mut rosidl_runtime_rs::Sequence<PathPoint>) -> bool;
}

// Corresponds to unitree_go__msg__PathPoint
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__PathPoint__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__PathPoint__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PathPoint {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__PathPoint__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__PathPoint__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__PathPoint__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PathPoint {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PathPoint where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/PathPoint";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__PathPoint() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__Req() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__Req__init(msg: *mut Req) -> bool;
    fn unitree_go__msg__Req__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Req>, size: usize) -> bool;
    fn unitree_go__msg__Req__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Req>);
    fn unitree_go__msg__Req__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Req>, out_seq: *mut rosidl_runtime_rs::Sequence<Req>) -> bool;
}

// Corresponds to unitree_go__msg__Req
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Req {

    // This member is not documented.
    #[allow(missing_docs)]
    pub uuid: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub body: rosidl_runtime_rs::String,

}



impl Default for Req {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__Req__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__Req__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Req {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__Req__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__Req__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__Req__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Req {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Req where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/Req";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__Req() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__Res() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__Res__init(msg: *mut Res) -> bool;
    fn unitree_go__msg__Res__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Res>, size: usize) -> bool;
    fn unitree_go__msg__Res__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Res>);
    fn unitree_go__msg__Res__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Res>, out_seq: *mut rosidl_runtime_rs::Sequence<Res>) -> bool;
}

// Corresponds to unitree_go__msg__Res
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Res {

    // This member is not documented.
    #[allow(missing_docs)]
    pub uuid: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: rosidl_runtime_rs::Sequence<u8>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub body: rosidl_runtime_rs::String,

}



impl Default for Res {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__Res__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__Res__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Res {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__Res__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__Res__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__Res__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Res {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Res where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/Res";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__Res() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__SportModeCmd() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__SportModeCmd__init(msg: *mut SportModeCmd) -> bool;
    fn unitree_go__msg__SportModeCmd__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SportModeCmd>, size: usize) -> bool;
    fn unitree_go__msg__SportModeCmd__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SportModeCmd>);
    fn unitree_go__msg__SportModeCmd__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SportModeCmd>, out_seq: *mut rosidl_runtime_rs::Sequence<SportModeCmd>) -> bool;
}

// Corresponds to unitree_go__msg__SportModeCmd
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    pub bms_cmd: super::super::msg::rmw::BmsCmd,


    // This member is not documented.
    #[allow(missing_docs)]
    pub path_point: [super::super::msg::rmw::PathPoint; 30],

}



impl Default for SportModeCmd {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__SportModeCmd__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__SportModeCmd__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SportModeCmd {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__SportModeCmd__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__SportModeCmd__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__SportModeCmd__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SportModeCmd {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SportModeCmd where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/SportModeCmd";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__SportModeCmd() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__SportModeState() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__SportModeState__init(msg: *mut SportModeState) -> bool;
    fn unitree_go__msg__SportModeState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SportModeState>, size: usize) -> bool;
    fn unitree_go__msg__SportModeState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SportModeState>);
    fn unitree_go__msg__SportModeState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SportModeState>, out_seq: *mut rosidl_runtime_rs::Sequence<SportModeState>) -> bool;
}

// Corresponds to unitree_go__msg__SportModeState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SportModeState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: super::super::msg::rmw::TimeSpec,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_code: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub imu_state: super::super::msg::rmw::IMUState,


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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__SportModeState__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__SportModeState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SportModeState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__SportModeState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__SportModeState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__SportModeState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SportModeState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SportModeState where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/SportModeState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__SportModeState() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__TimeSpec() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__TimeSpec__init(msg: *mut TimeSpec) -> bool;
    fn unitree_go__msg__TimeSpec__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TimeSpec>, size: usize) -> bool;
    fn unitree_go__msg__TimeSpec__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TimeSpec>);
    fn unitree_go__msg__TimeSpec__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TimeSpec>, out_seq: *mut rosidl_runtime_rs::Sequence<TimeSpec>) -> bool;
}

// Corresponds to unitree_go__msg__TimeSpec
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Time indicates a specific point in time, relative to a clock's 0 point.
/// The seconds component, valid over all int32 values.

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__TimeSpec__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__TimeSpec__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TimeSpec {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__TimeSpec__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__TimeSpec__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__TimeSpec__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TimeSpec {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TimeSpec where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/TimeSpec";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__TimeSpec() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__UwbState() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__UwbState__init(msg: *mut UwbState) -> bool;
    fn unitree_go__msg__UwbState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UwbState>, size: usize) -> bool;
    fn unitree_go__msg__UwbState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UwbState>);
    fn unitree_go__msg__UwbState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UwbState>, out_seq: *mut rosidl_runtime_rs::Sequence<UwbState>) -> bool;
}

// Corresponds to unitree_go__msg__UwbState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__UwbState__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__UwbState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UwbState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__UwbState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__UwbState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__UwbState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UwbState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UwbState where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/UwbState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__UwbState() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__UwbSwitch() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__UwbSwitch__init(msg: *mut UwbSwitch) -> bool;
    fn unitree_go__msg__UwbSwitch__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UwbSwitch>, size: usize) -> bool;
    fn unitree_go__msg__UwbSwitch__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UwbSwitch>);
    fn unitree_go__msg__UwbSwitch__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UwbSwitch>, out_seq: *mut rosidl_runtime_rs::Sequence<UwbSwitch>) -> bool;
}

// Corresponds to unitree_go__msg__UwbSwitch
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UwbSwitch {

    // This member is not documented.
    #[allow(missing_docs)]
    pub enabled: u8,

}



impl Default for UwbSwitch {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__UwbSwitch__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__UwbSwitch__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UwbSwitch {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__UwbSwitch__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__UwbSwitch__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__UwbSwitch__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UwbSwitch {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UwbSwitch where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/UwbSwitch";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__UwbSwitch() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__WirelessController() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__WirelessController__init(msg: *mut WirelessController) -> bool;
    fn unitree_go__msg__WirelessController__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WirelessController>, size: usize) -> bool;
    fn unitree_go__msg__WirelessController__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WirelessController>);
    fn unitree_go__msg__WirelessController__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WirelessController>, out_seq: *mut rosidl_runtime_rs::Sequence<WirelessController>) -> bool;
}

// Corresponds to unitree_go__msg__WirelessController
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__WirelessController__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__WirelessController__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WirelessController {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__WirelessController__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__WirelessController__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__WirelessController__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WirelessController {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WirelessController where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/WirelessController";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__WirelessController() }
  }
}



#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "unitree_api__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_api__msg__Request() -> *const std::ffi::c_void;
}

#[link(name = "unitree_api__rosidl_generator_c")]
extern "C" {
    fn unitree_api__msg__Request__init(msg: *mut Request) -> bool;
    fn unitree_api__msg__Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Request>, size: usize) -> bool;
    fn unitree_api__msg__Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Request>);
    fn unitree_api__msg__Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Request>) -> bool;
}

// Corresponds to unitree_api__msg__Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: super::super::msg::rmw::RequestHeader,


    // This member is not documented.
    #[allow(missing_docs)]
    pub parameter: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub binary: rosidl_runtime_rs::Sequence<u8>,

}



impl Default for Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_api__msg__Request__init(&mut msg as *mut _) {
        panic!("Call to unitree_api__msg__Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Request where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_api/msg/Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_api__msg__Request() }
  }
}


#[link(name = "unitree_api__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_api__msg__RequestHeader() -> *const std::ffi::c_void;
}

#[link(name = "unitree_api__rosidl_generator_c")]
extern "C" {
    fn unitree_api__msg__RequestHeader__init(msg: *mut RequestHeader) -> bool;
    fn unitree_api__msg__RequestHeader__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RequestHeader>, size: usize) -> bool;
    fn unitree_api__msg__RequestHeader__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RequestHeader>);
    fn unitree_api__msg__RequestHeader__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RequestHeader>, out_seq: *mut rosidl_runtime_rs::Sequence<RequestHeader>) -> bool;
}

// Corresponds to unitree_api__msg__RequestHeader
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RequestHeader {

    // This member is not documented.
    #[allow(missing_docs)]
    pub identity: super::super::msg::rmw::RequestIdentity,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lease: super::super::msg::rmw::RequestLease,


    // This member is not documented.
    #[allow(missing_docs)]
    pub policy: super::super::msg::rmw::RequestPolicy,

}



impl Default for RequestHeader {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_api__msg__RequestHeader__init(&mut msg as *mut _) {
        panic!("Call to unitree_api__msg__RequestHeader__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RequestHeader {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__RequestHeader__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__RequestHeader__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__RequestHeader__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RequestHeader {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RequestHeader where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_api/msg/RequestHeader";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_api__msg__RequestHeader() }
  }
}


#[link(name = "unitree_api__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_api__msg__RequestIdentity() -> *const std::ffi::c_void;
}

#[link(name = "unitree_api__rosidl_generator_c")]
extern "C" {
    fn unitree_api__msg__RequestIdentity__init(msg: *mut RequestIdentity) -> bool;
    fn unitree_api__msg__RequestIdentity__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RequestIdentity>, size: usize) -> bool;
    fn unitree_api__msg__RequestIdentity__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RequestIdentity>);
    fn unitree_api__msg__RequestIdentity__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RequestIdentity>, out_seq: *mut rosidl_runtime_rs::Sequence<RequestIdentity>) -> bool;
}

// Corresponds to unitree_api__msg__RequestIdentity
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RequestIdentity {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub api_id: i64,

}



impl Default for RequestIdentity {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_api__msg__RequestIdentity__init(&mut msg as *mut _) {
        panic!("Call to unitree_api__msg__RequestIdentity__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RequestIdentity {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__RequestIdentity__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__RequestIdentity__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__RequestIdentity__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RequestIdentity {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RequestIdentity where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_api/msg/RequestIdentity";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_api__msg__RequestIdentity() }
  }
}


#[link(name = "unitree_api__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_api__msg__RequestLease() -> *const std::ffi::c_void;
}

#[link(name = "unitree_api__rosidl_generator_c")]
extern "C" {
    fn unitree_api__msg__RequestLease__init(msg: *mut RequestLease) -> bool;
    fn unitree_api__msg__RequestLease__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RequestLease>, size: usize) -> bool;
    fn unitree_api__msg__RequestLease__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RequestLease>);
    fn unitree_api__msg__RequestLease__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RequestLease>, out_seq: *mut rosidl_runtime_rs::Sequence<RequestLease>) -> bool;
}

// Corresponds to unitree_api__msg__RequestLease
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RequestLease {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: i64,

}



impl Default for RequestLease {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_api__msg__RequestLease__init(&mut msg as *mut _) {
        panic!("Call to unitree_api__msg__RequestLease__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RequestLease {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__RequestLease__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__RequestLease__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__RequestLease__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RequestLease {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RequestLease where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_api/msg/RequestLease";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_api__msg__RequestLease() }
  }
}


#[link(name = "unitree_api__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_api__msg__RequestPolicy() -> *const std::ffi::c_void;
}

#[link(name = "unitree_api__rosidl_generator_c")]
extern "C" {
    fn unitree_api__msg__RequestPolicy__init(msg: *mut RequestPolicy) -> bool;
    fn unitree_api__msg__RequestPolicy__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RequestPolicy>, size: usize) -> bool;
    fn unitree_api__msg__RequestPolicy__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RequestPolicy>);
    fn unitree_api__msg__RequestPolicy__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RequestPolicy>, out_seq: *mut rosidl_runtime_rs::Sequence<RequestPolicy>) -> bool;
}

// Corresponds to unitree_api__msg__RequestPolicy
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RequestPolicy {

    // This member is not documented.
    #[allow(missing_docs)]
    pub priority: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub noreply: bool,

}



impl Default for RequestPolicy {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_api__msg__RequestPolicy__init(&mut msg as *mut _) {
        panic!("Call to unitree_api__msg__RequestPolicy__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RequestPolicy {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__RequestPolicy__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__RequestPolicy__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__RequestPolicy__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RequestPolicy {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RequestPolicy where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_api/msg/RequestPolicy";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_api__msg__RequestPolicy() }
  }
}


#[link(name = "unitree_api__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_api__msg__Response() -> *const std::ffi::c_void;
}

#[link(name = "unitree_api__rosidl_generator_c")]
extern "C" {
    fn unitree_api__msg__Response__init(msg: *mut Response) -> bool;
    fn unitree_api__msg__Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Response>, size: usize) -> bool;
    fn unitree_api__msg__Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Response>);
    fn unitree_api__msg__Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Response>) -> bool;
}

// Corresponds to unitree_api__msg__Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: super::super::msg::rmw::ResponseHeader,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub binary: rosidl_runtime_rs::Sequence<i8>,

}



impl Default for Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_api__msg__Response__init(&mut msg as *mut _) {
        panic!("Call to unitree_api__msg__Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Response where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_api/msg/Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_api__msg__Response() }
  }
}


#[link(name = "unitree_api__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_api__msg__ResponseHeader() -> *const std::ffi::c_void;
}

#[link(name = "unitree_api__rosidl_generator_c")]
extern "C" {
    fn unitree_api__msg__ResponseHeader__init(msg: *mut ResponseHeader) -> bool;
    fn unitree_api__msg__ResponseHeader__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ResponseHeader>, size: usize) -> bool;
    fn unitree_api__msg__ResponseHeader__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ResponseHeader>);
    fn unitree_api__msg__ResponseHeader__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ResponseHeader>, out_seq: *mut rosidl_runtime_rs::Sequence<ResponseHeader>) -> bool;
}

// Corresponds to unitree_api__msg__ResponseHeader
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ResponseHeader {

    // This member is not documented.
    #[allow(missing_docs)]
    pub identity: super::super::msg::rmw::RequestIdentity,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::super::msg::rmw::ResponseStatus,

}



impl Default for ResponseHeader {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_api__msg__ResponseHeader__init(&mut msg as *mut _) {
        panic!("Call to unitree_api__msg__ResponseHeader__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ResponseHeader {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__ResponseHeader__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__ResponseHeader__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__ResponseHeader__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ResponseHeader {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ResponseHeader where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_api/msg/ResponseHeader";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_api__msg__ResponseHeader() }
  }
}


#[link(name = "unitree_api__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_api__msg__ResponseStatus() -> *const std::ffi::c_void;
}

#[link(name = "unitree_api__rosidl_generator_c")]
extern "C" {
    fn unitree_api__msg__ResponseStatus__init(msg: *mut ResponseStatus) -> bool;
    fn unitree_api__msg__ResponseStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ResponseStatus>, size: usize) -> bool;
    fn unitree_api__msg__ResponseStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ResponseStatus>);
    fn unitree_api__msg__ResponseStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ResponseStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<ResponseStatus>) -> bool;
}

// Corresponds to unitree_api__msg__ResponseStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ResponseStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub code: i32,

}



impl Default for ResponseStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_api__msg__ResponseStatus__init(&mut msg as *mut _) {
        panic!("Call to unitree_api__msg__ResponseStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ResponseStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__ResponseStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__ResponseStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_api__msg__ResponseStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ResponseStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ResponseStatus where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_api/msg/ResponseStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_api__msg__ResponseStatus() }
  }
}



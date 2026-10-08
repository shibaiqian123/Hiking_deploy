#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to unitree_api__msg__Request

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: super::msg::RequestHeader,


    // This member is not documented.
    #[allow(missing_docs)]
    pub parameter: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub binary: Vec<u8>,

}



impl Default for Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Request::default())
  }
}

impl rosidl_runtime_rs::Message for Request {
  type RmwMsg = super::msg::rmw::Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: super::msg::RequestHeader::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        parameter: msg.parameter.as_str().into(),
        binary: msg.binary.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: super::msg::RequestHeader::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        parameter: msg.parameter.as_str().into(),
        binary: msg.binary.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: super::msg::RequestHeader::from_rmw_message(msg.header),
      parameter: msg.parameter.to_string(),
      binary: msg.binary
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to unitree_api__msg__RequestHeader

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RequestHeader {

    // This member is not documented.
    #[allow(missing_docs)]
    pub identity: super::msg::RequestIdentity,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lease: super::msg::RequestLease,


    // This member is not documented.
    #[allow(missing_docs)]
    pub policy: super::msg::RequestPolicy,

}



impl Default for RequestHeader {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::RequestHeader::default())
  }
}

impl rosidl_runtime_rs::Message for RequestHeader {
  type RmwMsg = super::msg::rmw::RequestHeader;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        identity: super::msg::RequestIdentity::into_rmw_message(std::borrow::Cow::Owned(msg.identity)).into_owned(),
        lease: super::msg::RequestLease::into_rmw_message(std::borrow::Cow::Owned(msg.lease)).into_owned(),
        policy: super::msg::RequestPolicy::into_rmw_message(std::borrow::Cow::Owned(msg.policy)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        identity: super::msg::RequestIdentity::into_rmw_message(std::borrow::Cow::Borrowed(&msg.identity)).into_owned(),
        lease: super::msg::RequestLease::into_rmw_message(std::borrow::Cow::Borrowed(&msg.lease)).into_owned(),
        policy: super::msg::RequestPolicy::into_rmw_message(std::borrow::Cow::Borrowed(&msg.policy)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      identity: super::msg::RequestIdentity::from_rmw_message(msg.identity),
      lease: super::msg::RequestLease::from_rmw_message(msg.lease),
      policy: super::msg::RequestPolicy::from_rmw_message(msg.policy),
    }
  }
}


// Corresponds to unitree_api__msg__RequestIdentity

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::RequestIdentity::default())
  }
}

impl rosidl_runtime_rs::Message for RequestIdentity {
  type RmwMsg = super::msg::rmw::RequestIdentity;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
        api_id: msg.api_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
      api_id: msg.api_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
      api_id: msg.api_id,
    }
  }
}


// Corresponds to unitree_api__msg__RequestLease

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RequestLease {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: i64,

}



impl Default for RequestLease {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::RequestLease::default())
  }
}

impl rosidl_runtime_rs::Message for RequestLease {
  type RmwMsg = super::msg::rmw::RequestLease;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
    }
  }
}


// Corresponds to unitree_api__msg__RequestPolicy

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::RequestPolicy::default())
  }
}

impl rosidl_runtime_rs::Message for RequestPolicy {
  type RmwMsg = super::msg::rmw::RequestPolicy;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        priority: msg.priority,
        noreply: msg.noreply,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      priority: msg.priority,
      noreply: msg.noreply,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      priority: msg.priority,
      noreply: msg.noreply,
    }
  }
}


// Corresponds to unitree_api__msg__Response

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: super::msg::ResponseHeader,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub binary: Vec<i8>,

}



impl Default for Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Response::default())
  }
}

impl rosidl_runtime_rs::Message for Response {
  type RmwMsg = super::msg::rmw::Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: super::msg::ResponseHeader::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        data: msg.data.as_str().into(),
        binary: msg.binary.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: super::msg::ResponseHeader::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        data: msg.data.as_str().into(),
        binary: msg.binary.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: super::msg::ResponseHeader::from_rmw_message(msg.header),
      data: msg.data.to_string(),
      binary: msg.binary
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to unitree_api__msg__ResponseHeader

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ResponseHeader {

    // This member is not documented.
    #[allow(missing_docs)]
    pub identity: super::msg::RequestIdentity,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::msg::ResponseStatus,

}



impl Default for ResponseHeader {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ResponseHeader::default())
  }
}

impl rosidl_runtime_rs::Message for ResponseHeader {
  type RmwMsg = super::msg::rmw::ResponseHeader;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        identity: super::msg::RequestIdentity::into_rmw_message(std::borrow::Cow::Owned(msg.identity)).into_owned(),
        status: super::msg::ResponseStatus::into_rmw_message(std::borrow::Cow::Owned(msg.status)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        identity: super::msg::RequestIdentity::into_rmw_message(std::borrow::Cow::Borrowed(&msg.identity)).into_owned(),
        status: super::msg::ResponseStatus::into_rmw_message(std::borrow::Cow::Borrowed(&msg.status)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      identity: super::msg::RequestIdentity::from_rmw_message(msg.identity),
      status: super::msg::ResponseStatus::from_rmw_message(msg.status),
    }
  }
}


// Corresponds to unitree_api__msg__ResponseStatus

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ResponseStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub code: i32,

}



impl Default for ResponseStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ResponseStatus::default())
  }
}

impl rosidl_runtime_rs::Message for ResponseStatus {
  type RmwMsg = super::msg::rmw::ResponseStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        code: msg.code,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      code: msg.code,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      code: msg.code,
    }
  }
}



#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to frhal_msgs__srv__ROSCmdInterface_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ROSCmdInterface_Request {
    /// ros用户输入的字符串指令，比如movej(p1,100)
    pub cmd_str: std::string::String,

}



impl Default for ROSCmdInterface_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ROSCmdInterface_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ROSCmdInterface_Request {
  type RmwMsg = super::srv::rmw::ROSCmdInterface_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        cmd_str: msg.cmd_str.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        cmd_str: msg.cmd_str.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      cmd_str: msg.cmd_str.to_string(),
    }
  }
}


// Corresponds to frhal_msgs__srv__ROSCmdInterface_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ROSCmdInterface_Response {
    /// 创建结果反馈，0-成功，-1-失败
    pub cmd_res: std::string::String,

}



impl Default for ROSCmdInterface_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ROSCmdInterface_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ROSCmdInterface_Response {
  type RmwMsg = super::srv::rmw::ROSCmdInterface_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        cmd_res: msg.cmd_res.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        cmd_res: msg.cmd_res.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      cmd_res: msg.cmd_res.to_string(),
    }
  }
}






#[link(name = "frhal_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__frhal_msgs__srv__ROSCmdInterface() -> *const std::ffi::c_void;
}

// Corresponds to frhal_msgs__srv__ROSCmdInterface
#[allow(missing_docs, non_camel_case_types)]
pub struct ROSCmdInterface;

impl rosidl_runtime_rs::Service for ROSCmdInterface {
    type Request = ROSCmdInterface_Request;
    type Response = ROSCmdInterface_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__frhal_msgs__srv__ROSCmdInterface() }
    }
}



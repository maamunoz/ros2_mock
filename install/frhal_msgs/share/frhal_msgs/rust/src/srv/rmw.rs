#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "frhal_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__frhal_msgs__srv__ROSCmdInterface_Request() -> *const std::ffi::c_void;
}

#[link(name = "frhal_msgs__rosidl_generator_c")]
extern "C" {
    fn frhal_msgs__srv__ROSCmdInterface_Request__init(msg: *mut ROSCmdInterface_Request) -> bool;
    fn frhal_msgs__srv__ROSCmdInterface_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ROSCmdInterface_Request>, size: usize) -> bool;
    fn frhal_msgs__srv__ROSCmdInterface_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ROSCmdInterface_Request>);
    fn frhal_msgs__srv__ROSCmdInterface_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ROSCmdInterface_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ROSCmdInterface_Request>) -> bool;
}

// Corresponds to frhal_msgs__srv__ROSCmdInterface_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ROSCmdInterface_Request {
    /// ros用户输入的字符串指令，比如movej(p1,100)
    pub cmd_str: rosidl_runtime_rs::String,

}



impl Default for ROSCmdInterface_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !frhal_msgs__srv__ROSCmdInterface_Request__init(&mut msg as *mut _) {
        panic!("Call to frhal_msgs__srv__ROSCmdInterface_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ROSCmdInterface_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { frhal_msgs__srv__ROSCmdInterface_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { frhal_msgs__srv__ROSCmdInterface_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { frhal_msgs__srv__ROSCmdInterface_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ROSCmdInterface_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ROSCmdInterface_Request where Self: Sized {
  const TYPE_NAME: &'static str = "frhal_msgs/srv/ROSCmdInterface_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__frhal_msgs__srv__ROSCmdInterface_Request() }
  }
}


#[link(name = "frhal_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__frhal_msgs__srv__ROSCmdInterface_Response() -> *const std::ffi::c_void;
}

#[link(name = "frhal_msgs__rosidl_generator_c")]
extern "C" {
    fn frhal_msgs__srv__ROSCmdInterface_Response__init(msg: *mut ROSCmdInterface_Response) -> bool;
    fn frhal_msgs__srv__ROSCmdInterface_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ROSCmdInterface_Response>, size: usize) -> bool;
    fn frhal_msgs__srv__ROSCmdInterface_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ROSCmdInterface_Response>);
    fn frhal_msgs__srv__ROSCmdInterface_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ROSCmdInterface_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ROSCmdInterface_Response>) -> bool;
}

// Corresponds to frhal_msgs__srv__ROSCmdInterface_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ROSCmdInterface_Response {
    /// 创建结果反馈，0-成功，-1-失败
    pub cmd_res: rosidl_runtime_rs::String,

}



impl Default for ROSCmdInterface_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !frhal_msgs__srv__ROSCmdInterface_Response__init(&mut msg as *mut _) {
        panic!("Call to frhal_msgs__srv__ROSCmdInterface_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ROSCmdInterface_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { frhal_msgs__srv__ROSCmdInterface_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { frhal_msgs__srv__ROSCmdInterface_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { frhal_msgs__srv__ROSCmdInterface_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ROSCmdInterface_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ROSCmdInterface_Response where Self: Sized {
  const TYPE_NAME: &'static str = "frhal_msgs/srv/ROSCmdInterface_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__frhal_msgs__srv__ROSCmdInterface_Response() }
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



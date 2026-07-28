#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "frhal_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__frhal_msgs__msg__FRState() -> *const std::ffi::c_void;
}

#[link(name = "frhal_msgs__rosidl_generator_c")]
extern "C" {
    fn frhal_msgs__msg__FRState__init(msg: *mut FRState) -> bool;
    fn frhal_msgs__msg__FRState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FRState>, size: usize) -> bool;
    fn frhal_msgs__msg__FRState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FRState>);
    fn frhal_msgs__msg__FRState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FRState>, out_seq: *mut rosidl_runtime_rs::Sequence<FRState>) -> bool;
}

// Corresponds to frhal_msgs__msg__FRState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FRState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub prg_state: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_code: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_mode: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j1_cur_pos: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j2_cur_pos: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j3_cur_pos: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j4_cur_pos: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j5_cur_pos: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j6_cur_pos: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cart_x_cur_pos: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cart_y_cur_pos: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cart_z_cur_pos: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cart_a_cur_pos: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cart_b_cur_pos: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cart_c_cur_pos: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tool_num: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub work_num: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j1_cur_tor: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j2_cur_tor: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j3_cur_tor: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j4_cur_tor: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j5_cur_tor: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j6_cur_tor: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub prg_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub prg_total_line: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub prg_cur_line: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub dgt_output_h: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub dgt_output_l: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tl_dgt_output_l: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub dgt_input_h: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub dgt_input_l: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tl_dgt_input_l: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ft_fx_data: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ft_fy_data: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ft_fz_data: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ft_tx_data: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ft_ty_data: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ft_tz_data: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ft_actstatus: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub emg: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_motion_done: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub grip_motion_done: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub exaxispos1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub exaxispos2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub exaxispos3: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub exaxispos4: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub check_sum: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub start_return: rosidl_runtime_rs::String,

}



impl Default for FRState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !frhal_msgs__msg__FRState__init(&mut msg as *mut _) {
        panic!("Call to frhal_msgs__msg__FRState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FRState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { frhal_msgs__msg__FRState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { frhal_msgs__msg__FRState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { frhal_msgs__msg__FRState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FRState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FRState where Self: Sized {
  const TYPE_NAME: &'static str = "frhal_msgs/msg/FRState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__frhal_msgs__msg__FRState() }
  }
}



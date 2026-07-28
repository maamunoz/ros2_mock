#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to frhal_msgs__msg__FRState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub prg_name: std::string::String,


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
    pub start_return: std::string::String,

}



impl Default for FRState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::FRState::default())
  }
}

impl rosidl_runtime_rs::Message for FRState {
  type RmwMsg = super::msg::rmw::FRState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        prg_state: msg.prg_state,
        error_code: msg.error_code,
        robot_mode: msg.robot_mode,
        j1_cur_pos: msg.j1_cur_pos,
        j2_cur_pos: msg.j2_cur_pos,
        j3_cur_pos: msg.j3_cur_pos,
        j4_cur_pos: msg.j4_cur_pos,
        j5_cur_pos: msg.j5_cur_pos,
        j6_cur_pos: msg.j6_cur_pos,
        cart_x_cur_pos: msg.cart_x_cur_pos,
        cart_y_cur_pos: msg.cart_y_cur_pos,
        cart_z_cur_pos: msg.cart_z_cur_pos,
        cart_a_cur_pos: msg.cart_a_cur_pos,
        cart_b_cur_pos: msg.cart_b_cur_pos,
        cart_c_cur_pos: msg.cart_c_cur_pos,
        tool_num: msg.tool_num,
        work_num: msg.work_num,
        j1_cur_tor: msg.j1_cur_tor,
        j2_cur_tor: msg.j2_cur_tor,
        j3_cur_tor: msg.j3_cur_tor,
        j4_cur_tor: msg.j4_cur_tor,
        j5_cur_tor: msg.j5_cur_tor,
        j6_cur_tor: msg.j6_cur_tor,
        prg_name: msg.prg_name.as_str().into(),
        prg_total_line: msg.prg_total_line,
        prg_cur_line: msg.prg_cur_line,
        dgt_output_h: msg.dgt_output_h,
        dgt_output_l: msg.dgt_output_l,
        tl_dgt_output_l: msg.tl_dgt_output_l,
        dgt_input_h: msg.dgt_input_h,
        dgt_input_l: msg.dgt_input_l,
        tl_dgt_input_l: msg.tl_dgt_input_l,
        ft_fx_data: msg.ft_fx_data,
        ft_fy_data: msg.ft_fy_data,
        ft_fz_data: msg.ft_fz_data,
        ft_tx_data: msg.ft_tx_data,
        ft_ty_data: msg.ft_ty_data,
        ft_tz_data: msg.ft_tz_data,
        ft_actstatus: msg.ft_actstatus,
        emg: msg.emg,
        robot_motion_done: msg.robot_motion_done,
        grip_motion_done: msg.grip_motion_done,
        exaxispos1: msg.exaxispos1,
        exaxispos2: msg.exaxispos2,
        exaxispos3: msg.exaxispos3,
        exaxispos4: msg.exaxispos4,
        check_sum: msg.check_sum,
        start_return: msg.start_return.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      prg_state: msg.prg_state,
      error_code: msg.error_code,
      robot_mode: msg.robot_mode,
      j1_cur_pos: msg.j1_cur_pos,
      j2_cur_pos: msg.j2_cur_pos,
      j3_cur_pos: msg.j3_cur_pos,
      j4_cur_pos: msg.j4_cur_pos,
      j5_cur_pos: msg.j5_cur_pos,
      j6_cur_pos: msg.j6_cur_pos,
      cart_x_cur_pos: msg.cart_x_cur_pos,
      cart_y_cur_pos: msg.cart_y_cur_pos,
      cart_z_cur_pos: msg.cart_z_cur_pos,
      cart_a_cur_pos: msg.cart_a_cur_pos,
      cart_b_cur_pos: msg.cart_b_cur_pos,
      cart_c_cur_pos: msg.cart_c_cur_pos,
      tool_num: msg.tool_num,
      work_num: msg.work_num,
      j1_cur_tor: msg.j1_cur_tor,
      j2_cur_tor: msg.j2_cur_tor,
      j3_cur_tor: msg.j3_cur_tor,
      j4_cur_tor: msg.j4_cur_tor,
      j5_cur_tor: msg.j5_cur_tor,
      j6_cur_tor: msg.j6_cur_tor,
        prg_name: msg.prg_name.as_str().into(),
      prg_total_line: msg.prg_total_line,
      prg_cur_line: msg.prg_cur_line,
      dgt_output_h: msg.dgt_output_h,
      dgt_output_l: msg.dgt_output_l,
      tl_dgt_output_l: msg.tl_dgt_output_l,
      dgt_input_h: msg.dgt_input_h,
      dgt_input_l: msg.dgt_input_l,
      tl_dgt_input_l: msg.tl_dgt_input_l,
      ft_fx_data: msg.ft_fx_data,
      ft_fy_data: msg.ft_fy_data,
      ft_fz_data: msg.ft_fz_data,
      ft_tx_data: msg.ft_tx_data,
      ft_ty_data: msg.ft_ty_data,
      ft_tz_data: msg.ft_tz_data,
      ft_actstatus: msg.ft_actstatus,
      emg: msg.emg,
      robot_motion_done: msg.robot_motion_done,
      grip_motion_done: msg.grip_motion_done,
      exaxispos1: msg.exaxispos1,
      exaxispos2: msg.exaxispos2,
      exaxispos3: msg.exaxispos3,
      exaxispos4: msg.exaxispos4,
      check_sum: msg.check_sum,
        start_return: msg.start_return.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      prg_state: msg.prg_state,
      error_code: msg.error_code,
      robot_mode: msg.robot_mode,
      j1_cur_pos: msg.j1_cur_pos,
      j2_cur_pos: msg.j2_cur_pos,
      j3_cur_pos: msg.j3_cur_pos,
      j4_cur_pos: msg.j4_cur_pos,
      j5_cur_pos: msg.j5_cur_pos,
      j6_cur_pos: msg.j6_cur_pos,
      cart_x_cur_pos: msg.cart_x_cur_pos,
      cart_y_cur_pos: msg.cart_y_cur_pos,
      cart_z_cur_pos: msg.cart_z_cur_pos,
      cart_a_cur_pos: msg.cart_a_cur_pos,
      cart_b_cur_pos: msg.cart_b_cur_pos,
      cart_c_cur_pos: msg.cart_c_cur_pos,
      tool_num: msg.tool_num,
      work_num: msg.work_num,
      j1_cur_tor: msg.j1_cur_tor,
      j2_cur_tor: msg.j2_cur_tor,
      j3_cur_tor: msg.j3_cur_tor,
      j4_cur_tor: msg.j4_cur_tor,
      j5_cur_tor: msg.j5_cur_tor,
      j6_cur_tor: msg.j6_cur_tor,
      prg_name: msg.prg_name.to_string(),
      prg_total_line: msg.prg_total_line,
      prg_cur_line: msg.prg_cur_line,
      dgt_output_h: msg.dgt_output_h,
      dgt_output_l: msg.dgt_output_l,
      tl_dgt_output_l: msg.tl_dgt_output_l,
      dgt_input_h: msg.dgt_input_h,
      dgt_input_l: msg.dgt_input_l,
      tl_dgt_input_l: msg.tl_dgt_input_l,
      ft_fx_data: msg.ft_fx_data,
      ft_fy_data: msg.ft_fy_data,
      ft_fz_data: msg.ft_fz_data,
      ft_tx_data: msg.ft_tx_data,
      ft_ty_data: msg.ft_ty_data,
      ft_tz_data: msg.ft_tz_data,
      ft_actstatus: msg.ft_actstatus,
      emg: msg.emg,
      robot_motion_done: msg.robot_motion_done,
      grip_motion_done: msg.grip_motion_done,
      exaxispos1: msg.exaxispos1,
      exaxispos2: msg.exaxispos2,
      exaxispos3: msg.exaxispos3,
      exaxispos4: msg.exaxispos4,
      check_sum: msg.check_sum,
      start_return: msg.start_return.to_string(),
    }
  }
}



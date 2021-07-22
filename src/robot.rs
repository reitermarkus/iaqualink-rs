use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Durations {
  custom_tim: usize,
  deep_tim: usize,
  first_smart_tim: usize,
  quick_tim: usize,
  smart_tim: usize,
  water_tim: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct RobotSensor {
  #[serde(rename = "type")]
  sensor_type: String,
  state: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct RobotSensors {
  sns_1: RobotSensor,
  sns_2: RobotSensor,
  sns_3: RobotSensor,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Robot {
  canister: usize,
  custom_cyc: usize,
  cycle_start_time: usize,
  durations: Durations,
  equipment_id: String,
  error_code: usize,
  error_state: usize,
  first_smrt_flag: usize,
  lift_control: usize,
  logger: usize,
  pr_cyc: usize,
  repeat: usize,
  #[serde(rename = "rmt_ctrl")]
  rmt_ctrl: usize,
  scan_time_duration: usize,
  sch_conf_0_enable: usize,
  sch_conf_0_hour: usize,
  sch_conf_0_min: usize,
  sch_conf_0_prt: usize,
  #[serde(rename = "schConf0WDay")]
  sch_conf_0_wday: usize,
  sch_conf_1_enable: usize,
  sch_conf_1_hour: usize,
  sch_conf_1_min: usize,
  sch_conf_1_prt: usize,
  #[serde(rename = "schConf1WDay")]
  sch_conf_1_wday: usize,
  sch_conf_2_enable: usize,
  sch_conf_2_hour: usize,
  sch_conf_2_min: usize,
  sch_conf_2_prt: usize,
  #[serde(rename = "schConf2WDay")]
  sch_conf_2_wday: usize,
  sch_conf_3_enable: usize,
  sch_conf_3_hour: usize,
  sch_conf_3_min: usize,
  sch_conf_3_prt: usize,
  #[serde(rename = "schConf3WDay")]
  sch_conf_3_wday: usize,
  sch_conf_4_enable: usize,
  sch_conf_4_hour: usize,
  sch_conf_4_min: usize,
  sch_conf_4_prt: usize,
  #[serde(rename = "schConf4WDay")]
  sch_conf_4_wday: usize,
  sch_conf_5_enable: usize,
  sch_conf_5_hour: usize,
  sch_conf_5_min: usize,
  sch_conf_5_prt: usize,
  #[serde(rename = "schConf5WDay")]
  sch_conf_5_wday: usize,
  sch_conf_6_enable: usize,
  sch_conf_6_hour: usize,
  sch_conf_6_min: usize,
  sch_conf_6_prt: usize,
  #[serde(rename = "schConf6WDay")]
  sch_conf_6_wday: usize,
  sensors: RobotSensors,
  state: usize,
  stepper: usize,
  stepper_adj_time: usize,
  total_hours: usize,
  vr: String,
  custom_intensity: usize,
}

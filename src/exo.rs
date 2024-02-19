use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::AwsState;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Sensor {
  pub sensor_type: String,
  pub state: usize,
  pub value: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct FilterPump {
  #[serde(rename = "type")]
  pub pump_type: usize,
  pub state: usize,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Color {
  AlpineWhite      = 0,
  SkyBlue          = 1,
  CobaltBlue       = 2,
  CarribeanBlue    = 3,
  SpringGreen      = 4,
  EmeraldGreen     = 5,
  EmeraldRose      = 6,
  Magenta          = 7,
  Violet           = 8,
  SlowFade         = 9,
  FastFade         = 10,
  BeautifulAmerica = 11,
  CarnivalTuesday  = 12,
  DiscoStyle       = 13,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Auxiliary {
  #[serde(rename = "type")]
  pub aux_type: String,
  pub color: Color,
  pub mode: usize,
  pub state: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct VspSpeed {
  pub min: usize,
  pub max: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SaltWaterChlorinator {
  pub amp: usize,
  pub temp: usize,
  pub vr: String,
  pub sns_3: Sensor,
  pub orp_sp: usize,
  pub production: usize,
  pub boost: usize,
  pub ph_sp: usize,
  pub swc: usize,
  pub version: String,
  pub sns_2: Sensor,
  pub low: usize,
  pub vsp: usize,
  pub lang: usize,
  pub ph_only: usize,
  pub sns_1: Sensor,
  pub aux_1: Auxiliary,
  pub swc_low: usize,
  pub dual_link: usize,
  pub vsp_speed: VspSpeed,
  pub exo_state: usize,
  pub aux_2: Auxiliary,
  pub boost_time: String,
  pub error_code: usize,
  pub aux230: usize,
  pub error_state: usize,
  pub sn: String,
  pub filter_pump: FilterPump,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SaltWaterChlorinatorEquipment {
  swc_0: SaltWaterChlorinator,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Timer {
  pub start: String,
  pub end: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Schedule {
  pub active: usize,
  pub enabled: usize,
  pub endpoint: String,
  pub id: String,
  pub name: String,
  pub timer: Timer,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Schedules {
  pub programmed: usize,
  pub supported: usize,
  pub sch1: Schedule,
  pub sch2: Schedule,
  pub sch3: Schedule,
  pub sch4: Schedule,
  pub sch9: Schedule,
  pub sch10: Schedule,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Heating {
  pub enabled: usize,
  pub priority_enabled: usize,
  pub sp: usize,
  pub sp_max: usize,
  pub sp_min: usize,
  pub state: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ExoState {
  pub aws: AwsState,
  pub equipment: SaltWaterChlorinatorEquipment,
  pub vr: String,
  pub main: Option<serde_json::Value>,
  pub hmi: Option<serde_json::Value>,
  pub schedules: Option<Schedules>,
  pub state: Option<serde_json::Value>,
  pub heating: Option<Heating>,
  pub debug: Option<serde_json::Value>,
  pub debug_main: Option<serde_json::Value>,
}

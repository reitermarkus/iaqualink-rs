use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Sensor {
  sensor_type: String,
  state: usize,
  value: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct FilterPump {
  #[serde(rename = "type")]
  pump_type: usize,
  state: usize,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Color {
  AlpineWhite = 0,
  SkyBlue = 1,
  CobaltBlue = 2,
  CarribeanBlue = 3,
  SpringGreen = 4,
  EmeraldGreen = 5,
  EmeraldRose = 6,
  Magenta = 7,
  Violet = 8,
  SlowFade = 9,
  FastFade = 10,
  BeautifulAmerica = 11,
  CarnivalTuesday = 12,
  DiscoStyle = 13,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Auxiliary {
  #[serde(rename = "type")]
  aux_type: String,
  color: Color,
  mode: usize,
  state: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SaltWaterChlorinator {
  amp: usize,
  temp: usize,
  vr: String,
  sns_3: Sensor,
  orp_sp: usize,
  production: usize,
  boost: usize,
  ph_sp: usize,
  swc: usize,
  version: String,
  sns_2: Sensor,
  low: usize,
  vsp: usize,
  lang: usize,
  ph_only: usize,
  sns_1: Sensor,
  aux_1: Auxiliary,
  swc_low: usize,
  dual_link: usize,
  exo_state: usize,
  aux_2: Auxiliary,
  boost_time: String,
  error_code: usize,
  aux230: usize,
  error_state: usize,
  sn: String,
  filter_pump: FilterPump,
}

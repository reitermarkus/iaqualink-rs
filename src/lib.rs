use std::collections::HashMap;

use serde_repr::*;

pub const HEADER_USER_AGENT: &'static str = "iAqualink/98 CFNetwork/978.0.7 Darwin/18.6.0";
pub const HEADER_ACCEPT: &'static str = "*/*";
pub const HEADER_ACCEPT_LANGUAGE: &'static str = "en-us";
pub const HEADER_ACCEPT_ENCODING: &'static str = "br, gzip, deflate";

pub const IAQUALINK_API_KEY: &'static str = "EOOEMOW4YR6QNB07";
pub const IAQUALINK_SESSION_URL: &'static str = "https://p-api.iaqualink.net/v1/mobile/session.json";
pub const IAQUALINK_LOGIN_URL: &'static str = "https://prod.zodiac-io.com/users/v1/login";
pub const IAQUALINK_DEVICES_URL: &'static str = "https://r-api.iaqualink.net/devices.json";
pub const IAQUALINK_DEVICES_V1_URL: &'static str = "https://prod.zodiac-io.com/devices/v1/";
pub const IAQUALINK_DEVICES_V2_URL: &'static str = "https://prod.zodiac-io.com/devices/v2/";
pub const IAQUALINK_PROD_URL: &'static str = "https://storage.googleapis.com/zodiac-iaqua-message-prod/message.json";

pub const IAQUALINK_COMMAND_GET_DEVICES: &'static str = "get_devices";
pub const IAQUALINK_COMMAND_GET_HOME: &'static str = "get_home";
pub const IAQUALINK_COMMAND_GET_ONETOUCH: &'static str = "get_onetouch";
pub const IAQUALINK_COMMAND_SET_AUX: &'static str = "set_aux";
pub const IAQUALINK_COMMAND_SET_LIGHT: &'static str = "set_light";
pub const IAQUALINK_COMMAND_SET_POOL_HEATER: &'static str = "set_pool_heater";
pub const IAQUALINK_COMMAND_SET_POOL_PUMP: &'static str = "set_pool_pump";
pub const IAQUALINK_COMMAND_SET_SOLAR_HEATER: &'static str = "set_solar_heater";
pub const IAQUALINK_COMMAND_SET_SPA_HEATER: &'static str = "set_spa_heater";
pub const IAQUALINK_COMMAND_SET_SPA_PUMP: &'static str = "set_spa_pump";
pub const IAQUALINK_COMMAND_SET_TEMPS: &'static str = "set_temps";

use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
  id: String,
  created_at: String,
  updated_at: String,
  email: String,
  username: String,
  first_name: Option<String>,
  last_name: Option<String>,
  address: Option<String>,
  address_1: Option<String>,
  address_2: Option<String>,
  postal_code: String,
  city: Option<String>,
  country: Option<String>,
  opt_in_1: String,
  opt_in_2: String,
  phone: String,
  role: String,
  state: String,
  time_zone: Option<String>,
  session_id: String,
  #[serde(rename = "cognitoPool")]
  cognito_pool: serde_json::Value,
  authentication_token: String,
  credentials: serde_json::Value,
  #[serde(rename = "userPoolOAuth")]
  user_pool_oauth: PoolOAuth,
  #[serde(flatten)]
  _unknown_fields: HashMap<String, serde_json::Value>,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PoolOAuth {
  access_token: String,
  expires_in: usize,
  token_type: String,
  refresh_token: String,
  id_token: String,
  #[serde(flatten)]
  _unknown_fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct System {
  id: usize,
  name: String,
  created_at: String,
  updated_at: String,
  device_type: String,
  firmware_version: Option<String>,
  last_activity_at: Option<String>,
  owner_id: usize,
  serial_number: String,
  target_firmware_version: Option<String>,
  update_firmware_start_at: Option<String>,
  updating: bool,
  #[serde(flatten)]
  _unknown_fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
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
#[serde(rename_all = "snake_case")]
pub struct RobotSensor {
  #[serde(rename = "type")]
  sensor_type: String,
  state: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RobotSensors {
  sns_1: RobotSensor,
  sns_2: RobotSensor,
  sns_3: RobotSensor,
}

#[derive(Debug, Serialize, Deserialize)]
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
  #[serde(flatten)]
  _unknown_fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Sensor {
  sensor_type: String,
  state: usize,
  value: usize,
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
#[serde(rename_all = "snake_case")]
pub struct Auxiliary {
  #[serde(rename = "type")]
  aux_type: String,
  color: Color,
  mode: usize,
  state: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterPump {
  #[serde(rename = "type")]
  pump_type: usize,
  state: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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
  #[serde(flatten)]
  _unknown_fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Equipment {
  Robot(Robot),
  #[serde(rename = "swc_0")]
  Swc0(SaltWaterChlorinator)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Timer {
  start: String,
  end: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Schedule {
  active: usize,
  enabled: usize,
  endpoint: String,
  id: String,
  name: String,
  timer: Timer,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Schedules {
  programmed: usize,
  supported: usize,
  sch1: Schedule,
  sch2: Schedule,
  sch3: Schedule,
  sch4: Schedule,
  sch9: Schedule,
  sch10: Schedule,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct State {
  reported: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Heating {
  enabled: usize,
  priority_enabled: usize,
  sp: usize,
  sp_max: usize,
  sp_min: usize,
  state: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportedState {
  dt: Option<String>,
  aws: serde_json::Value,
  ebox_data: Option<serde_json::Value>,
  equipment: Equipment,
  job_id: Option<String>,
  sn: Option<String>,
  vr: String,
  main: Option<serde_json::Value>,
  hmi: Option<serde_json::Value>,
  schedules: Option<Schedules>,
  state: Option<serde_json::Value>,
  heating: Option<Heating>,
  debug: Option<serde_json::Value>,
  #[serde(flatten)]
  _unknown_fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceState {
  reported: ReportedState,
  #[serde(flatten)]
  _unknown_fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceShadow {
  device_id: String,
  state: DeviceState,
  ts: usize,
  #[serde(flatten)]
  _unknown_fields: HashMap<String, serde_json::Value>,
}

impl Client {
  fn client() -> reqwest::Result<reqwest::Client> {
    reqwest::Client::builder()
      .user_agent("iAqualink/447 CFNetwork/1240.0.4 Darwin/20.5.0")
      .build()
  }

  pub async fn sign_in(&self) -> reqwest::Result<LoginResponse> {
    let http_client = Self::client()?;

    let mut credentials = HashMap::<&str, &str>::new();
    credentials.insert("email", &self.email);
    credentials.insert("password", &self.password);

    http_client.post(IAQUALINK_LOGIN_URL)
      .json(&credentials)
      .send()
      .await?
      .json()
      .await
  }

  pub async fn devices(&self, login_response: &LoginResponse) -> reqwest::Result<Vec<System>> {
    let http_client = Self::client()?;

    let mut credentials = HashMap::<&str, &str>::new();
    credentials.insert("user_id", &login_response.id);
    credentials.insert("api_key", IAQUALINK_API_KEY);
    credentials.insert("authentication_token", &login_response.authentication_token);

    let url = reqwest::Url::parse_with_params(IAQUALINK_DEVICES_URL, &credentials).unwrap();

    http_client.get(url)
      .send()
      .await?
      .json()
      .await
  }

  fn command_url(command: &str, serial: &str, login_response: &LoginResponse) -> reqwest::Url {
    let mut params = HashMap::<&str, &str>::new();
    params.insert("actionID", "command");
    params.insert("command", command);
    params.insert("serial", serial);
    params.insert("sessionID", &login_response.session_id);

    reqwest::Url::parse_with_params(IAQUALINK_SESSION_URL, &params).unwrap()
  }

  pub async fn home(&self, serial: &str, login_response: &LoginResponse) -> reqwest::Result<serde_json::Value> {
    let http_client = Self::client()?;

    http_client.get(Self::command_url(IAQUALINK_COMMAND_GET_HOME, serial, login_response))
    .send()
    .await?
    .json()
    .await
  }

  pub async fn device(&self, serial: &str, login_response: &LoginResponse) -> reqwest::Result<serde_json::Value> {
    let http_client = Self::client()?;

    http_client.get(Self::command_url(IAQUALINK_COMMAND_GET_DEVICES, serial, login_response))
    .send()
    .await?
    .json()
    .await
  }

  pub async fn site(&self, serial: &str, login_response: &LoginResponse) -> reqwest::Result<serde_json::Value> {
    let http_client = Self::client()?;

    let url = reqwest::Url::parse(IAQUALINK_DEVICES_V1_URL)
      .and_then(|url| url.join(&format!("{}/site", serial)))
      .unwrap();

    http_client.get(url)
      .bearer_auth(&login_response.user_pool_oauth.id_token)
      .send()
      .await?
      .json()
      .await
  }

  pub async fn info(&self, serial: &str, login_response: &LoginResponse) -> reqwest::Result<serde_json::Value> {
    let http_client = Self::client()?;

    let url = reqwest::Url::parse(IAQUALINK_DEVICES_V1_URL)
      .and_then(|url| url.join(&format!("{}/info", serial)))
      .unwrap();

    http_client.get(url)
    .bearer_auth(&login_response.user_pool_oauth.id_token)
      .send()
      .await?
      .json()
      .await
  }

  pub async fn features(&self, serial: &str, login_response: &LoginResponse) -> reqwest::Result<serde_json::Value> {
    let http_client = Self::client()?;

    let url = reqwest::Url::parse(IAQUALINK_DEVICES_V2_URL)
      .and_then(|url| url.join(&format!("{}/features", serial)))
      .unwrap();

    http_client.get(url)
      .bearer_auth(&login_response.user_pool_oauth.id_token)
      .send()
      .await?
      .json()
      .await
  }

  // See: https://community.home-assistant.io/t/jandy-iaqualink-pool-integration/105633/276
  pub async fn shadow(&self, serial: &str, login_response: &LoginResponse) -> reqwest::Result<DeviceShadow> {
    let http_client = Self::client()?;

    let url = reqwest::Url::parse(IAQUALINK_DEVICES_V1_URL)
      .and_then(|url| url.join(&format!("{}/shadow", serial)))
      .unwrap();

    http_client.get(url)
      .bearer_auth(&login_response.user_pool_oauth.id_token)
      .send()
      .await?
      .json()
      .await
  }

  pub async fn set_shadow<S: Serialize>(&self, serial: &str, login_response: &LoginResponse, value: S) -> reqwest::Result<serde_json::Value> {
    let http_client = Self::client()?;

    let url = reqwest::Url::parse(IAQUALINK_DEVICES_V1_URL)
      .and_then(|url| url.join(&format!("{}/shadow", serial)))
      .unwrap();

    http_client.post(url)
      .bearer_auth(&login_response.user_pool_oauth.id_token)
      .json(&value)
      .send()
      .await?
      .json()
      .await
  }
}

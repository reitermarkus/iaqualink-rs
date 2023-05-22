use std::collections::HashMap;

use reqwest::Method;
use serde::{Serialize, Deserialize};

mod exo;
pub use exo::*;

mod robot;
pub use robot::*;

pub const IAQUALINK_API_KEY: &'static str = "EOOEMOW4YR6QNB07";
pub const IAQUALINK_SESSION_URL: &'static str = "https://p-api.iaqualink.net/v1/mobile/session.json";
pub const IAQUALINK_LOGIN_URL: &'static str = "https://prod.zodiac-io.com/users/v1/login";
pub const IAQUALINK_DEVICES_URL: &'static str = "https://r-api.iaqualink.net/devices.json";
pub const IAQUALINK_DEVICES_V1_URL: &'static str = "https://prod.zodiac-io.com/devices/v1/";
pub const IAQUALINK_DEVICES_V2_URL: &'static str = "https://prod.zodiac-io.com/devices/v2/";

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

#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Credentials {
  access_key_id: String,
  expiration: DateTime<Utc>,
  identity_id: String,
  secret_key: String,
  session_token: String,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct CognitoPool {
  app_client_id: String,
  region: String,
  domain: String,
  pool_id: String,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
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
  cognito_pool: CognitoPool,
  authentication_token: String,
  credentials: Credentials,
  #[serde(rename = "userPoolOAuth")]
  user_pool_oauth: PoolOAuth,
}


#[derive(Debug, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct PoolOAuth {
  access_token: String,
  expires_in: usize,
  token_type: String,
  refresh_token: String,
  id_token: String,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
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
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct RobotEquipment {
  robot: Robot,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SaltWaterChlorinatorEquipment {
  swc_0: SaltWaterChlorinator,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Timer {
  start: String,
  end: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Schedule {
  active: usize,
  enabled: usize,
  endpoint: String,
  id: String,
  name: String,
  timer: Timer,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
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
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct State {
  reported: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Heating {
  enabled: usize,
  priority_enabled: usize,
  sp: usize,
  sp_max: usize,
  sp_min: usize,
  state: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AwsStateStatus {
  Connected,
  Disconnected,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct AwsState {
  session_id: String,
  status: AwsStateStatus,
  timestamp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct EboxData {
  complete_cleaner_pn: String,
  complete_cleaner_sn: String,
  control_box_pn: String,
  control_box_sn: String,
  motor_block_sn: String,
  power_supply_sn: String,
  sensor_block_sn: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ExoState {
  aws: AwsState,
  equipment: SaltWaterChlorinatorEquipment,
  vr: String,
  main: Option<serde_json::Value>,
  hmi: Option<serde_json::Value>,
  schedules: Option<Schedules>,
  state: Option<serde_json::Value>,
  heating: Option<Heating>,
  debug: Option<serde_json::Value>,
  debug_main: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct VrState {
  dt: String,
  aws: AwsState,
  ebox_data: EboxData,
  equipment: RobotEquipment,
  job_id: String,
  sn: String,
  vr: String
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[serde(untagged)]
pub enum ReportedState {
  #[serde(rename_all = "camelCase")]
  Exo(ExoState),
  #[serde(rename_all = "camelCase")]
  Vr(VrState),
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct DeviceState {
  reported: ReportedState,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct DeviceShadow {
  device_id: String,
  state: DeviceState,
  ts: usize,
}

impl System {
  fn request(&self, method: Method, path: &str, login_response: &LoginResponse) -> reqwest::RequestBuilder {
    reqwest::Client::new()
      .request(method, self.url(path))
      .bearer_auth(&login_response.user_pool_oauth.id_token)
  }

  fn url(&self, path: &str) -> reqwest::Url {
    reqwest::Url::parse(IAQUALINK_DEVICES_V1_URL)
      .and_then(|url| url.join(&format!("{}/{}", self.serial_number, path)))
      .unwrap()
  }

  pub async fn site(&self, login_response: &LoginResponse) -> reqwest::Result<serde_json::Value> {
    self.request(Method::GET, "site", login_response).send().await?.json().await
  }

  pub async fn info(&self, login_response: &LoginResponse) -> reqwest::Result<serde_json::Value> {
    self.request(Method::GET, "info", login_response).send().await?.json().await
  }

  pub async fn features(&self, login_response: &LoginResponse) -> reqwest::Result<serde_json::Value> {
    self.request(Method::GET, "features", login_response).send().await?.json().await
  }

  // See: https://community.home-assistant.io/t/jandy-iaqualink-pool-integration/105633/276
  pub async fn shadow(&self, login_response: &LoginResponse) -> reqwest::Result<DeviceShadow> {
    self.request(Method::GET, "shadow", login_response).send().await?.json().await
  }

  pub async fn set_shadow<S: Serialize>(&self, login_response: &LoginResponse, value: S) -> reqwest::Result<serde_json::Value> {
    self.request(Method::POST, "shadow", login_response).json(&value).send().await?.json().await
  }
}

impl Client {
  pub async fn sign_in(&self) -> reqwest::Result<LoginResponse> {
    let mut credentials = HashMap::<&str, &str>::new();
    credentials.insert("email", &self.email);
    credentials.insert("password", &self.password);

    reqwest::Client::new()
      .post(IAQUALINK_LOGIN_URL)
      .json(&credentials)
      .send()
      .await?
      .json()
      .await
  }

  pub async fn devices(&self, login_response: &LoginResponse) -> reqwest::Result<Vec<System>> {
    let mut credentials = HashMap::<&str, &str>::new();
    credentials.insert("user_id", &login_response.id);
    credentials.insert("api_key", IAQUALINK_API_KEY);
    credentials.insert("authentication_token", &login_response.authentication_token);

    let url = reqwest::Url::parse_with_params(IAQUALINK_DEVICES_URL, &credentials).unwrap();

    reqwest::Client::new()
      .get(url)
      .send()
      .await?
      .json()
      .await
  }
}

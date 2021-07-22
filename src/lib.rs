use std::collections::HashMap;

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

#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(debug, serde(deny_unknown_fields))]
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
}


#[derive(Debug, Deserialize)]
#[cfg_attr(debug, serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct PoolOAuth {
  access_token: String,
  expires_in: usize,
  token_type: String,
  refresh_token: String,
  id_token: String,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(debug, serde(deny_unknown_fields))]
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
#[cfg_attr(debug, serde(deny_unknown_fields))]
#[serde(rename_all = "snake_case")]
pub enum Equipment {
  Robot(Robot),
  #[serde(rename = "swc_0")]
  Swc0(SaltWaterChlorinator)
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug, serde(deny_unknown_fields))]
pub struct Timer {
  start: String,
  end: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug, serde(deny_unknown_fields))]
pub struct Schedule {
  active: usize,
  enabled: usize,
  endpoint: String,
  id: String,
  name: String,
  timer: Timer,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug, serde(deny_unknown_fields))]
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
#[cfg_attr(debug, serde(deny_unknown_fields))]
pub struct State {
  reported: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug, serde(deny_unknown_fields))]
pub struct Heating {
  enabled: usize,
  priority_enabled: usize,
  sp: usize,
  sp_max: usize,
  sp_min: usize,
  state: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug, serde(deny_unknown_fields))]
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
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug, serde(deny_unknown_fields))]
pub struct DeviceState {
  reported: ReportedState,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct DeviceShadow {
  device_id: String,
  state: DeviceState,
  ts: usize,
}

impl Client {
  fn client() -> reqwest::Result<reqwest::Client> {
    reqwest::Client::builder()
      // .user_agent("iAqualink/447 CFNetwork/1240.0.4 Darwin/20.5.0")
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

use std::collections::HashMap;
use std::str::FromStr;
use std::time::SystemTime;
use std::sync::Arc;

use aws_smithy_http::body::SdkBody;
use aws_sig_auth::signer::{OperationSigningConfig, RequestConfig, SigV4Signer};
use aws_credential_types::provider::ProvideCredentials;
use aws_types::region::{Region, SigningRegion};
use aws_types::SigningService;
use chrono::{DateTime, Utc};
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
pub const IAQUALINK_AWSIOT_ENDPOINT: &'static str = "a1zi08qpbrtjyq-ats.iot.us-east-1.amazonaws.com";

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
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
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

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MqttResponse {
  client_token: String,
  metadata: serde_json::Value,
  state: serde_json::Value,
  timestamp: u64,
  version: u32,
}

impl System {
  fn request(&self, method: Method, api_url: &str, path: &str, login_response: &LoginResponse) -> reqwest::RequestBuilder {
    let url = reqwest::Url::parse(api_url)
      .and_then(|url| url.join(&format!("{}/{}", self.serial_number, path)))
      .unwrap();

    println!("URL: {}", url);

    reqwest::Client::new()
      .request(method, url)
      .header("Authorization", format!("{} {}", login_response.user_pool_oauth.token_type, login_response.user_pool_oauth.id_token))
  }



  pub async fn site(&self, login_response: &LoginResponse) -> reqwest::Result<serde_json::Value> {
    self.request(Method::GET, IAQUALINK_DEVICES_V2_URL, "site", login_response).send().await?.json().await
  }

  pub async fn info(&self, login_response: &LoginResponse) -> reqwest::Result<serde_json::Value> {
    self.request(Method::GET, IAQUALINK_DEVICES_V2_URL, "info", login_response).send().await?.json().await
  }

  pub async fn features(&self, login_response: &LoginResponse) -> reqwest::Result<serde_json::Value> {
    self.request(Method::GET, IAQUALINK_DEVICES_V2_URL, "features", login_response).send().await?.json().await
  }

  // See: https://community.home-assistant.io/t/jandy-iaqualink-pool-integration/105633/276
  pub async fn shadow(&self, login_response: &LoginResponse) -> reqwest::Result<DeviceShadow> {
    self.request(Method::GET, IAQUALINK_DEVICES_V1_URL, "shadow", login_response).send().await?.json().await
  }

  pub async fn set_shadow<S: Serialize>(&self, login_response: &LoginResponse, value: S) -> reqwest::Result<serde_json::Value> {
    self.request(Method::POST, IAQUALINK_DEVICES_V1_URL, "shadow", login_response).json(&value).send().await?.json().await
  }

  pub async fn subscribe(&self, login_response: &LoginResponse) -> anyhow::Result<()> {
    let thing_name = self.serial_number.clone();
    let client_id = format!("{}_rust", login_response.cognito_pool.app_client_id);

    let valid_for = {
      let now = Utc::now();
      login_response.credentials.expiration.signed_duration_since(now)
    };

    let credentials = aws_credential_types::Credentials::new(
      login_response.credentials.access_key_id.clone(),
      login_response.credentials.secret_key.clone(),
      Some(login_response.credentials.session_token.clone()),
      SystemTime::now().checked_add(valid_for.to_std().unwrap()),
      "iaqualink",
    );


    let credential_provider = credentials;
    let region = Region::new(login_response.cognito_pool.region.clone());

    use rumqttc::{v5::{MqttOptions, AsyncClient, Event, mqttbytes::{QoS, v5::{ConnectProperties, Packet}}}, Transport};

    let mut mqtt_options = MqttOptions::new(client_id, format!("wss://{}/mqtt", IAQUALINK_AWSIOT_ENDPOINT), 443);
    mqtt_options.set_transport(Transport::wss_with_default_config());
    let mut connect_properties = ConnectProperties::new();
    connect_properties.max_packet_size = Some(16 * 1024);
    mqtt_options.set_connect_properties(connect_properties);


    mqtt_options.set_request_modifier(move |request| {
      let credential_provider = credential_provider.clone();
      let region = region.clone();

      async move {
          let request_config = RequestConfig {
              request_ts: SystemTime::now(),
              region: &SigningRegion::from(region.clone()),
              service: &SigningService::from_static("iotdata"),
              payload_override: None,
          };

        let (parts, body) = request.into_parts();
        let mut request: http::Request<SdkBody> = http::Request::from_parts(parts, SdkBody::empty());

        let signer = SigV4Signer::new();
        signer.sign(
          &OperationSigningConfig::default_config(),
          &request_config,
          &credential_provider.provide_credentials().await.unwrap(),
          &mut request,
        ).unwrap();

        let (parts, _) = request.into_parts();
        http::Request::from_parts(parts, body)
      }
    });

    // mqtt_options.aws_credential_provider = Some(Arc::new(Box::new(credential_provider)));

    let (mut client, mut eventloop) = AsyncClient::new(mqtt_options, 10);

    let topic_prefix = format!("$aws/things/{thing_name}/shadow");

    client.subscribe(format!("{topic_prefix}/get/accepted"), QoS::AtLeastOnce).await?;
    client.subscribe(format!("{topic_prefix}/get/rejected"), QoS::AtLeastOnce).await?;
    client.subscribe(format!("{topic_prefix}/update/delta"), QoS::AtLeastOnce).await?;
    client.subscribe(format!("{topic_prefix}/update/accepted"), QoS::AtLeastOnce).await?;
    client.subscribe(format!("{topic_prefix}/update/documents"), QoS::AtLeastOnce).await?;
    client.subscribe(format!("{topic_prefix}/update/rejected"), QoS::AtLeastOnce).await?;

    loop {
      if let Event::Incoming(notification) = eventloop.poll().await.unwrap() {

        match notification {
          Packet::Publish(state) => {
            let topic = std::str::from_utf8(&state.topic).unwrap();
            let state: MqttResponse = serde_json::from_slice(&state.payload).unwrap();
            println!("Received {topic} = {:#?}", state);
          }
          packet => {
            dbg!(packet);
          },
        }
      }
    }

    Ok(())
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

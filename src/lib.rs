use std::{
  collections::HashMap,
  str::{self, FromStr},
  sync::Arc,
  time::SystemTime,
};

use aws_credential_types::provider::ProvideCredentials;
use aws_sig_auth::signer::{OperationSigningConfig, RequestConfig, SigV4Signer};
use aws_smithy_http::body::SdkBody;
use aws_types::{
  region::{Region, SigningRegion},
  SigningService,
};
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};

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
  id: u32,
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
  expires_in: u16,
  token_type: String,
  refresh_token: String,
  id_token: String,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct System {
  id: u32,
  name: String,
  created_at: String,
  updated_at: String,
  device_type: String,
  firmware_version: Option<String>,
  last_activity_at: Option<String>,
  owner_id: u32,
  serial_number: String,
  target_firmware_version: Option<String>,
  update_firmware_start_at: Option<String>,
  updating: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct State {
  reported: serde_json::Value,
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
  timestamp: u64,
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
#[serde(rename_all = "camelCase")]
pub struct DeviceState {
  reported: ReportedState,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct DeviceShadow {
  device_id: String,
  state: DeviceState,
  ts: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub enum MqttMetadataTimestamp {
  Timestamp(u32),
  #[serde(untagged)]
  Nested(Box<HashMap<String, MqttMetadataTimestamp>>),
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct MqttMetadata {
  desired: Option<MqttMetadataTimestamp>,
  reported: Option<MqttMetadataTimestamp>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct MqttState {
  desired: Option<serde_json::Value>,
  reported: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct MqttDocumentsState {
  state: MqttState,
  metadata: MqttMetadata,
  version: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[serde(untagged)]
pub enum MqttResponse {
  #[serde(rename_all = "camelCase")]
  UpdateAccepted {
    state: MqttState,
    metadata: MqttMetadata,
    timestamp: u64,
    client_token: Option<String>,
    version: u32,
  },
  #[serde(rename_all = "camelCase")]
  UpdateDelta {
    state: Option<MqttState>,
    metadata: Option<MqttMetadata>,
    timestamp: u64,
    client_token: Option<String>,
    version: u32,
  },
  #[serde(rename_all = "camelCase")]
  UpdateDocuments {
    state: Option<MqttState>,
    metadata: Option<MqttMetadata>,
    previous: Option<MqttDocumentsState>,
    current: Option<MqttDocumentsState>,
    timestamp: u64,
    client_token: Option<String>,
  },
  #[serde(rename_all = "camelCase")]
  Error { client_token: Option<String>, code: u16, message: String },
}

impl System {
  fn request(
    &self,
    method: Method,
    api_url: &str,
    path: &str,
    login_response: &LoginResponse,
  ) -> reqwest::RequestBuilder {
    let url =
      reqwest::Url::parse(api_url).and_then(|url| url.join(&format!("{}/{}", self.serial_number, path))).unwrap();

    println!("URL: {}", url);

    reqwest::Client::new().request(method, url).header(
      "Authorization",
      format!("{} {}", login_response.user_pool_oauth.token_type, login_response.user_pool_oauth.id_token),
    )
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

  pub async fn set_shadow<S: Serialize>(
    &self,
    login_response: &LoginResponse,
    value: S,
  ) -> reqwest::Result<serde_json::Value> {
    self
      .request(Method::POST, IAQUALINK_DEVICES_V1_URL, "shadow", login_response)
      .json(&value)
      .send()
      .await?
      .json()
      .await
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

    use rumqttc::{
      v5::{
        mqttbytes::{
          v5::{ConnectProperties, Packet},
          QoS,
        },
        AsyncClient, Event, MqttOptions,
      },
      Transport,
    };

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
        signer
          .sign(
            &OperationSigningConfig::default_config(),
            &request_config,
            &credential_provider.provide_credentials().await.unwrap(),
            &mut request,
          )
          .unwrap();

        let (parts, _) = request.into_parts();
        http::Request::from_parts(parts, body)
      }
    });

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
            let topic = str::from_utf8(&state.topic).unwrap();

            match serde_json::from_slice::<MqttResponse>(&state.payload) {
              Ok(state) => println!("Received {topic} = {:#?}", state),
              Err(err) => {
                panic!("Received invalid payload for {topic}: {}\n{}", err, str::from_utf8(&state.payload).unwrap())
              },
            }
          },
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

    let res = reqwest::Client::new().post(IAQUALINK_LOGIN_URL).json(&credentials).send().await;
    res?.json().await
  }

  pub async fn devices(&self, login_response: &LoginResponse) -> reqwest::Result<Vec<System>> {
    let mut credentials = HashMap::<&str, &str>::new();
    let id = login_response.id.to_string();
    credentials.insert("user_id", &id);
    credentials.insert("api_key", IAQUALINK_API_KEY);
    credentials.insert("authentication_token", &login_response.authentication_token);

    let url = reqwest::Url::parse_with_params(IAQUALINK_DEVICES_URL, &credentials).unwrap();

    reqwest::Client::new().get(url).send().await?.json().await
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn mqtt_response() {
    let response = r#"
    {
      "previous": {
        "state": {
          "desired": {
            "equipment": {
              "swc_0": {
                "aux_1": {}
              }
            },
            "schedules": {
              "sch1": {
                "enabled": 1,
                "timer": {
                  "start": "08:00",
                  "end": "10:00"
                }
              },
              "sch2": {
                "enabled": 1,
                "timer": {
                  "start": "14:00",
                  "end": "20:00"
                }
              },
              "sch3": {
                "enabled": 1,
                "timer": {
                  "start": "08:00",
                  "end": "10:00"
                }
              },
              "sch4": {
                "enabled": 1,
                "timer": {
                  "start": "14:00",
                  "end": "20:00"
                }
              }
            }
          },
          "reported": {
            "aws": {
              "status": "connected",
              "timestamp": 1686468192272,
              "session_id": "82fc61c6-23ce-4dea-ba59-2c6d79b1f7e3"
            },
            "debug": {
              "Version Firmware": "V85W4B0",
              "OTA success": 4,
              "OTA fail": 0,
              "OTA fail by disconnection": 0,
              "OTA fail global": 0,
              "MQTT connection": 1,
              "MQTT disconnection total": 0,
              "OTA State": 0,
              "Nb reboot du to MQTT issue": 2065,
              "Last error": 65278,
              "RSSI": -94,
              "Nb_Success_Pub_MSG": 0,
              "Nb_Success_Sub_Receive": 0,
              "Nb_Fail_Publish_MSG": 0,
              "Still alive": 1
            },
            "vr": "V85W4",
            "equipment": {
              "swc_0": {
                "sns_2": {
                  "sensor_type": "Orp",
                  "state": 1,
                  "value": 790
                },
                "temp": 1,
                "vsp": 1,
                "exo_state": 1,
                "sns_1": {
                  "sensor_type": "Ph",
                  "state": 1,
                  "value": 57
                },
                "aux_1": {
                  "type": "light",
                  "color": 0,
                  "state": 1,
                  "mode": 1
                },
                "lang": 4,
                "amp": 1,
                "aux230": 1,
                "sn": "ALWA01011336200223",
                "production": 0,
                "aux_2": {
                  "type": "heat",
                  "color": 0,
                  "state": 0,
                  "mode": 3
                },
                "swc": 50,
                "swc_low": 10,
                "orp_sp": 740,
                "version": "V1",
                "sns_3": {
                  "sensor_type": "Water temp",
                  "state": 1,
                  "value": 26
                },
                "vr": "V85R70",
                "ph_only": 1,
                "dual_link": 1,
                "ph_sp": 72,
                "low": 0,
                "boost": 0,
                "boost_time": "24:00",
                "error_state": 0,
                "filter_pump": {
                  "type": 1,
                  "state": 0
                },
                "error_code": 0,
                "vsp_speed": {
                  "min": 600,
                  "max": 3080
                }
              }
            },
            "state": {
              "reported": {
                "debug_main": {
                  "tr": 100
                }
              }
            },
            "schedules": {
              "sch2": {
                "timer": {
                  "start": "14:00",
                  "end": "20:00"
                },
                "enabled": 1,
                "active": 0,
                "id": "sch_2",
                "name": "Salt Water Chlorinator 2",
                "endpoint": "swc_2"
              },
              "sch10": {
                "id": "sch_10",
                "name": "Aux 2",
                "endpoint": "aux2",
                "enabled": 0,
                "active": 1,
                "timer": {
                  "start": "00:00",
                  "end": "00:00"
                }
              },
              "sch1": {
                "timer": {
                  "start": "08:00",
                  "end": "10:00"
                },
                "id": "sch_1",
                "name": "Salt Water Chlorinator 1",
                "endpoint": "swc_1",
                "enabled": 1,
                "active": 1
              },
              "sch9": {
                "timer": {
                  "start": "19:00",
                  "end": "23:59"
                },
                "enabled": 1,
                "active": 0,
                "id": "sch_9",
                "name": "Aux 1",
                "endpoint": "aux1"
              },
              "supported": 6,
              "programmed": 5,
              "sch3": {
                "enabled": 1,
                "active": 0,
                "timer": {
                  "start": "08:00",
                  "end": "10:00"
                },
                "id": "sch_3",
                "name": "Filter Pump 1",
                "endpoint": "ssp_1"
              },
              "sch4": {
                "enabled": 1,
                "active": 0,
                "id": "sch_4",
                "name": "Filter Pump 2",
                "endpoint": "ssp_2",
                "timer": {
                  "start": "14:00",
                  "end": "20:00"
                }
              }
            },
            "hmi": {
              "ff": {
                "fn": "/fluidra-ota-prod/exo/V85W4_OTA.bin",
                "vr": "V85W4",
                "ts": 1652278875,
                "pg": {
                  "fs": 507300,
                  "bd": 507300,
                  "ts": 1652278522390,
                  "te": 1652278875697
                }
              },
              "fw": {
                "fn": "/fluidra-ota-prod/exo/V85W4_OTA.bin",
                "vr": "V85W4"
              }
            },
            "main": {
              "ff": {
                "fn": "/fluidra-ota-prod/exo/V85R70_OTA.bin",
                "vr": "V85R70",
                "ts": 1652279182,
                "pg": {
                  "fs": 402464,
                  "bd": 402464,
                  "ts": 1652278917214,
                  "te": 1652279182779
                }
              }
            },
            "debug_main": {
              "tr": 100
            },
            "heating": {
              "enabled": 1,
              "state": 2,
              "priority_enabled": 0,
              "sp": 28,
              "sp_min": 15,
              "sp_max": 32
            }
          }
        },
        "metadata": {
          "desired": {
            "equipment": {
              "swc_0": {
                "aux_1": {}
              }
            },
            "schedules": {
              "sch1": {
                "enabled": {
                  "timestamp": 1686511299
                },
                "timer": {
                  "start": {
                    "timestamp": 1686511299
                  },
                  "end": {
                    "timestamp": 1686511299
                  }
                }
              },
              "sch2": {
                "enabled": {
                  "timestamp": 1686511299
                },
                "timer": {
                  "start": {
                    "timestamp": 1686511299
                  },
                  "end": {
                    "timestamp": 1686511299
                  }
                }
              },
              "sch3": {
                "enabled": {
                  "timestamp": 1686511299
                },
                "timer": {
                  "start": {
                    "timestamp": 1686511299
                  },
                  "end": {
                    "timestamp": 1686511299
                  }
                }
              },
              "sch4": {
                "enabled": {
                  "timestamp": 1686511299
                },
                "timer": {
                  "start": {
                    "timestamp": 1686511299
                  },
                  "end": {
                    "timestamp": 1686511299
                  }
                }
              }
            }
          },
          "reported": {
            "aws": {
              "status": {
                "timestamp": 1686468192
              },
              "timestamp": {
                "timestamp": 1686468192
              },
              "session_id": {
                "timestamp": 1686468192
              }
            },
            "debug": {
              "Version Firmware": {
                "timestamp": 1686451882
              },
              "OTA success": {
                "timestamp": 1686451882
              },
              "OTA fail": {
                "timestamp": 1686451882
              },
              "OTA fail by disconnection": {
                "timestamp": 1686451882
              },
              "OTA fail global": {
                "timestamp": 1686451882
              },
              "MQTT connection": {
                "timestamp": 1686451882
              },
              "MQTT disconnection total": {
                "timestamp": 1686451882
              },
              "OTA State": {
                "timestamp": 1686451882
              },
              "Nb reboot du to MQTT issue": {
                "timestamp": 1686451882
              },
              "Last error": {
                "timestamp": 1686451882
              },
              "RSSI": {
                "timestamp": 1686451882
              },
              "Nb_Success_Pub_MSG": {
                "timestamp": 1686451882
              },
              "Nb_Success_Sub_Receive": {
                "timestamp": 1686451882
              },
              "Nb_Fail_Publish_MSG": {
                "timestamp": 1686451882
              },
              "Still alive": {
                "timestamp": 1686451882
              }
            },
            "vr": {
              "timestamp": 1686468192
            },
            "equipment": {
              "swc_0": {
                "sns_2": {
                  "sensor_type": {
                    "timestamp": 1686508208
                  },
                  "state": {
                    "timestamp": 1686508208
                  },
                  "value": {
                    "timestamp": 1686508208
                  }
                },
                "temp": {
                  "timestamp": 1683352378
                },
                "vsp": {
                  "timestamp": 1683352378
                },
                "exo_state": {
                  "timestamp": 1683352435
                },
                "sns_1": {
                  "sensor_type": {
                    "timestamp": 1686511250
                  },
                  "state": {
                    "timestamp": 1686511250
                  },
                  "value": {
                    "timestamp": 1686511250
                  }
                },
                "aux_1": {
                  "type": {
                    "timestamp": 1683352368
                  },
                  "color": {
                    "timestamp": 1683352381
                  },
                  "state": {
                    "timestamp": 1686511305
                  },
                  "mode": {
                    "timestamp": 1683352412
                  }
                },
                "lang": {
                  "timestamp": 1683352388
                },
                "amp": {
                  "timestamp": 1683352388
                },
                "aux230": {
                  "timestamp": 1683352388
                },
                "sn": {
                  "timestamp": 1683352388
                },
                "production": {
                  "timestamp": 1686506435
                },
                "aux_2": {
                  "type": {
                    "timestamp": 1683352369
                  },
                  "color": {
                    "timestamp": 1683352408
                  },
                  "state": {
                    "timestamp": 1683352390
                  },
                  "mode": {
                    "timestamp": 1683352403
                  }
                },
                "swc": {
                  "timestamp": 1683352404
                },
                "swc_low": {
                  "timestamp": 1683352433
                },
                "orp_sp": {
                  "timestamp": 1683352371
                },
                "version": {
                  "timestamp": 1683352453
                },
                "sns_3": {
                  "sensor_type": {
                    "timestamp": 1686509960
                  },
                  "state": {
                    "timestamp": 1686509960
                  },
                  "value": {
                    "timestamp": 1686509960
                  }
                },
                "vr": {
                  "timestamp": 1683352395
                },
                "ph_only": {
                  "timestamp": 1683352498
                },
                "dual_link": {
                  "timestamp": 1683352498
                },
                "ph_sp": {
                  "timestamp": 1683352383
                },
                "low": {
                  "timestamp": 1683352407
                },
                "boost": {
                  "timestamp": 1683352389
                },
                "boost_time": {
                  "timestamp": 1683352380
                },
                "error_state": {
                  "timestamp": 1683352415
                },
                "filter_pump": {
                  "type": {
                    "timestamp": 1683366231
                  },
                  "state": {
                    "timestamp": 1686506429
                  }
                },
                "error_code": {
                  "timestamp": 1683352419
                },
                "vsp_speed": {
                  "min": {
                    "timestamp": 1683366057
                  },
                  "max": {
                    "timestamp": 1683366058
                  }
                }
              }
            },
            "state": {
              "reported": {
                "debug_main": {
                  "tr": {
                    "timestamp": 1626814321
                  }
                }
              }
            },
            "schedules": {
              "sch2": {
                "timer": {
                  "start": {
                    "timestamp": 1684799527
                  },
                  "end": {
                    "timestamp": 1684799527
                  }
                },
                "enabled": {
                  "timestamp": 1686506436
                },
                "active": {
                  "timestamp": 1686506436
                },
                "id": {
                  "timestamp": 1683352393
                },
                "name": {
                  "timestamp": 1683352393
                },
                "endpoint": {
                  "timestamp": 1683352393
                }
              },
              "sch10": {
                "id": {
                  "timestamp": 1683352475
                },
                "name": {
                  "timestamp": 1683352475
                },
                "endpoint": {
                  "timestamp": 1683352475
                },
                "enabled": {
                  "timestamp": 1686511304
                },
                "active": {
                  "timestamp": 1686511304
                },
                "timer": {
                  "start": {
                    "timestamp": 1683352377
                  },
                  "end": {
                    "timestamp": 1683352377
                  }
                }
              },
              "sch1": {
                "timer": {
                  "start": {
                    "timestamp": 1684799421
                  },
                  "end": {
                    "timestamp": 1684799421
                  }
                },
                "id": {
                  "timestamp": 1683352362
                },
                "name": {
                  "timestamp": 1683352362
                },
                "endpoint": {
                  "timestamp": 1683352362
                },
                "enabled": {
                  "timestamp": 1686463233
                },
                "active": {
                  "timestamp": 1686463233
                }
              },
              "sch9": {
                "timer": {
                  "start": {
                    "timestamp": 1683352414
                  },
                  "end": {
                    "timestamp": 1683352414
                  }
                },
                "enabled": {
                  "timestamp": 1686511299
                },
                "active": {
                  "timestamp": 1686511299
                },
                "id": {
                  "timestamp": 1683352394
                },
                "name": {
                  "timestamp": 1683352394
                },
                "endpoint": {
                  "timestamp": 1683352394
                }
              },
              "supported": {
                "timestamp": 1683366233
              },
              "programmed": {
                "timestamp": 1683366233
              },
              "sch3": {
                "enabled": {
                  "timestamp": 1686470435
                },
                "active": {
                  "timestamp": 1686470435
                },
                "timer": {
                  "start": {
                    "timestamp": 1684799423
                  },
                  "end": {
                    "timestamp": 1684799423
                  }
                },
                "id": {
                  "timestamp": 1683366244
                },
                "name": {
                  "timestamp": 1683366244
                },
                "endpoint": {
                  "timestamp": 1683366244
                }
              },
              "sch4": {
                "enabled": {
                  "timestamp": 1686506430
                },
                "active": {
                  "timestamp": 1686506430
                },
                "id": {
                  "timestamp": 1683366242
                },
                "name": {
                  "timestamp": 1683366242
                },
                "endpoint": {
                  "timestamp": 1683366242
                },
                "timer": {
                  "start": {
                    "timestamp": 1684799528
                  },
                  "end": {
                    "timestamp": 1684799528
                  }
                }
              }
            },
            "hmi": {
              "ff": {
                "fn": {
                  "timestamp": 1652278522
                },
                "vr": {
                  "timestamp": 1652278522
                },
                "ts": {
                  "timestamp": 1652278875
                },
                "pg": {
                  "fs": {
                    "timestamp": 1652278522
                  },
                  "bd": {
                    "timestamp": 1652278875
                  },
                  "ts": {
                    "timestamp": 1652278522
                  },
                  "te": {
                    "timestamp": 1652278875
                  }
                }
              },
              "fw": {
                "fn": {
                  "timestamp": 1652278888
                },
                "vr": {
                  "timestamp": 1652278888
                }
              }
            },
            "main": {
              "ff": {
                "fn": {
                  "timestamp": 1652278916
                },
                "vr": {
                  "timestamp": 1652278916
                },
                "ts": {
                  "timestamp": 1652279183
                },
                "pg": {
                  "fs": {
                    "timestamp": 1652278917
                  },
                  "bd": {
                    "timestamp": 1652279182
                  },
                  "ts": {
                    "timestamp": 1652278917
                  },
                  "te": {
                    "timestamp": 1652279182
                  }
                }
              }
            },
            "debug_main": {
              "tr": {
                "timestamp": 1652279423
              }
            },
            "heating": {
              "enabled": {
                "timestamp": 1683352382
              },
              "state": {
                "timestamp": 1686506432
              },
              "priority_enabled": {
                "timestamp": 1683352398
              },
              "sp": {
                "timestamp": 1683352416
              },
              "sp_min": {
                "timestamp": 1683352416
              },
              "sp_max": {
                "timestamp": 1683352416
              }
            }
          }
        },
        "version": 2070905
      },
      "current": {
        "state": {
          "desired": {
            "equipment": {
              "swc_0": {
                "aux_1": {}
              }
            },
            "schedules": {
              "sch1": {
                "enabled": 1,
                "timer": {
                  "start": "08:00",
                  "end": "10:00"
                }
              },
              "sch2": {
                "enabled": 1,
                "timer": {
                  "start": "14:00",
                  "end": "20:00"
                }
              },
              "sch3": {
                "enabled": 1,
                "timer": {
                  "start": "08:00",
                  "end": "10:00"
                }
              },
              "sch4": {
                "enabled": 1,
                "timer": {
                  "start": "14:00",
                  "end": "20:00"
                }
              }
            }
          },
          "reported": {
            "aws": {
              "status": "connected",
              "timestamp": 1686468192272,
              "session_id": "82fc61c6-23ce-4dea-ba59-2c6d79b1f7e3"
            },
            "debug": {
              "Version Firmware": "V85W4B0",
              "OTA success": 4,
              "OTA fail": 0,
              "OTA fail by disconnection": 0,
              "OTA fail global": 0,
              "MQTT connection": 1,
              "MQTT disconnection total": 0,
              "OTA State": 0,
              "Nb reboot du to MQTT issue": 2065,
              "Last error": 65278,
              "RSSI": -94,
              "Nb_Success_Pub_MSG": 0,
              "Nb_Success_Sub_Receive": 0,
              "Nb_Fail_Publish_MSG": 0,
              "Still alive": 1
            },
            "vr": "V85W4",
            "equipment": {
              "swc_0": {
                "sns_2": {
                  "sensor_type": "Orp",
                  "state": 1,
                  "value": 790
                },
                "temp": 1,
                "vsp": 1,
                "exo_state": 1,
                "sns_1": {
                  "sensor_type": "Ph",
                  "state": 1,
                  "value": 57
                },
                "aux_1": {
                  "type": "light",
                  "color": 0,
                  "state": 1,
                  "mode": 1
                },
                "lang": 4,
                "amp": 1,
                "aux230": 1,
                "sn": "ALWA01011336200223",
                "production": 0,
                "aux_2": {
                  "type": "heat",
                  "color": 0,
                  "state": 0,
                  "mode": 3
                },
                "swc": 50,
                "swc_low": 10,
                "orp_sp": 740,
                "version": "V1",
                "sns_3": {
                  "sensor_type": "Water temp",
                  "state": 1,
                  "value": 26
                },
                "vr": "V85R70",
                "ph_only": 1,
                "dual_link": 1,
                "ph_sp": 72,
                "low": 0,
                "boost": 0,
                "boost_time": "24:00",
                "error_state": 0,
                "filter_pump": {
                  "type": 1,
                  "state": 0
                },
                "error_code": 0,
                "vsp_speed": {
                  "min": 600,
                  "max": 3080
                }
              }
            },
            "state": {
              "reported": {
                "debug_main": {
                  "tr": 100
                }
              }
            },
            "schedules": {
              "sch2": {
                "timer": {
                  "start": "14:00",
                  "end": "20:00"
                },
                "enabled": 1,
                "active": 0,
                "id": "sch_2",
                "name": "Salt Water Chlorinator 2",
                "endpoint": "swc_2"
              },
              "sch10": {
                "id": "sch_10",
                "name": "Aux 2",
                "endpoint": "aux2",
                "enabled": 0,
                "active": 1,
                "timer": {
                  "start": "00:00",
                  "end": "00:00"
                }
              },
              "sch1": {
                "timer": {
                  "start": "08:00",
                  "end": "10:00"
                },
                "id": "sch_1",
                "name": "Salt Water Chlorinator 1",
                "endpoint": "swc_1",
                "enabled": 1,
                "active": 1
              },
              "sch9": {
                "timer": {
                  "start": "19:00",
                  "end": "23:59"
                },
                "enabled": 1,
                "active": 1,
                "id": "sch_9",
                "name": "Aux 1",
                "endpoint": "aux1"
              },
              "supported": 6,
              "programmed": 5,
              "sch3": {
                "enabled": 1,
                "active": 0,
                "timer": {
                  "start": "08:00",
                  "end": "10:00"
                },
                "id": "sch_3",
                "name": "Filter Pump 1",
                "endpoint": "ssp_1"
              },
              "sch4": {
                "enabled": 1,
                "active": 0,
                "id": "sch_4",
                "name": "Filter Pump 2",
                "endpoint": "ssp_2",
                "timer": {
                  "start": "14:00",
                  "end": "20:00"
                }
              }
            },
            "hmi": {
              "ff": {
                "fn": "/fluidra-ota-prod/exo/V85W4_OTA.bin",
                "vr": "V85W4",
                "ts": 1652278875,
                "pg": {
                  "fs": 507300,
                  "bd": 507300,
                  "ts": 1652278522390,
                  "te": 1652278875697
                }
              },
              "fw": {
                "fn": "/fluidra-ota-prod/exo/V85W4_OTA.bin",
                "vr": "V85W4"
              }
            },
            "main": {
              "ff": {
                "fn": "/fluidra-ota-prod/exo/V85R70_OTA.bin",
                "vr": "V85R70",
                "ts": 1652279182,
                "pg": {
                  "fs": 402464,
                  "bd": 402464,
                  "ts": 1652278917214,
                  "te": 1652279182779
                }
              }
            },
            "debug_main": {
              "tr": 100
            },
            "heating": {
              "enabled": 1,
              "state": 2,
              "priority_enabled": 0,
              "sp": 28,
              "sp_min": 15,
              "sp_max": 32
            }
          }
        },
        "metadata": {
          "desired": {
            "equipment": {
              "swc_0": {
                "aux_1": {}
              }
            },
            "schedules": {
              "sch1": {
                "enabled": {
                  "timestamp": 1686511299
                },
                "timer": {
                  "start": {
                    "timestamp": 1686511299
                  },
                  "end": {
                    "timestamp": 1686511299
                  }
                }
              },
              "sch2": {
                "enabled": {
                  "timestamp": 1686511299
                },
                "timer": {
                  "start": {
                    "timestamp": 1686511299
                  },
                  "end": {
                    "timestamp": 1686511299
                  }
                }
              },
              "sch3": {
                "enabled": {
                  "timestamp": 1686511299
                },
                "timer": {
                  "start": {
                    "timestamp": 1686511299
                  },
                  "end": {
                    "timestamp": 1686511299
                  }
                }
              },
              "sch4": {
                "enabled": {
                  "timestamp": 1686511299
                },
                "timer": {
                  "start": {
                    "timestamp": 1686511299
                  },
                  "end": {
                    "timestamp": 1686511299
                  }
                }
              }
            }
          },
          "reported": {
            "aws": {
              "status": {
                "timestamp": 1686468192
              },
              "timestamp": {
                "timestamp": 1686468192
              },
              "session_id": {
                "timestamp": 1686468192
              }
            },
            "debug": {
              "Version Firmware": {
                "timestamp": 1686451882
              },
              "OTA success": {
                "timestamp": 1686451882
              },
              "OTA fail": {
                "timestamp": 1686451882
              },
              "OTA fail by disconnection": {
                "timestamp": 1686451882
              },
              "OTA fail global": {
                "timestamp": 1686451882
              },
              "MQTT connection": {
                "timestamp": 1686451882
              },
              "MQTT disconnection total": {
                "timestamp": 1686451882
              },
              "OTA State": {
                "timestamp": 1686451882
              },
              "Nb reboot du to MQTT issue": {
                "timestamp": 1686451882
              },
              "Last error": {
                "timestamp": 1686451882
              },
              "RSSI": {
                "timestamp": 1686451882
              },
              "Nb_Success_Pub_MSG": {
                "timestamp": 1686451882
              },
              "Nb_Success_Sub_Receive": {
                "timestamp": 1686451882
              },
              "Nb_Fail_Publish_MSG": {
                "timestamp": 1686451882
              },
              "Still alive": {
                "timestamp": 1686451882
              }
            },
            "vr": {
              "timestamp": 1686468192
            },
            "equipment": {
              "swc_0": {
                "sns_2": {
                  "sensor_type": {
                    "timestamp": 1686508208
                  },
                  "state": {
                    "timestamp": 1686508208
                  },
                  "value": {
                    "timestamp": 1686508208
                  }
                },
                "temp": {
                  "timestamp": 1683352378
                },
                "vsp": {
                  "timestamp": 1683352378
                },
                "exo_state": {
                  "timestamp": 1683352435
                },
                "sns_1": {
                  "sensor_type": {
                    "timestamp": 1686511250
                  },
                  "state": {
                    "timestamp": 1686511250
                  },
                  "value": {
                    "timestamp": 1686511250
                  }
                },
                "aux_1": {
                  "type": {
                    "timestamp": 1683352368
                  },
                  "color": {
                    "timestamp": 1683352381
                  },
                  "state": {
                    "timestamp": 1686511305
                  },
                  "mode": {
                    "timestamp": 1683352412
                  }
                },
                "lang": {
                  "timestamp": 1683352388
                },
                "amp": {
                  "timestamp": 1683352388
                },
                "aux230": {
                  "timestamp": 1683352388
                },
                "sn": {
                  "timestamp": 1683352388
                },
                "production": {
                  "timestamp": 1686506435
                },
                "aux_2": {
                  "type": {
                    "timestamp": 1683352369
                  },
                  "color": {
                    "timestamp": 1683352408
                  },
                  "state": {
                    "timestamp": 1683352390
                  },
                  "mode": {
                    "timestamp": 1683352403
                  }
                },
                "swc": {
                  "timestamp": 1683352404
                },
                "swc_low": {
                  "timestamp": 1683352433
                },
                "orp_sp": {
                  "timestamp": 1683352371
                },
                "version": {
                  "timestamp": 1683352453
                },
                "sns_3": {
                  "sensor_type": {
                    "timestamp": 1686509960
                  },
                  "state": {
                    "timestamp": 1686509960
                  },
                  "value": {
                    "timestamp": 1686509960
                  }
                },
                "vr": {
                  "timestamp": 1683352395
                },
                "ph_only": {
                  "timestamp": 1683352498
                },
                "dual_link": {
                  "timestamp": 1683352498
                },
                "ph_sp": {
                  "timestamp": 1683352383
                },
                "low": {
                  "timestamp": 1683352407
                },
                "boost": {
                  "timestamp": 1683352389
                },
                "boost_time": {
                  "timestamp": 1683352380
                },
                "error_state": {
                  "timestamp": 1683352415
                },
                "filter_pump": {
                  "type": {
                    "timestamp": 1683366231
                  },
                  "state": {
                    "timestamp": 1686506429
                  }
                },
                "error_code": {
                  "timestamp": 1683352419
                },
                "vsp_speed": {
                  "min": {
                    "timestamp": 1683366057
                  },
                  "max": {
                    "timestamp": 1683366058
                  }
                }
              }
            },
            "state": {
              "reported": {
                "debug_main": {
                  "tr": {
                    "timestamp": 1626814321
                  }
                }
              }
            },
            "schedules": {
              "sch2": {
                "timer": {
                  "start": {
                    "timestamp": 1684799527
                  },
                  "end": {
                    "timestamp": 1684799527
                  }
                },
                "enabled": {
                  "timestamp": 1686506436
                },
                "active": {
                  "timestamp": 1686506436
                },
                "id": {
                  "timestamp": 1683352393
                },
                "name": {
                  "timestamp": 1683352393
                },
                "endpoint": {
                  "timestamp": 1683352393
                }
              },
              "sch10": {
                "id": {
                  "timestamp": 1683352475
                },
                "name": {
                  "timestamp": 1683352475
                },
                "endpoint": {
                  "timestamp": 1683352475
                },
                "enabled": {
                  "timestamp": 1686511304
                },
                "active": {
                  "timestamp": 1686511304
                },
                "timer": {
                  "start": {
                    "timestamp": 1683352377
                  },
                  "end": {
                    "timestamp": 1683352377
                  }
                }
              },
              "sch1": {
                "timer": {
                  "start": {
                    "timestamp": 1684799421
                  },
                  "end": {
                    "timestamp": 1684799421
                  }
                },
                "id": {
                  "timestamp": 1683352362
                },
                "name": {
                  "timestamp": 1683352362
                },
                "endpoint": {
                  "timestamp": 1683352362
                },
                "enabled": {
                  "timestamp": 1686463233
                },
                "active": {
                  "timestamp": 1686463233
                }
              },
              "sch9": {
                "timer": {
                  "start": {
                    "timestamp": 1683352414
                  },
                  "end": {
                    "timestamp": 1683352414
                  }
                },
                "enabled": {
                  "timestamp": 1686511306
                },
                "active": {
                  "timestamp": 1686511306
                },
                "id": {
                  "timestamp": 1683352394
                },
                "name": {
                  "timestamp": 1683352394
                },
                "endpoint": {
                  "timestamp": 1683352394
                }
              },
              "supported": {
                "timestamp": 1683366233
              },
              "programmed": {
                "timestamp": 1683366233
              },
              "sch3": {
                "enabled": {
                  "timestamp": 1686470435
                },
                "active": {
                  "timestamp": 1686470435
                },
                "timer": {
                  "start": {
                    "timestamp": 1684799423
                  },
                  "end": {
                    "timestamp": 1684799423
                  }
                },
                "id": {
                  "timestamp": 1683366244
                },
                "name": {
                  "timestamp": 1683366244
                },
                "endpoint": {
                  "timestamp": 1683366244
                }
              },
              "sch4": {
                "enabled": {
                  "timestamp": 1686506430
                },
                "active": {
                  "timestamp": 1686506430
                },
                "id": {
                  "timestamp": 1683366242
                },
                "name": {
                  "timestamp": 1683366242
                },
                "endpoint": {
                  "timestamp": 1683366242
                },
                "timer": {
                  "start": {
                    "timestamp": 1684799528
                  },
                  "end": {
                    "timestamp": 1684799528
                  }
                }
              }
            },
            "hmi": {
              "ff": {
                "fn": {
                  "timestamp": 1652278522
                },
                "vr": {
                  "timestamp": 1652278522
                },
                "ts": {
                  "timestamp": 1652278875
                },
                "pg": {
                  "fs": {
                    "timestamp": 1652278522
                  },
                  "bd": {
                    "timestamp": 1652278875
                  },
                  "ts": {
                    "timestamp": 1652278522
                  },
                  "te": {
                    "timestamp": 1652278875
                  }
                }
              },
              "fw": {
                "fn": {
                  "timestamp": 1652278888
                },
                "vr": {
                  "timestamp": 1652278888
                }
              }
            },
            "main": {
              "ff": {
                "fn": {
                  "timestamp": 1652278916
                },
                "vr": {
                  "timestamp": 1652278916
                },
                "ts": {
                  "timestamp": 1652279183
                },
                "pg": {
                  "fs": {
                    "timestamp": 1652278917
                  },
                  "bd": {
                    "timestamp": 1652279182
                  },
                  "ts": {
                    "timestamp": 1652278917
                  },
                  "te": {
                    "timestamp": 1652279182
                  }
                }
              }
            },
            "debug_main": {
              "tr": {
                "timestamp": 1652279423
              }
            },
            "heating": {
              "enabled": {
                "timestamp": 1683352382
              },
              "state": {
                "timestamp": 1686506432
              },
              "priority_enabled": {
                "timestamp": 1683352398
              },
              "sp": {
                "timestamp": 1683352416
              },
              "sp_min": {
                "timestamp": 1683352416
              },
              "sp_max": {
                "timestamp": 1683352416
              }
            }
          }
        },
        "version": 2070906
      },
      "timestamp": 1686511306
    }
    "#;

    serde_json::from_str::<MqttResponse>(response);
  }
}

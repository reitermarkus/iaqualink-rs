use std::env;

use iaqualink::Client;

#[tokio::main]
async fn main() {
  let client = Client {
    email: env::var("EMAIL").unwrap(),
    password: env::var("PASSWORD").unwrap(),
  };

  let login_response = client.sign_in().await.unwrap();
  let _ = dbg!(&login_response);

  let devices_res = client.devices(&login_response).await;
  let _ = dbg!(devices_res);

  let cleaner_serial = "KF2000008286";
  let salt_serial = "JT20007118";

  let serial = cleaner_serial;
  let home_res = client.home(serial, &login_response).await;
  let _ = dbg!(home_res);

  let device_res = client.device(serial, &login_response).await;
  let _ = dbg!(device_res);

  let site_res = client.site(serial, &login_response).await;
  let _ = dbg!(site_res);

  let info_res = client.info(serial, &login_response).await;
  let _ = dbg!(info_res);

  let features_res = client.features(serial, &login_response).await;
  let _ = dbg!(features_res);

  let shadow_res = client.shadow(serial, &login_response).await;
  let _ = dbg!(shadow_res);

  // let set_shadow_res = client.set_shadow(serial, &login_response, serde_json::json!({
  //   "state": {
  //     "desired": {
  //       "equipment": {
  //         "swc_0": {
  //           "production": 1
  //         }
  //       }
  //     }
  //   }
  // })).await;
  // dbg!(set_shadow_res);
}

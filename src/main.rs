use std::env;

use iaqualink::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let client = Client { email: env::var("EMAIL").unwrap(), password: env::var("PASSWORD").unwrap() };

  let login_response = client.sign_in().await.expect("Login failed.");
  let _ = dbg!(&login_response);

  let devices_res = client.devices(&login_response).await?;
  let _ = dbg!(&devices_res);

  for device in &devices_res {
    let info_res = device.info(&login_response).await;
    let _ = dbg!(info_res);

    let features_res = device.features(&login_response).await;
    let _ = dbg!(features_res);

    let site_res = device.site(&login_response).await;
    let _ = dbg!(site_res);

    let shadow_res = device.shadow(&login_response).await;
    let _ = dbg!(shadow_res);

    let mut subscription = device.subscribe(&login_response).await?;

    loop {
      let (topic, response) = subscription.recv().await?;
      println!("Received {topic}: {response:#?}");
    }
  }

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

  Ok(())
}

# gira_iot_api

gira_iot_api enables you to controll your Gira X1 / Homeserver through the Gira IOT API.

:warning: **Warning:** This project is in a very early stage. Everything might change. Nevertheless feedback/contribution is welcome.

Example usage:
```rust
use gira_iot_api::x1::X1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let myx1 = X1::new("10.10.1.12", "Username", "My$up3rs3cur3P4$$w0rd");
    myx1.connect().await;
    myx1.get_ui().await;
    myx1.create_lights();
    myx1.lights.list();
    myx1.lights.tunable.list();
    myx1.lights.tunable.switch_on(9).await;
    myx1.lights.tunable.tune(9, 1000).await;
    Ok(())
}
```
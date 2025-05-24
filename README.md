# gira_iot_api

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
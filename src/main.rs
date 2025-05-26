use gira_iot_api::x1::X1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let myx1 = X1::new("10.10.1.12", "Username", "My$up3rs3cur3P4$$w0rd");
    myx1.connect().await;
    myx1.get_ui().await;
    myx1.create_devices().await;

    let list = myx1.lights.tunable.list();
    println!("{:?}", list);
    Ok(())
}

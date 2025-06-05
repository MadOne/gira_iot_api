use gira_iot_api::x1::X1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let myx1 = X1::new("10.10.1.12", "Username", "My$up3rs3cur3P4$$w0rd");
    myx1.connect().await;
    let a = myx1.get_ui().await;
    //println!("{a}");
    myx1.create_devices().await;

    let mut locations = myx1.create_locations();
    //println!("{locations:?}");
    myx1.set_location_id(&mut locations);
    println!("{locations:?}");
    //let list = myx1.lights.list().await;

    let a = myx1.lights.light.lock().await;
    println! {"{:?}", a};
    //println!("{:?}", list);
    Ok(())
}

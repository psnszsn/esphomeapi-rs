
use proto_esphome::api;

extern crate esphomeapi;
use esphomeapi::ApiClient;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let mut client = ApiClient::new("10.10.30.160:6053", Some("test".into()), "asdasdasd".into())?;
    client.connect()?;
    let info = client.device_info()?;
    println!("Device info: {:#?}", info);
    let services = client.list_entities_services();
    println!("Device info: {:#?}", services);
    let key = services.unwrap().into_iter().nth(0).unwrap().downcast::<api::ListEntitiesLightResponse>().unwrap().key;
    client.light_command(*key, Some(false), None,None,None,None,None,None,None)?;

    std::thread::sleep(std::time::Duration::from_millis(4000));

    Ok(())
}


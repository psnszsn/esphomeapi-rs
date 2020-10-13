use pb_jelly::Message;
use proto_esphome::api;
use downcast_rs::Downcast;


pub trait EspTrait: Downcast + std::fmt::Debug {
    fn get_no(&self) -> u32;

    fn to_vec(&self) -> Vec<u8>;

}
downcast_rs::impl_downcast!(EspTrait);

fn is_string(s: &dyn std::any::Any) {
    if s.is::<String>() {
        println!("It's a string!");
    } else {
        println!("Not a string...");
    }
}

fn test(s: Box<dyn EspTrait>){
    is_string(&s);
}

pub fn from_vec(u: u32, msg: Vec<u8>) -> std::io::Result<Box<dyn EspTrait>> {
    let b: Box<dyn EspTrait> = match u {
        1 => Box::new(api::HelloRequest::deserialize_from_slice(msg.as_slice())?),
        2 => Box::new(api::HelloResponse::deserialize_from_slice(msg.as_slice())?),
        3 => Box::new(api::ConnectRequest::deserialize_from_slice(msg.as_slice())?),
        4 => Box::new(api::ConnectResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        5 => Box::new(api::DisconnectRequest::deserialize_from_slice(
            msg.as_slice(),
        )?),
        6 => Box::new(api::DisconnectResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        7 => Box::new(api::PingRequest::deserialize_from_slice(msg.as_slice())?),
        8 => Box::new(api::PingResponse::deserialize_from_slice(msg.as_slice())?),
        9 => Box::new(api::DeviceInfoRequest::deserialize_from_slice(
            msg.as_slice(),
        )?),
        10 => Box::new(api::DeviceInfoResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        11 => Box::new(api::ListEntitiesRequest::deserialize_from_slice(
            msg.as_slice(),
        )?),
        12 => {
            Box::new(api::ListEntitiesBinarySensorResponse::deserialize_from_slice(msg.as_slice())?)
        }
        13 => Box::new(api::ListEntitiesCoverResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        14 => Box::new(api::ListEntitiesFanResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        15 => Box::new(api::ListEntitiesLightResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        16 => Box::new(api::ListEntitiesSensorResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        17 => Box::new(api::ListEntitiesSwitchResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        18 => Box::new(api::ListEntitiesTextSensorResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        19 => Box::new(api::ListEntitiesDoneResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        20 => Box::new(api::SubscribeStatesRequest::deserialize_from_slice(
            msg.as_slice(),
        )?),
        21 => Box::new(api::BinarySensorStateResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        22 => Box::new(api::CoverStateResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        23 => Box::new(api::FanStateResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        24 => Box::new(api::LightStateResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        25 => Box::new(api::SensorStateResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        26 => Box::new(api::SwitchStateResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        27 => Box::new(api::TextSensorStateResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        28 => Box::new(api::SubscribeLogsRequest::deserialize_from_slice(
            msg.as_slice(),
        )?),
        29 => Box::new(api::SubscribeLogsResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        30 => Box::new(api::CoverCommandRequest::deserialize_from_slice(
            msg.as_slice(),
        )?),
        31 => Box::new(api::FanCommandRequest::deserialize_from_slice(
            msg.as_slice(),
        )?),
        32 => Box::new(api::LightCommandRequest::deserialize_from_slice(
            msg.as_slice(),
        )?),
        33 => Box::new(api::SwitchCommandRequest::deserialize_from_slice(
            msg.as_slice(),
        )?),
        34 => Box::new(
            api::SubscribeHomeassistantServicesRequest::deserialize_from_slice(msg.as_slice())?,
        ),
        35 => Box::new(api::HomeassistantServiceResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        36 => Box::new(api::GetTimeRequest::deserialize_from_slice(msg.as_slice())?),
        37 => Box::new(api::GetTimeResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        38 => Box::new(
            api::SubscribeHomeAssistantStatesRequest::deserialize_from_slice(msg.as_slice())?,
        ),
        39 => Box::new(
            api::SubscribeHomeAssistantStateResponse::deserialize_from_slice(msg.as_slice())?,
        ),
        40 => Box::new(api::HomeAssistantStateResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        41 => Box::new(api::ListEntitiesServicesResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        42 => Box::new(api::ExecuteServiceRequest::deserialize_from_slice(
            msg.as_slice(),
        )?),
        43 => Box::new(api::ListEntitiesCameraResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        44 => Box::new(api::CameraImageResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        45 => Box::new(api::CameraImageRequest::deserialize_from_slice(
            msg.as_slice(),
        )?),
        46 => Box::new(api::ListEntitiesClimateResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        47 => Box::new(api::ClimateStateResponse::deserialize_from_slice(
            msg.as_slice(),
        )?),
        48 => Box::new(api::ClimateCommandRequest::deserialize_from_slice(
            msg.as_slice(),
        )?),

        // _ => Box::new(api::PingResponse::deserialize_from_slice(msg.as_slice())?),
        _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))?,
    };
    Ok(b)
}

macro_rules! impl_esptrait {
    ($type:path, $n:expr) => {
        impl EspTrait for $type {
            fn get_no(&self) -> u32 {
                $n
            }

            fn to_vec(&self) -> Vec<u8> {
                self.serialize_to_vec()
            }
        }
    };
}
impl_esptrait!(api::HelloRequest, 1);
impl_esptrait!(api::HelloResponse, 2);
impl_esptrait!(api::ConnectRequest, 3);
impl_esptrait!(api::ConnectResponse, 4);
impl_esptrait!(api::DisconnectRequest, 5);
impl_esptrait!(api::DisconnectResponse, 6);
impl_esptrait!(api::PingRequest, 7);
impl_esptrait!(api::PingResponse, 8);
impl_esptrait!(api::DeviceInfoRequest, 9);
impl_esptrait!(api::DeviceInfoResponse, 10);
impl_esptrait!(api::ListEntitiesRequest, 11);
impl_esptrait!(api::ListEntitiesBinarySensorResponse, 12);
impl_esptrait!(api::ListEntitiesCoverResponse, 13);
impl_esptrait!(api::ListEntitiesFanResponse, 14);
impl_esptrait!(api::ListEntitiesLightResponse, 15);
impl_esptrait!(api::ListEntitiesSensorResponse, 16);
impl_esptrait!(api::ListEntitiesSwitchResponse, 17);
impl_esptrait!(api::ListEntitiesTextSensorResponse, 18);
impl_esptrait!(api::ListEntitiesDoneResponse, 19);
impl_esptrait!(api::SubscribeStatesRequest, 20);
impl_esptrait!(api::BinarySensorStateResponse, 21);
impl_esptrait!(api::CoverStateResponse, 22);
impl_esptrait!(api::FanStateResponse, 23);
impl_esptrait!(api::LightStateResponse, 24);
impl_esptrait!(api::SensorStateResponse, 25);
impl_esptrait!(api::SwitchStateResponse, 26);
impl_esptrait!(api::TextSensorStateResponse, 27);
impl_esptrait!(api::SubscribeLogsRequest, 28);
impl_esptrait!(api::SubscribeLogsResponse, 29);
impl_esptrait!(api::CoverCommandRequest, 30);
impl_esptrait!(api::FanCommandRequest, 31);
impl_esptrait!(api::LightCommandRequest, 32);
impl_esptrait!(api::SwitchCommandRequest, 33);
impl_esptrait!(api::SubscribeHomeassistantServicesRequest, 34);
impl_esptrait!(api::HomeassistantServiceResponse, 35);
impl_esptrait!(api::GetTimeRequest, 36);
impl_esptrait!(api::GetTimeResponse, 37);
impl_esptrait!(api::SubscribeHomeAssistantStatesRequest, 38);
impl_esptrait!(api::SubscribeHomeAssistantStateResponse, 39);
impl_esptrait!(api::HomeAssistantStateResponse, 40);
impl_esptrait!(api::ListEntitiesServicesResponse, 41);
impl_esptrait!(api::ExecuteServiceRequest, 42);
impl_esptrait!(api::ListEntitiesCameraResponse, 43);
impl_esptrait!(api::CameraImageResponse, 44);
impl_esptrait!(api::CameraImageRequest, 45);
impl_esptrait!(api::ListEntitiesClimateResponse, 46);
impl_esptrait!(api::ClimateStateResponse, 47);
impl_esptrait!(api::ClimateCommandRequest, 48);

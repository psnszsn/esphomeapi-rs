use pb_jelly::Message;
use proto_esphome::api;
use std::convert::TryFrom;

macro_rules! impl_foos{($($no: expr => $enumvariant: ident($foo: ty),)*) => {

    #[derive(Debug)]
    pub enum EspMessage {
        $($enumvariant($foo),)*
    }
    impl EspMessage {
        pub fn to_vec(&self) -> Vec<u8> {
            match self {
                $(EspMessage::$enumvariant(x) => x.serialize_to_vec(),)*
            }
        }
    }

    impl From<EspMessage> for u32{
        fn from(m: EspMessage) -> Self {
            match m{
                $(EspMessage::$enumvariant(_) => $no,)*
            }
        }
    }

    impl TryFrom<(u32, Vec<u8>)> for EspMessage{
        type Error = Box<dyn std::error::Error>;
        fn try_from(f: (u32,Vec<u8>)) -> Result<Self,Self::Error> {
            let (num, data) =f;
            let a = match num{
                $($no => Ok(EspMessage::$enumvariant(Message::deserialize_from_slice(data.as_slice())?)),)*
                _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Asd"))?
            };
            a
        }
    }

    $(
    impl From<$foo> for EspMessage{
        fn from(m: $foo) -> Self {
            EspMessage::$enumvariant(m)
        }
    })*

}}

impl_foos!(
     1=>HelloRequest(api::HelloRequest),
     2=>HelloResponse(api::HelloResponse),
     3=>ConnectRequest(api::ConnectRequest),
     4=>ConnectResponse(api::ConnectResponse),
     5=>DisconnectRequest(api::DisconnectRequest),
     6=>DisconnectResponse(api::DisconnectResponse),
     7=>PingRequest(api::PingRequest),
     8=>PingResponse(api::PingResponse),
     9=>DeviceInfoRequest(api::DeviceInfoRequest),
     10=>DeviceInfoResponse(api::DeviceInfoResponse),
     11=>ListEntitiesRequest(api::ListEntitiesRequest),
     12=>ListEntitiesBinarySensorResponse(api::ListEntitiesBinarySensorResponse),
     13=>ListEntitiesCoverResponse(api::ListEntitiesCoverResponse),
     14=>ListEntitiesFanResponse(api::ListEntitiesFanResponse),
     15=>ListEntitiesLightResponse(api::ListEntitiesLightResponse),
     16=>ListEntitiesSensorResponse(api::ListEntitiesSensorResponse),
     17=>ListEntitiesSwitchResponse(api::ListEntitiesSwitchResponse),
     18=>ListEntitiesTextSensorResponse(api::ListEntitiesTextSensorResponse),
     19=>ListEntitiesDoneResponse(api::ListEntitiesDoneResponse),
     20=>SubscribeStatesRequest(api::SubscribeStatesRequest),
     21=>BinarySensorStateResponse(api::BinarySensorStateResponse),
     22=>CoverStateResponse(api::CoverStateResponse),
     23=>FanStateResponse(api::FanStateResponse),
     24=>LightStateResponse(api::LightStateResponse),
     25=>SensorStateResponse(api::SensorStateResponse),
     26=>SwitchStateResponse(api::SwitchStateResponse),
     27=>TextSensorStateResponse(api::TextSensorStateResponse),
     28=>SubscribeLogsRequest(api::SubscribeLogsRequest),
     29=>SubscribeLogsResponse(api::SubscribeLogsResponse),
     30=>CoverCommandRequest(api::CoverCommandRequest),
     31=>FanCommandRequest(api::FanCommandRequest),
     32=>LightCommandRequest(api::LightCommandRequest),
     33=>SwitchCommandRequest(api::SwitchCommandRequest),
     34=>SubscribeHomeassistantServicesRequest(api::SubscribeHomeassistantServicesRequest),
     35=>HomeassistantServiceResponse(api::HomeassistantServiceResponse),
     36=>GetTimeRequest(api::GetTimeRequest),
     37=>GetTimeResponse(api::GetTimeResponse),
     38=>SubscribeHomeAssistantStatesRequest(api::SubscribeHomeAssistantStatesRequest),
     39=>SubscribeHomeAssistantStateResponse(api::SubscribeHomeAssistantStateResponse),
     40=>HomeAssistantStateResponse(api::HomeAssistantStateResponse),
     41=>ListEntitiesServicesResponse(api::ListEntitiesServicesResponse),
     42=>ExecuteServiceRequest(api::ExecuteServiceRequest),
     43=>ListEntitiesCameraResponse(api::ListEntitiesCameraResponse),
     44=>CameraImageResponse(api::CameraImageResponse),
     45=>CameraImageRequest(api::CameraImageRequest),
     46=>ListEntitiesClimateResponse(api::ListEntitiesClimateResponse),
     47=>ClimateStateResponse(api::ClimateStateResponse),
     48=>ClimateCommandRequest(api::ClimateCommandRequest),
);

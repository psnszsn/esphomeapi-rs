use super::connection::ApiConnection;
use super::message;
use proto_esphome::api;
use thiserror::Error;

pub struct ApiClient<A: std::net::ToSocketAddrs> {
    addr: A,
    connection: Option<ApiConnection>,
    client_info: String,
    password: String,
}
impl<A: std::net::ToSocketAddrs> ApiClient<A> {
    pub fn new(
        addr: A,
        password: Option<String>,
        client_info: String,
    ) -> Result<ApiClient<A>, Box<dyn std::error::Error>> {
        Ok(ApiClient {
            addr,
            connection: None,
            client_info,
            password: password.unwrap_or("".into()),
        })
    }
    pub fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.connection = Some(ApiConnection::new(
            &self.addr,
            self.password.clone(),
            &self.client_info,
        )?);
        self.connection.as_mut().unwrap().login();
        Ok(())
    }
    pub fn device_info(&mut self) -> Result<api::DeviceInfoResponse, ClientError> {
        let req = api::DeviceInfoRequest {};
        let resp = self
            .connection
            .as_mut()
            .ok_or(ClientError::Unknown)?
            .write_message_read_response(req)
            .map_err(|_| ClientError::Unknown)?;

        if resp.as_any().is::<api::DeviceInfoResponse>() {
            // panic!();
            let r = resp.downcast::<api::DeviceInfoResponse>().unwrap();
            Ok(*r)
        } else {
            Err(ClientError::Unknown)
        }
    }

    pub fn list_entities_services(
        &mut self,
    ) -> Result<Vec<Box<dyn message::EspTrait>>, ClientError> {
        let req = api::ListEntitiesRequest {};
        let connection = self.connection.as_mut().ok_or(ClientError::Unknown)?;

        connection.write_message(req);
        let mut v: Vec<Box<dyn message::EspTrait>> = Vec::new();

        loop {
            let resp = connection.read_response().unwrap();
            if resp.as_any().is::<api::ListEntitiesDoneResponse>() {
                break;
            } else {
                v.push(resp);
            };
        }
        Ok(v)
    }
    pub fn light_command(
        &mut self,
        key: u32,
        state: Option<bool>,
        brightness: Option<f32>,
        rgb: Option<(f32, f32, f32)>,
        white: Option<f32>,
        color_temperature: Option<f32>,
        transition_length: Option<u32>,
        flash_length: Option<u32>,
        effect: Option<String>,
    ) -> Result<(), ClientError> {
        let req = api::LightCommandRequest {
            key: pb_jelly::Fixed32(key),
            has_state: state.is_some(),
            state: state.unwrap_or(bool::default()),
            has_rgb: rgb.is_some(),
            red: rgb.unwrap_or(Default::default()).0,
            green: rgb.unwrap_or(Default::default()).1,
            blue: rgb.unwrap_or(Default::default()).2,
            has_brightness: brightness.is_some(),
            brightness: brightness.unwrap_or(Default::default()),
            has_white: white.is_some(),
            white: white.unwrap_or(Default::default()),
            has_color_temperature: color_temperature.is_some(),
            color_temperature: color_temperature.unwrap_or(Default::default()),
            has_transition_length: transition_length.is_some(),
            transition_length: transition_length.unwrap_or(Default::default()),
            has_flash_length: flash_length.is_some(),
            flash_length: flash_length.unwrap_or(Default::default()),
            has_effect: effect.is_some(),
            effect: effect.unwrap_or(Default::default()),
        };

        let light = api::LightCommandRequest {
            key: pb_jelly::Fixed32(1274414981),
            has_state: true,
            state: false,
            ..api::LightCommandRequest::default()
        };
        let connection = self.connection.as_mut().ok_or(ClientError::Unknown)?;
        connection.write_message(req)?;
        Ok(())
    }
}
#[derive(Error, Debug)]
pub enum ClientError {
    #[error("data store disconnected")]
    ConnectionError(#[from] Box<dyn std::error::Error>),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

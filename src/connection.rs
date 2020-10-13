use super::message;
use integer_encoding::{VarInt, VarIntReader};
use proto_esphome::api;
use std::io::{Read, Write};
use std::net::TcpStream;

pub struct ApiConnection {
    password: String,
    authenticated: bool,
    tcp: TcpStream,
}
impl ApiConnection {
    pub fn new(
        addr: &impl std::net::ToSocketAddrs,
        password: String,
        client_info: &str,
    ) -> Result<ApiConnection, Box<dyn std::error::Error>> {
        let mut new = ApiConnection {
            tcp: TcpStream::connect(addr).unwrap(),
            authenticated: false,
            password,
        };

        let hello = api::HelloRequest {
            client_info: client_info.to_string(),
        };

        let resp = new.write_message_read_response(hello)?;

        if !resp.as_any().is::<api::HelloResponse>() {
            panic!();
        }
        Ok(new)
    }
    pub fn login(&mut self) {
        let connect = api::ConnectRequest {
            password: self.password.clone(),
        };
        self.write_message_read_response(connect).unwrap();
        self.authenticated = true;
    }

    pub fn write_message(
        &mut self,
        message: impl message::EspTrait,
    ) -> Result<(), Box<dyn std::error::Error>>
 {
        let raw_msg = message.to_vec();

        // let h_str = hello.serialize();
        let leng = raw_msg.len().encode_var_vec();
        let msg_type = message.get_no().encode_var_vec();

        println!("Length {:X?}", leng);
        println!("Bytes 0x{:X?}", raw_msg);

        let req = [vec![0], leng, msg_type, raw_msg].concat();

        self.tcp.write(req.as_slice())?;
        Ok(())
    }

    pub fn read_response(
       &mut self,
    ) -> Result<Box<dyn message::EspTrait>, Box<dyn std::error::Error>>
    {
        let stream = &mut self.tcp;
        let preamble = read_n(stream, 1);

        if preamble[0] != 0x00 {
            panic!();
        }

        let length: u32 = stream.read_varint()?;
        // println!("len {:X?}", length);
        let msg_type: u32 = stream.read_varint()?;
        // println!("type {:X?}", msg_type);

        let msg_raw = read_n(stream, length as u64);
        let msg = message::from_vec(msg_type, msg_raw);
        println!("message {:#?}", msg);

        Ok(msg?)
    }

    pub fn write_message_read_response(
        &mut self,
        message: impl message::EspTrait,
    ) -> Result<Box<dyn message::EspTrait>, Box<dyn std::error::Error>> {
        // stream.write(message.as_slice())?;
        self.write_message(message)?;

        println!("Sent Hello, awaiting reply...");
        self.read_response()
    }
}

fn read_n<R>(reader: &mut R, bytes_to_read: u64) -> Vec<u8>
where
    R: Read,
{
    let mut buf = vec![];
    let mut chunk = reader.take(bytes_to_read);
    // Do appropriate error handling for your situation
    // Maybe it's OK if you didn't read enough bytes?
    let n = chunk.read_to_end(&mut buf).expect("Didn't read enough");
    assert_eq!(bytes_to_read as usize, n);
    buf
}

use tokio_core::reactor::Core;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Encoder, Decoder, Framed};
use tokio_proto::pipeline::ServerProto;
use tokio_proto::pipeline::ClientProto;
use tokio_service::Service;
use tokio_uds_proto::UnixClient;
use futures::{future, Future};
use bytes::BytesMut;

use mrsc;
use serde_json;

use std::io;
use std::str;
use std::sync::Mutex;
use std::sync::Arc;

use blocks::BlockPointer;
use recipe::BlockRecipe;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CtlRequest {
    Ping,
    Write(BlockRecipe),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CtlResponse {
    Pong,
    Ack(BlockPointer),
}


pub struct JsonCodec;

impl Decoder for JsonCodec {
    type Item = serde_json::Value;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<serde_json::Value>> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            let line = buf.split_to(i);

            buf.split_to(1);

            match str::from_utf8(&line) {
                Ok(s) => {
                    match serde_json::from_str(&s) {
                        Ok(value) => Ok(Some(value)),
                        Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid json")),
                    }
                },
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid UTF-8")),
            }
        } else {
            Ok(None)
        }
    }

}

impl Encoder for JsonCodec {
    type Item = serde_json::Value;
    type Error = io::Error;

    fn encode(&mut self, msg: serde_json::Value, buf: &mut BytesMut) -> io::Result<()> {
        let msg = msg.to_string();
        buf.extend(msg.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}


pub struct CtlProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for CtlProto {
    type Request = serde_json::Value;
    type Response = serde_json::Value;

    type Transport = Framed<T, JsonCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(JsonCodec))
    }
}

impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for CtlProto {
    type Request = serde_json::Value;
    type Response = serde_json::Value;

    type Transport = Framed<T, JsonCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(JsonCodec))
    }
}


type Channel = mrsc::Channel<CtlRequest, CtlResponse>;
pub struct CtlService(Channel);

impl CtlService {
    pub fn new(channel: Arc<Mutex<Channel>>) -> CtlService {
        let lock = channel.lock();
        CtlService(lock.unwrap().clone())
    }

    fn req(&self, req: CtlRequest) -> CtlResponse {

        debug!("ctl(req): {:?}", req);
        let reply = self.0.req(req.into()).unwrap();
        let resp = reply.recv().unwrap();
        debug!("ctl(resp): {:?}", resp);

        resp
    }
}

impl Service for CtlService {
    type Request = serde_json::Value;
    type Response = serde_json::Value;

    type Error = io::Error;

    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let req: CtlRequest = match serde_json::from_value(req) {
            Ok(req) => req,
            Err(_) => return Box::new(future::err(io::Error::new(io::ErrorKind::Other, "invalid json"))),
        };

        let resp = self.req(req);
        let resp = serde_json::to_value(resp).unwrap();
        Box::new(future::ok(resp))
    }
}

pub struct Client {
    socket: String,
}

impl Client {
    pub fn new<I: Into<String>>(socket: I) -> Client {
        Client {
            socket: socket.into(),
        }
    }

    // TODO: this doesn't fail if the server disconnects
    pub fn send(&self, req: &CtlRequest) -> Result<CtlResponse, io::Error> {
        let mut core = Core::new()?;

        let server = UnixClient::new(CtlProto);
        let uds = server.connect(&self.socket, &core.handle())?;

        debug!("ctl(req): {:?}", req);
        let cmd = serde_json::to_value(req)?;
        let reply = core.run(uds.call(cmd))?;
        debug!("ctl(reply): {:?}", reply);

        let reply = serde_json::from_value(reply)?;

        Ok(reply)
    }

    #[inline]
    pub fn write_block(&self, block: BlockRecipe) -> Result<BlockPointer, io::Error> {
        let reply = self.send(&CtlRequest::Write(block))?;

        match reply {
            CtlResponse::Ack(pointer) => Ok(pointer),
            _ => Err(io::Error::new(io::ErrorKind::Other, "invalid reply")),
        }
    }
}

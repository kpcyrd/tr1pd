use scaproust::{self, SessionBuilder, Session, Ipc, Req, Rep};
use serde_json;

use std::str;

use blocks::BlockPointer;
use recipe::BlockRecipe;

pub mod errors {
    use std::io;
    use std::str;
    use serde_json;

    use rpc::CtlResponse;

    error_chain! {
        errors {
            InvalidResponse(reply: CtlResponse) {
                description("invalid response")
                display("invalid response: {:?}", reply)
            }
        }

        foreign_links {
            Io(io::Error);
            Json(serde_json::Error);
            Utf8(str::Utf8Error);
        }
    }
}
use self::errors::{Result, ErrorKind};


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

pub fn create_session() -> Session {
    SessionBuilder::new()
        .with("ipc", Ipc)
        .build().unwrap()
}

pub struct Server {
    #[allow(dead_code)]
    session: scaproust::Session,
    socket: scaproust::Socket,
}

impl Server {
    pub fn bind(url: &str) -> Result<Server> {
        let mut session = create_session();
        let mut socket = session.create_socket::<Rep>()?;

        socket.bind(url)?;

        Ok(Server {
            session,
            socket,
        })
    }

    pub fn recv(&mut self) -> Result<CtlRequest> {
        let req = self.socket.recv()?;
        debug!("ctl(req, raw): {:?}", req);
        let string = str::from_utf8(&req)?;
        let request = serde_json::from_str(string)?;
        debug!("ctl(req): {:?}", request);
        Ok(request)
    }

    pub fn reply(&mut self, reply: CtlResponse) -> Result<()> {
        debug!("ctl(resp): {:?}", reply);
        let response = serde_json::to_string(&reply)?;
        let buffer = response.as_bytes().to_vec();
        self.socket.send(buffer)?;
        Ok(())
    }
}

pub struct ClientBuilder {
    url: String,
}

impl ClientBuilder {
    pub fn new<I: Into<String>>(url: I) -> ClientBuilder {
        ClientBuilder {
            url: url.into(),
        }
    }

    pub fn connect(&self) -> Result<Client> {
        let mut session = create_session();
        let mut socket = session.create_socket::<Req>()?;

        socket.connect(&self.url)?;

        Ok(Client {
            session,
            socket,
        })
    }
}

pub struct Client {
    #[allow(dead_code)]
    session: scaproust::Session,
    socket: scaproust::Socket,
}

impl Client {
    pub fn send(&mut self, req: &CtlRequest) -> Result<CtlResponse> {
        debug!("ctl(req): {:?}", req);
        let request = serde_json::to_string(req)?;
        self.socket.send(request.as_bytes().to_vec())?;
        let buffer = self.socket.recv()?;

        let string = str::from_utf8(&buffer)?;
        let reply = serde_json::from_str(&string)?;
        debug!("ctl(reply): {:?}", reply);

        Ok(reply)
    }

    #[inline]
    pub fn write_block(&mut self, block: BlockRecipe) -> Result<BlockPointer> {
        let reply = self.send(&CtlRequest::Write(block))?;

        match reply {
            CtlResponse::Ack(pointer) => Ok(pointer),
            _ => Err(ErrorKind::InvalidResponse(reply).into()),
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        use std::time;
        use std::thread;
        thread::sleep(time::Duration::from_millis(50)); // dirty workaround for linger
    }
}

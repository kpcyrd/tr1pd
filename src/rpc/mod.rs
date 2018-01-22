use zmq;
use serde_json;

use std::str;
use std::fs::{self, Permissions};
use std::os::unix::fs::PermissionsExt;

use blocks::BlockPointer;
use recipe::BlockRecipe;

mod errors {
    use std::io;
    use std::str;
    use serde_json;
    use zmq;

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
            Zmq(zmq::Error);
        }
    }
}
pub use self::errors::{Result, Error, ErrorKind};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CtlRequest {
    Ping,
    Write(BlockRecipe),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CtlResponse {
    Pong,
    Ack(BlockPointer),
    Nack,
}

pub struct Server {
    #[allow(dead_code)]
    ctx: zmq::Context,
    socket: zmq::Socket,
}

impl Server {
    pub fn bind(url: &str) -> Result<Server> {
        let ctx = zmq::Context::new();
        let socket = ctx.socket(zmq::REP)?;

        socket.bind(url)?;

        // fix permissions
        if url.starts_with("ipc://") {
            // TODO: write a proper solution
            let perms = Permissions::from_mode(0o770);
            fs::set_permissions(&url[6..], perms)?;
        }

        Ok(Server {
            ctx,
            socket,
        })
    }

    pub fn recv(&mut self) -> Result<CtlRequest> {
        let req = self.socket.recv_msg(0)?;
        debug!("ctl(req, raw): {:?}", req);
        let string = str::from_utf8(&req)?;
        let request = serde_json::from_str(string)?;
        debug!("ctl(req): {:?}", request);
        Ok(request)
    }

    pub fn reply(&mut self, reply: CtlResponse) -> Result<()> {
        debug!("ctl(resp): {:?}", reply);
        let response = serde_json::to_string(&reply)?;
        self.socket.send(response.as_bytes(), 0)?;
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
        let ctx = zmq::Context::new();
        let socket = ctx.socket(zmq::REQ)?;

        socket.connect(&self.url)?;

        Ok(Client {
            ctx,
            socket,
        })
    }
}

pub struct Client {
    #[allow(dead_code)]
    ctx: zmq::Context,
    socket: zmq::Socket,
}

impl Client {
    pub fn send(&mut self, req: &CtlRequest) -> Result<CtlResponse> {
        debug!("ctl(req): {:?}", req);
        let request = serde_json::to_string(req)?;
        self.socket.send(request.as_bytes(), 0)?;
        let buffer = self.socket.recv_msg(0)?;

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

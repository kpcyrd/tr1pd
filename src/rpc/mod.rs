use zmq;

use std::fs::{self, Permissions};
use std::os::unix::fs::PermissionsExt;

use blocks::BlockPointer;
use recipe::BlockRecipe;

#[allow(unused_variables)]
mod wire;

mod errors {
    use std;
    use zmq;

    use rpc::CtlResponse;

    error_chain! {
        errors {
            InvalidRecipe(recipe: Vec<u8>) {
                description("invalid recipe")
                display("invalid recipe: {:?}", recipe)
            }
            InvalidRequest(req: Vec<u8>) {
                description("invalid request")
                display("invalid request: {:?}", req)
            }
            InvalidResponse(resp: Vec<u8>) {
                description("invalid response")
                display("invalid response: {:?}", resp)

            }

            UnexpectedResponse(reply: CtlResponse) {
                description("unexpected response")
                display("unexpected response: {:?}", reply)
            }
        }

        foreign_links {
            Io(std::io::Error);
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
        let bytes = self.socket.recv_msg(0)?;
        debug!("ctl(req, raw): {:?}", bytes);

        let request = CtlRequest::decode(&bytes)?;
        debug!("ctl(req): {:?}", request);
        Ok(request)
    }

    pub fn reply(&mut self, reply: &CtlResponse) -> Result<()> {
        debug!("ctl(resp): {:?}", reply);

        let mut bytes = Vec::new();
        reply.encode(&mut bytes);
        self.socket.send(&bytes, 0)?;

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

        let mut bytes = Vec::new();
        req.encode(&mut bytes);
        self.socket.send(&bytes, 0)?;

        let bytes = self.socket.recv_msg(0)?;
        let reply = CtlResponse::decode(&bytes)?;
        debug!("ctl(reply): {:?}", reply);

        Ok(reply)
    }

    #[inline]
    pub fn write_block(&mut self, block: BlockRecipe) -> Result<BlockPointer> {
        let reply = self.send(&CtlRequest::Write(block))?;

        match reply {
            CtlResponse::Ack(pointer) => Ok(pointer),
            _ => Err(ErrorKind::UnexpectedResponse(reply).into()),
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

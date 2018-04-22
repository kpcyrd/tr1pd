use nom::{IResult, be_u8, be_u16};

use rpc::{BlockRecipe, CtlRequest, CtlResponse};
use rpc::errors::{Result, ErrorKind};
use wire::{pointer, len_to_u16_vec};


impl BlockRecipe {
    pub fn encode(&self, buf: &mut Vec<u8>) {
        use self::BlockRecipe::*;
        match *self {
            Rekey => { buf.extend(b"\x00"); },
            Info(ref bytes) => {
                buf.extend(b"\x01");
                buf.extend(&len_to_u16_vec(bytes.len()).expect("block len overflow"));
                buf.extend(bytes);
            },
        }
    }

    pub fn decode(buf: &[u8]) -> Result<BlockRecipe> {
        if let IResult::Done(_, recipe) = recipe(&buf) {
            Ok(recipe)
        } else {
            Err(ErrorKind::InvalidRecipe(buf.to_vec()).into())
        }
    }
}

fn recipe_info(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    do_parse!(input,
        length: be_u16          >>
        bytes: take!(length)    >>
        ({
            bytes.to_vec()
        })
    )
}

fn recipe(input: &[u8]) -> IResult<&[u8], BlockRecipe> {
    do_parse!(input,
        recipe: switch!(be_u8,
            0x00 => value!(BlockRecipe::Rekey) |
            0x01 => map!(recipe_info, BlockRecipe::Info)
        ) >>
        (recipe)
    )
}

impl CtlRequest {
    pub fn encode(&self, buf: &mut Vec<u8>) {
        use self::CtlRequest::*;
        match *self {
            Ping => { buf.extend(b"\x00"); },
            Write(ref recipe) => {
                buf.extend(b"\x01");
                recipe.encode(buf);
            },
        }
    }

    pub fn decode(buf: &[u8]) -> Result<CtlRequest> {
        if let IResult::Done(_, request) = request(&buf) {
            Ok(request)
        } else {
            Err(ErrorKind::InvalidRequest(buf.to_vec()).into())
        }
    }
}

fn request(input: &[u8]) -> IResult<&[u8], CtlRequest> {
    do_parse!(input,
        request: switch!(be_u8,
            0x00 => value!(CtlRequest::Ping) |
            0x01 => map!(recipe, CtlRequest::Write)
        ) >>
        (request)
    )
}

impl CtlResponse {
    pub fn encode(&self, buf: &mut Vec<u8>) {
        use self::CtlResponse::*;
        match *self {
            Pong => { buf.extend(b"\x00"); },
            Ack(ref pointer) => {
                buf.extend(b"\x01");
                buf.extend(pointer.bytes());
            },
            Nack => { buf.extend(b"\x02"); },
        }
    }

    pub fn decode(buf: &[u8]) -> Result<CtlResponse> {
        if let IResult::Done(_, response) = response(&buf) {
            Ok(response)
        } else {
            Err(ErrorKind::InvalidResponse(buf.to_vec()).into())
        }
    }
}

fn response(input: &[u8]) -> IResult<&[u8], CtlResponse> {
    do_parse!(input,
        response: switch!(be_u8,
            0x00 => value!(CtlResponse::Pong) |
            0x01 => map!(pointer, CtlResponse::Ack) |
            0x02 => value!(CtlResponse::Nack)
        ) >>
        (response)
    )
}

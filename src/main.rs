extern crate openssl;

use std::os;
use std::io::{IoResult, TcpStream};
use openssl::ssl::{SslContext, Sslv23, SslStream};

pub enum Status {
    Auth,
    NonAuth,
    Logout
}

pub struct GmailSocket {
    _stream: SslStream<TcpStream>,
    _length: int,
    _status: Status
}

impl GmailSocket {
    pub fn connect() -> IoResult<GmailSocket> {
        let tcpstream = try!(TcpStream::connect("imap.gmail.com", 993));
        //let tcpstream = try!(TcpStream::connect("cert-test.sandbox.google.com", 443));
        let mut sslcontext = SslContext::new(Sslv23);
        match sslcontext.set_CA_file("./cert.pem") {
            None => {},
            Some(e) => fail!("{}", e)
        }
        let sslstream = SslStream::new(&sslcontext, tcpstream);
        Ok(GmailSocket {_stream: sslstream, _length: 0, _status: NonAuth})
    }

    pub fn fetch_from_github(&mut self) {
        println!("0");
        self._stream.write_str("GET / HTTP/1.1\r\nHost: cert-test.sandbox.google.com\r\nAccept: */*\r\n\r\n");
        println!("{}", self._stream.read_to_string().ok());
    }

    pub fn cap(&mut self) {
        println!("0");
        self._stream.write_str("C0 CAPABILITY\r\n");
        println!("1");
        println!("{}", self._stream.read_to_string().ok());
        println!("2");
    }

    pub fn login(&mut self, username: &str, password: &str) -> IoResult<bool> {
        let req = format!("L0{:d} LOGIN {:s} {:s}\r\n", self.inc(), username, password);
        try!(self._stream.write_str(req.as_slice()));
        println!("0");
        match try!(self._stream.read_to_string()) {
            res => println!("{}", res)
        }
        println!("1");
        Ok(true)
    }

    pub fn logout(&mut self) -> IoResult<bool> {
        let req = format!("L0{:d} LOGOUT\r\n", self.inc());
        try!(self._stream.write_str(req.as_slice()));
        match try!(self._stream.read_to_string()) {
            res => println!("{}", res)
        }
        Ok(true)
    }

    fn inc(&mut self) -> int {
        self._length += 1;
        self._length
    }
}

fn main() {
    match os::args().tail() {
        [ref username, ref password] => {
            let mut gmail_socket = match GmailSocket::connect() {
                Ok(s) => s,
                Err(e) => fail!("{}", e)
            };
            gmail_socket.cap();
            //gmail_socket.fetch_from_github();
            //println!("{}", gmail_socket.login(username.as_slice(), password.as_slice()));
            //println!("{}", gmail_socket.logout());
        },
        _ => fail!("Arguments error")
    }
}

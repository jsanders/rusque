use std::rt::io::{Reader, Writer};
use std::rt::io::net::ip::{Ipv4Addr,SocketAddr};
use std::rt::io::net::tcp::TcpStream;
use std::str;

// This functionality will exist in default to_uint after [0] is merged..
// [0]: https://github.com/mozilla/rust/pull/9250
trait ToUintSafe {
  fn to_uint_safe(&self) -> Option<uint>;
}

impl ToUintSafe for int {
  fn to_uint_safe(&self) -> Option<uint> {
    if *self < 0 {
      None
    } else {
      Some(self.to_uint())
    }
  }
}

struct Redis {
  stream: TcpStream
}

enum Reply {
  BulkReply(Option<~str>),
  IntegerReply(int)
}

impl Redis {
  fn connect() -> Redis {
    let ip_addr = Ipv4Addr(127, 0, 0, 1);
    let tcp_addr = SocketAddr { ip: ip_addr, port: 6379 };
    Redis { stream: TcpStream::connect(tcp_addr).expect("Failed to connect to redis on 127.0.0.1:6379!") }
  }

  fn write_command(&mut self, args: &[&str]) {
    let num_args = format!("*{:u}\r\n", args.len());
    self.stream.write(num_args.as_bytes());

    for arg in args.iter() {
      let arg_len = format!("${:u}\r\n", arg.len());
      self.stream.write(arg_len.as_bytes());

      let arg = format!("{:s}\r\n", *arg);
      self.stream.write(arg.as_bytes());
    }
  }

  fn read_reply(&mut self) -> Reply {
    let mut buf = ~[0u8, ..1024];
    let bytes_read = self.stream.read(buf).expect("Failed to get reply from redis server");
    let reply = str::from_utf8(buf.slice_to(bytes_read));
    let (header, rest) = match reply.find_str("\r\n") {
      None => fail!("Invalid reply"),
      Some(index) => {
        ( reply.slice_to(index), reply.slice_from(index + 2) )
      }
    };

    let (sentinel, header_data) = (header[0] as char, header.slice_from(1));
    match sentinel {
      '$' => self.read_bulk_reply(header_data, rest),
      ':' => self.read_integer_reply(header_data, rest),
      _   => fail!("Invalid reply")
    }
  }

  fn read_bulk_reply(&self, len: &str, rest: &str) -> Reply {
    let len: int = from_str(len).expect("Invalid bulk reply");
    BulkReply(match len {
      -1 => None,
      _  => {
        let len = len.to_uint_safe().expect("Invalid bulk reply");
        Some(rest.slice_to(len).to_owned())
      }
    })
  }

  fn read_integer_reply(&self, integer: &str, rest: &str) -> Reply {
    assert!(rest.len() == 0, "Invalid integer reply");

    IntegerReply(from_str::<int>(integer).expect("Invalid integer reply"))
  }

  fn do_command(&mut self, args: &[&str]) -> Reply {
    self.write_command(args);
    self.read_reply()
  }

  fn lpop(&mut self, key: &str) -> Option<~str> {
    match self.do_command(&[ &"lpop", key ]) {
      BulkReply(reply) => reply,
      _ => fail!("Protocol error: `lpop` expects bulk reply")
    }
  }

  fn lpush(&mut self, key: &str, value: &str) -> int {
    match self.do_command(&[ &"lpush", key, value ]) {
      IntegerReply(reply) => reply,
      _ => fail!("Protocol error: `lpush` expects integer reply")
    }
  }
}

#[test]
fn test_lpop_lpush() {
  let mut redis = Redis::connect();
  redis.lpush("some_key", "some_value");
  let popped = redis.lpop("some_key").expect("Failed to pop!");
  assert_eq!(popped, ~"some_value")
}

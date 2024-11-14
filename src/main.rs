use std::{net::TcpListener, process::exit};

// Use TLS to authenticate connections
/* 
  Resoureces:
    https://osamana.com/en/blog/securing-tcp-connections-with-ssltls/
    https://docs.rs/rustls/latest/rustls/

 */

fn main() {
  // Chat server:
  // TCP main server/broker - displays connected clients
  // Need encryption and authentication
  let listener = TcpListener::bind("127.0.0.1:10051");
  match listener {
    Ok(tcp_listener)  => {
      stream_loop(tcp_listener);
    },
    Err(err) => {
      println!("Failed to open tcp listener: {}", err);
      exit(1);
    }
  }
}

fn stream_loop(tcp_listener: TcpListener) {
  for stream in tcp_listener.incoming() {
    let stream = stream.unwrap();
    println!("Connection established with {}", stream.peer_addr().unwrap());
  }
}

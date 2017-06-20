#[macro_use]
extern crate clap;

use clap::App;
use std::io::prelude::*;
use std::fs::File;
use std::net::UdpSocket;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

const BUFFER_SIZE: usize = 188 * 7;

fn main() {

  let yaml = load_yaml!("cli.yml");
  let matches = App::from_yaml(yaml).get_matches();

  let port = matches.value_of("port").unwrap();
  let host = matches.value_of("host").unwrap();
  let server_ip = matches.value_of("server-ip").unwrap();
  let server_port = matches.value_of("server-port").unwrap();
  let source_file = matches.value_of("file").unwrap();
  
  let loop_enabled = matches.is_present("loop");
  let bitrate = value_t!(matches.value_of("bitrate"), u64).expect("unable to parse MpegTS bitrate");

  println!("From    : {}:{}", server_ip, server_port);
  println!("To      : {}:{}", host, port);
  println!("File    : {}", source_file);
  println!("Loop    : {}", loop_enabled);
  println!("Bitrate : {}", bitrate);

  let broadcast_ip = host.to_string() + ":" + port;
  let bind_ip = server_ip.to_string() + ":" + server_port;
  
  let socket = UdpSocket::bind(bind_ip).expect("couldn't bind to address");

  let mut file = File::open(source_file).expect("unable to open file");
  let _r = socket.set_broadcast(true);
  socket.connect(broadcast_ip).expect("connect function failed");

  let nano_sec_sleep = 665778; // 1 packet at 100Mbps
  let mut total_data_sended : u64 = 0;

  let start_time = Instant::now();

  loop {
    let elasped = start_time.elapsed();
    let diff = elasped.as_secs() * 1000000000 + elasped.subsec_nanos() as u64;

    while diff / 1000 * bitrate > total_data_sended * 1000000 {
      let mut buffer = [0; BUFFER_SIZE];
      let _size = file.read_exact(&mut buffer);
      let _res = socket.send(&buffer);
      total_data_sended += (BUFFER_SIZE * 8) as u64;
    }

    sleep(Duration::new(0, nano_sec_sleep as u32));
  }
}

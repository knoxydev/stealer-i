pub mod client_md
{
	use std::net::{TcpStream};
	use std::io::{Read, Write};

	pub fn start() {
		match TcpStream::connect("localhost:3333") {
			Ok(mut stream) =>
			{
				println!("Successfully connected to server in port 3333");

				let mut resp = String::from("Hello");
				let msg: &[u8] = resp.as_bytes();
				stream.write_all(msg).unwrap();

				let mut data = [0; 512];
				match stream.read(&mut data) {
					Ok(x) => { println!("reply: {:?}", String::from_utf8_lossy(&data[..x])); },
					Err(e) => { println!("failed to receive data: {}", e); }
				}
			},
			Err(e) => { println!("Failed to connect: {}", e); }
		}
		println!("Terminated.");
	}
}
pub mod client_md
{
	use std::net::{TcpStream};
	use std::io::{Read, Write};
	use std::str::from_utf8;

	pub fn start() {
		match TcpStream::connect("localhost:3333") {
			Ok(mut stream) => {
				println!("Successfully connected to server in port 3333");

				loop {
					let mut resp = String::from("Hello");
					let msg: &[u8] = resp.as_bytes();

					stream.write(msg).unwrap();
					//println!("Sent Hello, awaiting reply...");

					// let mut data = [0 as u8; 6]; // using 6 byte buffer
					// match stream.read_exact(&mut data)
					// {
					// 	Ok(_) => {
					// 		if &data == msg { println!("Reply is ok!"); }
					// 		else {
					// 			let text = from_utf8(&data).unwrap();
					// 			println!("Unexpected reply: {}", text);
					// 		}
					// 	},
					// 	Err(e) => { println!("Failed to receive data: {}", e); }
					// }
				}

			},
			Err(e) => { println!("Failed to connect: {}", e); }
		}
		println!("Terminated.");
	}
}
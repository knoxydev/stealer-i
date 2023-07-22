#![allow(warnings)]

pub mod server_mod
{
	use std::thread;
	use std::net::{TcpListener, TcpStream, Shutdown};
	use std::io::{Read, Write};

	fn handle_client(mut stream: TcpStream)
	{
		let mut data = [0; 512];
		match stream.read(&mut data) {
			Ok(x) =>
			{
				let rqt = String::from_utf8_lossy(&data[..x]);
				println!("{}", &rqt);

				stream.write("ok".as_bytes()).expect("Failed to write response");
			},
			Err(_) =>
			{
				println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
				stream.shutdown(Shutdown::Both).unwrap();
				
				return;
			}
		}
	}

	pub fn server_fn()
	{
		let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
		
		println!("Server listening on port 3333");
		for stream in listener.incoming()
		{
			match stream {
				Ok(stream) => {
					println!("New connection: {}", stream.peer_addr().unwrap());
					thread::spawn(move||
						{ handle_client(stream); });
				}
				Err(e) => { println!("Error: {}", e); }
			}
		}

		drop(listener); // connection closed
	}
}
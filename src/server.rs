pub mod server_mod
{
	use std::thread;
	use std::net::{TcpListener, TcpStream, Shutdown};
	use std::io::{Read, Write};
	use std::fs::File;
	use std::io::Result;


	fn handle_client(mut stream: TcpStream) -> Result<()>
	{
		let mut buffer = [0; 8];
		match stream.read_exact(&mut buffer) {
			Ok(()) =>
			{
				let file_size = u64::from_ne_bytes(buffer);
				let mut data = Vec::new();
				data.resize(file_size as usize, 0);

				match stream.read_exact(&mut data) {
					Ok(()) => {
						File::create("target/data.zip")?.write_all(&data)?;
						stream.write("ok".as_bytes()).expect("Failed to write response");
					},
					Err(_) => println!("Error while reading zip file data"),
				}
			}
			Err(_) => println!("Error while reading zip file size"),
		}

		Ok(())
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

		drop(listener);
	}
}
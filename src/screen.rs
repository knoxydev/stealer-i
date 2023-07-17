pub mod screen_md
{
	use screenshots::Screen;
	use std::{fs, time::Instant};


	pub fn start()
	{
		let screens = Screen::all().unwrap();

		for screen in screens
		{
			let mut image = screen.capture().unwrap();
			let mut buffer = image.to_png().unwrap();
			fs::write("target/screen-1.png", buffer).unwrap();
			//fs::write(format!("target/{}.png", screen.display_info.id), buffer).unwrap();
		}
	}
}
pub mod screen_md
{
	use screenshots::Screen;
	use std::io::Result;

	pub fn start() -> Result<Vec<u8>>
	{
		let screens = Screen::all().unwrap();
		let mut buffer: Vec<u8> = Vec::new();

		for screen in screens
		{
			let image = screen.capture().unwrap();
			buffer = image.to_png().unwrap();
		}

		Ok(buffer)
	}
}
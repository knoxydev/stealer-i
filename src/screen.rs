pub mod screen_md
{
	use screenshots::Screen;
	use std::fs;


	pub fn start()
	{
		let screens = Screen::all().unwrap();

		for screen in screens
		{
			let image = screen.capture().unwrap();
			let buffer = image.to_png().unwrap();
			fs::write("screen-1.png", buffer).unwrap();
		}
	}
}
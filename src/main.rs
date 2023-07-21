#![allow(warnings)]


// MODULES
mod sysinfo;

mod screen;
pub use crate::screen::screen_md;

mod bot;
pub use crate::bot::bot_md;

mod client;
pub use crate::client::client_md;

mod token;


fn main()
{
	println!("Hello, world!");

	screen_md::start();
	client_md::start();
}

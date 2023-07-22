#![allow(warnings)]


// MODULES
mod sysinfo;

mod screen;

mod bot;
pub use crate::bot::bot_md;

mod client;
pub use crate::client::client_md;

mod token;


fn main()
{
	println!("Hello, world!");

	client_md::start();
}

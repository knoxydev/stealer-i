#![allow(warnings)]


// MODULES
mod screen;
mod sysinfo;
mod session;
//mod server;

mod client;
pub use crate::client::client_md;


fn main()
{
	println!("Hello, world!");

	client_md::start(String::from("/sysinfo"));
	client_md::start(String::from("/screen"));
	client_md::start(String::from("/session"));
}

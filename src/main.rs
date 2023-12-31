#![allow(warnings)]


// MODULES
mod screen;
mod sysinfo;
mod session;

mod client;
pub use crate::client::client_md;

mod wifi;
pub use crate::wifi::wifi_md;


fn main()
{
	println!("Hello, world!");

	client_md::start(String::from("/wifi"));
	client_md::start(String::from("/sysinfo"));
	client_md::start(String::from("/screen"));
	client_md::start(String::from("/session"));
}

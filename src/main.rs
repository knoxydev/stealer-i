#![allow(warnings)]


// MODULES
mod screen;
mod sysinfo;
mod session;

mod client;
pub use crate::client::client_md;

mod zip;
pub use crate::zip::zip_md;


fn main()
{
	println!("Hello, world!");

	zip_md::start();
	client_md::start();
}

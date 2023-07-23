#![allow(warnings)]


// MODULES
mod sysinfo;
mod screen;

mod client;
pub use crate::client::client_md;

mod zip;
pub use crate::zip::zip_md;


// PACKAGES
use std::fs;


fn main()
{
	println!("Hello, world!");

	zip_md::start();
	client_md::start();
}

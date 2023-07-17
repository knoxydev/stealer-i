#![allow(warnings)]


// MODULES
mod sysinfo;
pub use crate::sysinfo::sysinfo_md;

mod screen;
pub use crate::screen::screen_md;


fn main()
{
	println!("Hello, world!");

	sysinfo_md::start();
	screen_md::start();
}

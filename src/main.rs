#![allow(warnings)]


// MODULES
mod sysinfo;
pub use crate::sysinfo::sysinfo_md;


fn main()
{
	println!("Hello, world!");

	sysinfo_md::start();
}

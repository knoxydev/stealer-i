pub mod sysinfo_md
{
	use sysinfo::{System, SystemExt};


	pub fn start() -> String
	{
		let mut sys = System::new_all();
		sys.refresh_all();

		let data: String = format!(
			"\nSystem name: {:?}\nSystem kernel version: {:?}\nSystem OS version: {:?}\nSystem host name: {:?}\n",
			sys.name().unwrap(),
			sys.kernel_version().unwrap(),
			sys.os_version().unwrap(),
			sys.host_name().unwrap());

		return data;
	}
}
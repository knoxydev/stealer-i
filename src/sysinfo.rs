pub mod sysinfo_md
{
	use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

	pub fn start()
	{
		let mut sys = System::new_all();
		sys.refresh_all();

		let data: String = format!(
			"\nSystem name: {:?}\nSystem kernel version: {:?}\nSystem OS version: {:?}\nSystem host name: {:?}\n",
			sys.name().unwrap(),
			sys.kernel_version().unwrap(),
			sys.os_version().unwrap(),
			sys.host_name().unwrap());

		println!("{}", data);
	}
}
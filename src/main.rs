use std::io::Error;
use std::env;

mod shared;
mod windows;

fn main() -> Result<(), Error> {
	let current_dir = get_current_dir()?;
    let dir = shared::get_watch_dir().unwrap_or(current_dir);

	#[cfg(windows)]
    let is_successful = windows::watch_directory(&dir);

	if is_successful.is_err() {
		println!("Failed! Error: {}", is_successful.unwrap_err());
	}

	Ok(())
}

fn get_current_dir() -> Result<String, Error> {
	let path = env::current_dir()?;
	Ok(path.into_os_string().into_string().unwrap())
}
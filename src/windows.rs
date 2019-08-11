#![cfg(windows)]

use std::ffi::CString;
use std::io;
use std::time::{ Instant };

use crate::shared;

extern crate winapi;
use winapi::um::fileapi::{ FindFirstChangeNotificationA, FindNextChangeNotification, FindCloseChangeNotification };
use winapi::um::winnt::{ FILE_NOTIFY_CHANGE_LAST_WRITE };
use winapi::um::handleapi::{ INVALID_HANDLE_VALUE };
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winbase::{ INFINITE, WAIT_OBJECT_0 };

#[allow(non_snake_case)]
pub fn watch_directory(dir: &str) -> Result<bool, io::Error> {
    const TRUE: i32 = 1;
    const FALSE: i32 = 0;
	const WAIT_TIMEOUT: u32 = 0x00000102;
    let mut last_updated: Instant = Instant::now();
    let c_string = CString::new(dir)?;              
	let lpPathName = c_string.as_ptr();

	unsafe {
        let change_handle = FindFirstChangeNotificationA(lpPathName, TRUE, FILE_NOTIFY_CHANGE_LAST_WRITE);
		if change_handle == INVALID_HANDLE_VALUE {
			println!("Invalid Handle value. Read last OS Error for more details.");
        	return Err(io::Error::last_os_error());
		}
		loop {
            println!("Waiting for changes in {}", dir);
			let dwWaitStatus = WaitForSingleObject(change_handle, INFINITE);
			match dwWaitStatus {
				WAIT_OBJECT_0 => {
                    check_last_updated(&mut last_updated);
                    if FindNextChangeNotification(change_handle) == FALSE {
                        return Err(io::Error::last_os_error());
                    }
				},
				WAIT_TIMEOUT => {
					println!("\nNo changes in the timeout period.\n");
					break;
				},
				_ => {
					println!("Unhandled dwWaitStatus. \n");
					FindCloseChangeNotification(change_handle);
					return Err(io::Error::last_os_error());
				}
			}
    	};
    };
    Ok(true)
}

fn check_last_updated(last_updated: &mut Instant) -> bool {
    //TIME BETWEEN NOTIFICATIONS
    const MIN_NOTIF_BREAK: u128 = 5000;
    let current_time = Instant::now();
    let time_passed = current_time.duration_since(*last_updated);
    //now we have the time passed, update previous last updated to new last updated
    if time_passed.as_millis() > MIN_NOTIF_BREAK {
        //update last updated time and current count
        *last_updated = current_time;
        shared::run_process();
        true
    }
    else {
        false
    }
}
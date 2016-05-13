// This file is part of sysctl. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sysctl/master/COPYRIGHT. No part of sysctl, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of sysctl. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sysctl/master/COPYRIGHT.


extern crate libc;
use self::libc::c_int;
use self::libc::c_void;
use self::libc::c_uint;
use self::libc::timeval;
use std::io::Error;
use std::io::Result;
use std::ptr;
use std::mem::size_of;
use std::mem::uninitialized;

// #[cfg(not(any(target_os = "linux", target_os = "android", target_os = "windows", target_os = "solaris")))]
// pub fn uptime() -> Result<timeval>
// {
// 	let boot_time = try!(boot_time());
//
// 	let mut now: timeval;
// 	unsafe
// 	{
// 		match self::libc::gettimeofday(now as *mut timeval, ptr::null_mut())
// 		{
// 			0 =>
// 			{
// 				Ok(timersub(now, boot_time))
// 			},
// 			-1 => Err(Error:last_os_error()),
// 		}
// 	}
// }



#[cfg(not(any(target_os = "linux", target_os = "android", target_os = "windows", target_os = "solaris")))]
pub fn maximum_number_of_processes() -> Result<c_int>
{
	sysctl(self::libc::CTL_KERN, self::libc::KERN_MAXPROC)
}

#[cfg(not(any(target_os = "linux", target_os = "android", target_os = "windows", target_os = "solaris")))]
pub fn boot_time() -> Result<timeval>
{
	sysctl(self::libc::CTL_KERN, self::libc::KERN_BOOTTIME)
}

#[cfg(not(any(target_os = "linux", target_os = "android", target_os = "windows", target_os = "solaris")))]
pub fn sysctl<T>(ctl: c_int, ctl_category: c_int) -> Result<T>
{
	let mut value: T = unsafe { uninitialized() };
	let mut mib: [i32; 2] = [ctl, ctl_category];
	let mut size = size_of::<T>();
	let pointer: *mut c_void = &mut value as *mut _ as *mut c_void;

	unsafe
	{
		match self::libc::sysctl(mib.as_mut_ptr() as *mut c_int, mib.len() as c_uint, pointer, &mut size as *mut usize, ptr::null_mut(), 0)
		{
			0 => Ok(value),
			-1 => Err(Error::last_os_error()),
			unexpected @ _ => panic!("Did not expect result code {}", unexpected),
		}
	}
}

#[cfg(not(any(target_os = "linux", target_os = "android", target_os = "windows", target_os = "solaris")))]
#[test]
fn test_maximum_number_of_processes()
{
	maximum_number_of_processes().unwrap();
}

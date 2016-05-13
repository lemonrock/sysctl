// This file is part of sysctl. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sysctl/master/COPYRIGHT. No part of sysctl, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of sysctl. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sysctl/master/COPYRIGHT.


#[macro_use] extern crate cfg_if;

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

cfg_if!
{
	if #[cfg(not(any(target_os = "linux", target_os = "android", target_os = "windows", target_os = "solaris")))]
	{
		mod bsd;
		pub use bsd::*;
	}
	else
	{
		// Unsupported
	}
}

// This file is part of sysctl. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sysctl/master/COPYRIGHT. No part of sysctl, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of sysctl. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sysctl/master/COPYRIGHT.


#[macro_use] extern crate cfg_if;
#[cfg(unix)] extern crate libc;
#[cfg(unix)] use libc::sysconf;
#[cfg(unix)] use libc::c_long;

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

#[cfg(unix)]
pub fn sysconf_number_of_processors_online() -> c_long
{
	sysconf(libc::_SC_NPROCESSORS_ONLN)
}

#[cfg(target_os = "macos")]
pub fn sysconf_number_of_processors_configured() -> c_long
{
	sysconf(libc::_SC_NPROCESSORS_CONF)
}

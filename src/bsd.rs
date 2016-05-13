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


#[allow(non_camel_case_types)]
// Clone and Debug can not be derived because we are inside cfg_if! and have nested #[cfg] statements
#[repr(i32)] // We'd like to use c_int here, but the compiler won't let us
enum Sysctl
{
	CTL_KERN = self::libc::CTL_KERN,
	CTL_VM = self::libc::CTL_VM,
	CTL_VFS = self::libc::CTL_VFS,
	CTL_NET = self::libc::CTL_NET,
	CTL_DEBUG = self::libc::CTL_DEBUG,
	CTL_HW = self::libc::CTL_HW,
	CTL_MACHDEP = self::libc::CTL_MACHDEP,
	#[cfg(any(target_os = "macos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd"))] CTL_USER = self::libc::CTL_USER,
	#[cfg(any(target_os = "dragonfly", target_os = "freebsd"))] CTL_P1003_1B = self::libc::CTL_P1003_1B,
	#[cfg(any(target_os = "dragonfly"))] CTL_LWKT = self::libc::CTL_LWKT,
	#[cfg(any(target_os = "bitrig", target_os = "openbsd"))] CTL_FS = self::libc::CTL_FS,
	#[cfg(any(target_os = "bitrig", target_os = "openbsd", target_os = "netbsd"))] CTL_DDB = self::libc::CTL_DDB,
	#[cfg(target_os = "netbsd")] CTL_PROC = self::libc::CTL_PROC,
	#[cfg(target_os = "netbsd")] CTL_VENDOR = self::libc::CTL_VENDOR,
	#[cfg(target_os = "netbsd")] CTL_EMUL = self::libc::CTL_EMUL,
	#[cfg(target_os = "netbsd")] CTL_SECURITY = self::libc::CTL_SECURITY,
}

impl Sysctl
{	
	pub fn sysctl<T>(self, ctl_category: c_int) -> Result<T>
	{
		let mut value: T = unsafe { uninitialized() };
		let mut mib: [i32; 2] = [self as i32, ctl_category];
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
}

pub fn maximum_number_of_processes() -> Result<c_int>
{
	Sysctl::CTL_KERN.sysctl(self::libc::KERN_MAXPROC)
}

pub fn boot_time() -> Result<timeval>
{
	Sysctl::CTL_KERN.sysctl(self::libc::KERN_BOOTTIME)
}

#[test]
fn test_maximum_number_of_processes()
{
	maximum_number_of_processes().unwrap();
}
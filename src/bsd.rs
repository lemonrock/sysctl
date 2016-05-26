// This file is part of sysctl. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sysctl/master/COPYRIGHT. No part of sysctl, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of sysctl. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sysctl/master/COPYRIGHT.


extern crate libc;
use self::libc::c_int;
use self::libc::c_void;
use self::libc::c_uint;
use self::libc::c_char;
use self::libc::int32_t;
use self::libc::size_t;
use self::libc::timeval;
use std::io::Error;
use std::io::Result;
use std::ptr::null_mut;
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
	pub fn get<T>(self, ctl_category: c_int) -> Result<T>
	{
		let mut value: T = unsafe { uninitialized() };
		let mut mib: [i32; 2] = [self as i32, ctl_category];
		let mut size = size_of::<T>();
		let pointer: *mut c_void = &mut value as *mut _ as *mut c_void;

		match unsafe { self::libc::sysctl(mib.as_mut_ptr() as *mut c_int, mib.len() as c_uint, pointer, &mut size as *mut usize, null_mut(), 0) }
		{
			0 => Ok(value),
			-1 => Err(Error::last_os_error()),
			unexpected @ _ => panic!("Did not expect result code {}", unexpected),
		}
	}
}

pub fn kern_maxproc() -> Result<c_int>
{
	Sysctl::CTL_KERN.get(self::libc::KERN_MAXPROC)
}

pub fn kern_boottime() -> Result<timeval>
{
	Sysctl::CTL_KERN.get(self::libc::KERN_BOOTTIME)
}

#[cfg(target_os = "macos")] static MachdepCpuCore_count: &'static [u8] = b"machdep.cpu.core_count\0";
#[cfg(target_os = "macos")]
pub fn machdep_cpu_core_count() -> Result<int32_t>
{
	sysctl_by_name(&MachdepCpuCore_count)
}

// The number of physical processors available in the current power management mode.
#[cfg(target_os = "macos")] static HwPhysicalcpu: &'static [u8] = b"hw.physicalcpu\0";
#[cfg(target_os = "macos")]
pub fn hw_physicalcpu() -> Result<int32_t>
{
	sysctl_by_name(&HwPhysicalcpu)
}

// The maximum number of physical processors that could be available this boot.
#[cfg(target_os = "macos")] static HwPhysicalcpu_max: &'static [u8] = b"hw.physicalcpu_max\0";
#[cfg(target_os = "macos")]
pub fn hw_physicalcpu_max() -> Result<int32_t>
{
	sysctl_by_name(&HwPhysicalcpu_max)
}

// The number of logical processors available in the current power management mode.
#[cfg(target_os = "macos")] static HwPhysicalcpu: &'static [u8] = b"hw.logicalcpu\0";
#[cfg(target_os = "macos")]
pub fn hw_logicalcpu() -> Result<int32_t>
{
	sysctl_by_name(&HwPhysicalcpu)
}

// The maximum number of logical processors that could be available this boot.
#[cfg(target_os = "macos")] static HwPhysicalcpu_max: &'static [u8] = b"hw.logicalcpu_max\0";
#[cfg(target_os = "macos")]
pub fn hw_logicalcpu_max() -> Result<int32_t>
{
	sysctl_by_name(&HwPhysicalcpu_max)
}

// The number of processor packages present on a machine (ie groups of cores)
#[cfg(target_os = "macos")] static HwPackages: &'static [u8] = b"hw.packages\0";
#[cfg(target_os = "macos")]
pub fn hw_packages() -> Result<int32_t>
{
	sysctl_by_name(&HwPackages)
}

#[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "dragonfly", target_os = "netbsd", target_os = "bitrig"))] static NwNcpu: &'static [u8] = b"hw.ncpu\0";
#[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "dragonfly", target_os = "netbsd", target_os = "bitrig"))]
/// Please note that this function is deprecated on Mac OS X
/// See also sysconf_number_of_processors_online and sysconf_number_of_processors_configured (latter is Mac OS X only)
pub fn hw_ncpu() -> Result<int32_t>
{
	sysctl_by_name(&NwNcpu)
}


// More names at https://www.freebsd.org/cgi/man.cgi?query=sysctl&apropos=0&sektion=8&manpath=FreeBSD+11-current&arch=default&format=html  and (apple) man sysctlbyname

/// Must end in '\0'
/// Not supported on OpenBSD, Solaris or Linux
#[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "dragonfly", target_os = "netbsd", target_os = "bitrig"))]
pub fn sysctl_by_name<T>(name: &'static [u8]) -> Result<T>
{
	let mut value: T = unsafe { uninitialized() };
	let mut size: size_t = size_of::<T>();
	let pointer: *mut c_void = &mut value as *mut _ as *mut c_void;
	
	match unsafe { self::libc::sysctlbyname(name.as_ptr() as *const c_char, pointer, &mut size as *mut usize, null_mut(), 0) }
	{
		0 => Ok(value),
		-1 => Err(Error::last_os_error()),
		unexpected @ _ => panic!("Did not expect result code {}", unexpected),
	}
}
					
#[test]
#[cfg(target_os = "macos")]
fn test_kern_maxproc()
{
	kern_maxproc().unwrap();
	machdep_cpu_core_count().unwrap();
}

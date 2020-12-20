extern crate winapi;

use std::mem;
use winapi::um::winnt::HANDLE;
use winapi::um::winnt::PAGE_EXECUTE_READWRITE;
use winapi::um::memoryapi::{
    VirtualProtectEx,
    ReadProcessMemory,
    WriteProcessMemory,
};

pub fn patch_instruction(destination: *mut u8, source: *mut u8, size: usize, handle: HANDLE) {
	unsafe {
		let mut old_protect;
		VirtualProtectEx(handle, destination as *mut winapi::ctypes::c_void, size, PAGE_EXECUTE_READWRITE, &mut old_protect);

		WriteProcessMemory(handle, destination as *mut winapi::ctypes::c_void, source as *mut winapi::ctypes::c_void, size, std::ptr::null_mut());
		VirtualProtectEx(handle, destination as *mut winapi::ctypes::c_void, size, old_protect, std::ptr::null_mut());
	}
}

/*
void dcs::mem::AssertEqual(BYTE* addr, const char* src, unsigned int size, HANDLE hProcess)
{
	DWORD oldProtect;
	BYTE* dst;

	VirtualProtectEx(hProcess, addr, size, PAGE_EXECUTE_READWRITE, &oldProtect);
	ReadProcessMemory(hProcess, addr, &dst, size, nullptr);
	VirtualProtectEx(hProcess, dst, size, oldProtect, nullptr);

	// if (memcmp(&dst, src, size)) throw std::exception("Memory assertion failed");
}
*/

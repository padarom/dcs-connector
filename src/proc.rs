extern crate winapi;

use std::mem;
use std::ffi::OsString;
use winapi::shared::minwindef::{MAX_PATH, HMODULE};
use std::os::windows::ffi::{OsStringExt};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::tlhelp32::{
	CreateToolhelp32Snapshot,
	Process32FirstW,
	Process32NextW,
	Module32FirstW,
	Module32NextW,
	TH32CS_SNAPPROCESS,
	TH32CS_SNAPMODULE,
	PROCESSENTRY32W,
	MODULEENTRY32W,
	MAX_MODULE_NAME32,
};

pub fn get_process_id(process_name: &str) -> Option<u32> {
    let snap_handle = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };

    if snap_handle == INVALID_HANDLE_VALUE {
        return None;
    }

    let mut process_entry : PROCESSENTRY32W = PROCESSENTRY32W {
        dwSize: mem::size_of::<PROCESSENTRY32W>() as u32,
        cntUsage: 0,
        th32ProcessID: 0,
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        cntThreads: 0,
        th32ParentProcessID: 0,
        pcPriClassBase: 0,
        dwFlags: 0,
        szExeFile: [0; MAX_PATH],
    };

    match unsafe { Process32FirstW(snap_handle, &mut process_entry) } {
        1 => {
            let mut process_success = 1i32;

            while process_success == 1 {
                match OsString::from_wide(&process_entry.szExeFile).to_str() {
                    Some(s) => {
                        if s.trim_matches(char::from(0)) == process_name {
                            unsafe { CloseHandle(snap_handle) };
                            return Some(process_entry.th32ProcessID);
                        }
                    },
                    None => {
                        println!("Error converting process name for PID {}", process_entry.th32ProcessID);
                    }
                }

                process_success = unsafe { Process32NextW(snap_handle, &mut process_entry) };
            }

            unsafe { CloseHandle(snap_handle) };
            None
        },
        0|_ => {
            unsafe { CloseHandle(snap_handle) };
            None
        }
    }
}

pub fn get_module_base_address(process_id: u32, module_name: &str) -> Option<*mut u8> {
	let snap_handle = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, process_id) };

	if snap_handle == INVALID_HANDLE_VALUE {
		return None;
	}

	let mut module_entry = MODULEENTRY32W {
		dwSize: mem::size_of::<MODULEENTRY32W>() as u32,
		th32ModuleID: 0,
		th32ProcessID: 0,
		GlblcntUsage: 0,
		ProccntUsage: 0,
		modBaseAddr: &mut 0u8 as *mut u8,
		modBaseSize: 0,
		hModule: 0 as HMODULE,
		szModule: [0; MAX_MODULE_NAME32 + 1],
		szExePath: [0; MAX_PATH],
	};

	match unsafe { Module32FirstW(snap_handle, &mut module_entry) } {
		1 => {
            let mut process_success = 1_i32;

            while process_success == 1 {
                match OsString::from_wide(&module_entry.szModule).to_str() {
                    Some(s) => {
                        if s.trim_matches(char::from(0)) == module_name {
                            unsafe { CloseHandle(snap_handle) };
                            return Some(module_entry.modBaseAddr);
                        }
                    },
                    None => {
                        println!("Error converting process name for PID {}", module_entry.th32ProcessID);
                    }
                }

                process_success = unsafe { Module32NextW(snap_handle, &mut module_entry) };
			}
			
            unsafe { CloseHandle(snap_handle) };
			None
		},
		0|_ => {
            unsafe { CloseHandle(snap_handle) };
			None
		}
	}
}

/*
uintptr_t dcs::proc::GetDMAAddress(HANDLE hProc, uintptr_t ptr, std::vector<unsigned int> offsets)
{
	uintptr_t addr = ptr;
	for (unsigned int i = 0; i < offsets.size(); i++)
	{
		ReadProcessMemory(hProc, (BYTE*)addr, &addr, sizeof(addr), nullptr);
		addr += offsets[i];
	}

	return addr;
}
*/
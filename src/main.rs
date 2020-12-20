extern crate winapi;
mod proc;
mod mem;

use winapi::um::winnt::PROCESS_ALL_ACCESS;
use winapi::um::processthreadsapi::OpenProcess;

fn main() {
    let process_name = "msedge.exe";

    let process_id = proc::get_process_id(process_name).unwrap();
    let module_base_address = proc::get_module_base_address(process_id, process_name).unwrap();

    let handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0i32, process_id) };
    mem::patch_instruction(module_base_address, module_base_address, 5, handle);
}
/*

dcs::Connector::~Connector()
{
	this->UnpatchMemoryOffsets();
	CloseHandle(this->hProcess);
}

bool dcs::Connector::ConnectToProcess(const wchar_t* procName)
{
	this->procName = procName;

	// Get the proc ID and base address of our target module
	DWORD procID = dcs::proc::GetProcID(procName);
	modBaseAddress = dcs::proc::GetModuleBaseAddress(procID, procName);

	// Obtain a process handle
	HANDLE hProcess = 0;
	hProcess = OpenProcess(PROCESS_ALL_ACCESS, NULL, procID);

	// Follow the pointer offset to store the vCamera struct pointer
	ReadProcessMemory(hProcess, (BYTE*)(modBaseAddress + 0xC22300), &this->vCameraPtr, sizeof(this->vCameraPtr), nullptr);

	// Patch the HMCS rendering instructions to use our injected camera rotation
	PatchMemoryOffsets();

	return true;
}

void dcs::Connector::PatchMemoryOffsets()
{
	BYTE* rollOffsetAddr = (BYTE*)(modBaseAddress  + 0x81d28d + 4);
	BYTE* pitchOffsetAddr = (BYTE*)(modBaseAddress + 0x81d295 + 4);
	BYTE* yawOffsetAddr = (BYTE*)(modBaseAddress   + 0x81d29d + 4);

	// Assert that we are indeed looking at the correct instructions before patching

	// 0xf2 0f10 9b 60010000		movsd xmm3,[rbx+00000160]		Roll value
	dcs::mem::AssertEqual(rollOffsetAddr, "\xf2\x0f\x10\x9b", 4, hProcess);
	// 0xf2 0f10 93 48010000		movsd xmm2,[rbx+00000148]		Pitch value
	dcs::mem::AssertEqual(pitchOffsetAddr, "\xf2\x0f\x10\x93", 4, hProcess);
	// 0xf2 0f10 8b 40010000		movsd xmm1,[rbx+00000140]		Yaw value
	dcs::mem::AssertEqual(yawOffsetAddr, "\xf2\x0f\x10\x8b", 4, hProcess);

	// Patch our custom offsets into the instructions
	dcs::mem::PatchInstruction(rollOffsetAddr, (BYTE*)"\xf0\x04", 2, hProcess);
	dcs::mem::PatchInstruction(pitchOffsetAddr, (BYTE*)"\xe8\x04", 2, hProcess);
	dcs::mem::PatchInstruction(yawOffsetAddr, (BYTE*)"\xe0\x04", 2, hProcess);
}

void dcs::Connector::UnpatchMemoryOffsets()
{
	BYTE* rollOffsetAddr = (BYTE*)(modBaseAddress  + 0x81d28d + 4);
	BYTE* pitchOffsetAddr = (BYTE*)(modBaseAddress + 0x81d295 + 4);
	BYTE* yawOffsetAddr = (BYTE*)(modBaseAddress   + 0x81d29d + 4);

	// Assert that we are indeed looking at the correct instructions before patching

	// 0xf2 0f10 9b f0040000		movsd xmm3,[rbx+000004f0]		Faked roll value
	dcs::mem::AssertEqual(rollOffsetAddr, "\xf2\x0f\x10\x9b", 4, hProcess);
	// 0xf2 0f10 93 e8040000		movsd xmm2,[rbx+000004e8]		Faked pitch value
	dcs::mem::AssertEqual(pitchOffsetAddr, "\xf2\x0f\x10\x93", 4, hProcess);
	// 0xf2 0f10 8b e0040000		movsd xmm1,[rbx+000004e0]		Faked yaw value
	dcs::mem::AssertEqual(yawOffsetAddr, "\xf2\x0f\x10\x8b", 4, hProcess);

	// Patch back the original offsets
	dcs::mem::PatchInstruction(rollOffsetAddr, (BYTE*)"\x60\x01", 2, hProcess);
	dcs::mem::PatchInstruction(pitchOffsetAddr, (BYTE*)"\x48\x01", 2, hProcess);
	dcs::mem::PatchInstruction(yawOffsetAddr, (BYTE*)"\x40\x01", 2, hProcess);
}

void dcs::Connector::WriteSensorData(double yaw, double pitch, double roll)
{
	WriteProcessMemory(hProcess, (BYTE*)(vCameraPtr + 0x04e0), &yaw, sizeof(yaw), nullptr);
	WriteProcessMemory(hProcess, (BYTE*)(vCameraPtr + 0x04e8), &pitch, sizeof(pitch), nullptr);
	WriteProcessMemory(hProcess, (BYTE*)(vCameraPtr + 0x04f0), &roll, sizeof(roll), nullptr);
}
*/
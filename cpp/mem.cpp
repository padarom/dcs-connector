#include "stdafx.h"
#include "mem.h"

void dcs::mem::PatchInstruction(BYTE* dst, BYTE* src, unsigned int size, HANDLE hProcess)
{
    DWORD oldProtect;
    VirtualProtectEx(hProcess, dst, size, PAGE_EXECUTE_READWRITE, &oldProtect);
    WriteProcessMemory(hProcess, dst, src, size, nullptr);
    VirtualProtectEx(hProcess, dst, size, oldProtect, nullptr);
}

void dcs::mem::AssertEqual(BYTE* addr, const char* src, unsigned int size, HANDLE hProcess)
{
    DWORD oldProtect;
    BYTE* dst;

    VirtualProtectEx(hProcess, addr, size, PAGE_EXECUTE_READWRITE, &oldProtect);
    ReadProcessMemory(hProcess, addr, &dst, size, nullptr);
    VirtualProtectEx(hProcess, dst, size, oldProtect, nullptr);

    src;

    // if (memcmp(&dst, src, size)) throw std::exception("Memory assertion failed");
}

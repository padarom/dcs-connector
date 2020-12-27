#pragma once

namespace dcs
{
    namespace proc
    {
        DWORD GetProcID(const TCHAR* procName);
        uintptr_t GetModuleBaseAddress(DWORD procID, const TCHAR* modName);
        uintptr_t GetDMAAddress(HANDLE hProc, uintptr_t ptr, std::vector<unsigned int> offsets);
    }
}

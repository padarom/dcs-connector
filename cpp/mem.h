#pragma once

namespace dcs
{
    namespace mem
    {
        void PatchInstruction(BYTE* dst, BYTE* src, unsigned int size, HANDLE hProcess);
        void AssertEqual(BYTE* dst, const char* src, unsigned int size, HANDLE hProcess);
    }
}

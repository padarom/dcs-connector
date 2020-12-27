#pragma once

namespace dcs
{
    class Connector
    {
        const TCHAR* procName;
        HANDLE hProcess;

        uintptr_t modBaseAddress;
        uintptr_t vCameraPtr;

    public:

        ~Connector();

        bool ConnectToProcess(const TCHAR* procName);
        void WriteSensorData(double yaw, double pitch, double roll);

    private:

        void PatchMemoryOffsets();
        void UnpatchMemoryOffsets();
    };
}

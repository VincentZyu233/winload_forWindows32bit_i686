#include <windows.h>

BOOL WINAPI GetFileInformationByHandleEx(
    HANDLE hFile,
    DWORD FileInformationClass,
    LPVOID lpFileInformation,
    DWORD dwBufferSize
) {
    HMODULE hKernel32 = GetModuleHandleW(L"kernel32.dll");
    if (hKernel32) {
        typedef BOOL (WINAPI *RealFunc)(HANDLE, DWORD, LPVOID, DWORD);
        RealFunc real = (RealFunc)GetProcAddress(hKernel32, "GetFileInformationByHandleEx");
        if (real)
            return real(hFile, FileInformationClass, lpFileInformation, dwBufferSize);
    }
    SetLastError(ERROR_INVALID_FUNCTION);
    return FALSE;
}

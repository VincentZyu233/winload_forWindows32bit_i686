#define WINAPI __stdcall
#define FALSE 0
#define TRUE 1
#define ERROR_INVALID_FUNCTION 1

typedef int BOOL;
typedef void *HANDLE;
typedef void *HMODULE;
typedef unsigned long DWORD;
typedef void *LPVOID;
typedef char CHAR;
typedef int (*FARPROC)();

__declspec(dllimport) HMODULE WINAPI GetModuleHandleA(const char *);
__declspec(dllimport) FARPROC WINAPI GetProcAddress(HMODULE, const char *);
__declspec(dllimport) void WINAPI SetLastError(DWORD);

BOOL WINAPI GetFileInformationByHandleEx(
    HANDLE hFile,
    DWORD FileInformationClass,
    LPVOID lpFileInformation,
    DWORD dwBufferSize
) {
    typedef BOOL (WINAPI *RealFunc)(HANDLE, DWORD, LPVOID, DWORD);
    HMODULE hKernel32 = GetModuleHandleA("kernel32.dll");
    if (hKernel32) {
        RealFunc real = (RealFunc)(void *)GetProcAddress(hKernel32, "GetFileInformationByHandleEx");
        if (real)
            return real(hFile, FileInformationClass, lpFileInformation, dwBufferSize);
    }
    SetLastError(ERROR_INVALID_FUNCTION);
    return FALSE;
}

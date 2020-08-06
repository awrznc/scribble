#include <windows.h>

int WINAPI WinMain(
    HINSTANCE hInstance,
    HINSTANCE hPrevInstance,
    LPSTR lpszCmdLine,
    int nCmdShow
) {
    MessageBox(NULL, TEXT("Hello, world!"), TEXT("Test"), MB_OK);

    return 0;
}

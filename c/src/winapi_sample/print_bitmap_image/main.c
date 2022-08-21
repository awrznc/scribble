// cl .\main.c gdi32.lib user32.lib /link /SUBSYSTEM:WINDOWS /entry:mainCRTStartup

#include <windows.h>

#define APP_NAME "print_bitmap"
#define APP_HIGHT 72

static char bytes[58] = {
    66, 77, 58, 0, 0, 0, 0, 0, 0, 0,
    54, 0, 0, 0, 40, 0, 0, 0, 1, 0,
    0, 0, 1, 0, 0, 0, 1, 0, 24, 0,
    0, 0, 0, 0, 4, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 39, 127, 255, 0
};

static LRESULT CALLBACK window_procedure(
    HWND hwnd,
    UINT uMsg,
    WPARAM wParam,
    LPARAM lParam
) {
    if (uMsg == WM_CREATE) {
        return 0;

    } else if(uMsg == WM_PAINT) {
                BITMAPFILEHEADER* bmfh;
        bmfh = (BITMAPFILEHEADER*)bytes;

        BITMAPINFOHEADER* bmih;
        bmih = (BITMAPINFOHEADER*)(bytes + sizeof(BITMAPFILEHEADER));
        BITMAPINFO* bmi;
        bmi = (BITMAPINFO*)bmih;

        void* bits;
        bits = (void*)(bytes + bmfh->bfOffBits);

        HDC hdc = GetDC(NULL);

        HBITMAP hbmp = CreateDIBitmap(hdc, bmih, CBM_INIT, bits, bmi, DIB_RGB_COLORS) ;

        ReleaseDC(NULL, hdc);

        PAINTSTRUCT ps;
        HWND paint_handle = BeginPaint(hwnd, &ps);
        HDC buffer = CreateCompatibleDC(paint_handle);
        SelectObject(buffer, hbmp);
        BitBlt(
                    paint_handle, 0, 0,
                    1,
                    1,
                    buffer , 0 , 0 , SRCCOPY );
        EndPaint(hwnd, &ps);
        return 0;

    } else if(uMsg == WM_TIMER) {
        return 0;

    } else if(uMsg == WM_DESTROY) {
        KillTimer(hwnd, 1);
        PostQuitMessage(0);
        return 0;
    }

    return DefWindowProc(hwnd, uMsg, wParam, lParam);
}

int main() {
    HINSTANCE instance_handle = (HINSTANCE)GetModuleHandle(0);

    WNDCLASSEX window_class_ex = { 0 };
    window_class_ex.cbSize        = sizeof(WNDCLASSEX);
    window_class_ex.style         = CS_HREDRAW | CS_VREDRAW;
    window_class_ex.lpfnWndProc   = window_procedure;
    window_class_ex.cbClsExtra    = 0;
    window_class_ex.cbWndExtra    = 0;
    window_class_ex.hInstance     = instance_handle;
    window_class_ex.hIcon         = (HICON)LoadImage(NULL, IDI_APPLICATION, IMAGE_ICON, 0, 0, LR_SHARED);
    window_class_ex.hCursor       = (HCURSOR)LoadImage(NULL, IDC_ARROW, IMAGE_CURSOR, 0, 0, LR_SHARED);
    window_class_ex.hbrBackground = CreateSolidBrush( RGB( 0xFE, 0xFE, 0xFE ) );
    window_class_ex.lpszMenuName  = NULL;
    window_class_ex.lpszClassName = APP_NAME;
    window_class_ex.hIconSm       = (HICON)LoadImage(NULL, IDI_APPLICATION, IMAGE_ICON, 0, 0, LR_SHARED);

    if (RegisterClassEx(&window_class_ex) == 0) return 0;

    unsigned int app_width = 400;
    unsigned int app_height = 200;
    unsigned int app_position_x = 100;
    unsigned int app_position_y = 100;
    HWND window_handler = CreateWindowEx(
        0,
        APP_NAME, APP_NAME, WS_SYSMENU | WS_MINIMIZEBOX,
        app_position_x, app_position_y,
        app_width, app_height,
        NULL, NULL, instance_handle, NULL
    );

    if (window_handler == NULL) return 0;

    int cmd_show = SW_RESTORE;
    ShowWindow(window_handler, cmd_show);
    UpdateWindow(window_handler);

    MSG message = { 0 };
    while (GetMessage(&message, NULL, 0, 0) > 0) {
        TranslateMessage(&message);
        DispatchMessage(&message);
    }

    return (int)message.wParam;
}

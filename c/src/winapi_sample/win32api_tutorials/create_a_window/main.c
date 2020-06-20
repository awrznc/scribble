#include <windows.h>

// #define WINDOW_CLASS_NAME "myWindowClass"
#define WINDOW_NAME L"My Windows"
#define WINDOW_X_SIZE 500
#define WINDOW_Y_SIZE 500


LRESULT CALLBACK WindowsProcedure(
    HWND hWnd,
    UINT msg,
    WPARAM wp,
    LPARAM lp
) {
    switch (msg) {
        case WM_DESTROY: PostQuitMessage; break;
        default: DefWindowProc(hWnd, msg, wp, lp);
    }
}

/* main */
int WINAPI WinMain (
    HINSTANCE hInst,
    HINSTANCE hPrevInst,
    LPSTR args,
    int ncmdshow
) {
    // メッセージボックスの表示
    // MessageBox(NULL, "HELLO", "My first GUI", MB_OK);

    WNDCLASS window_class = {0};

    window_class.hbrBackground = (HBRUSH)COLOR_WINDOW;  // 背景色
    window_class.hCursor = LoadCursor(NULL, IDC_ARROW); // ウィンドウ内にカーソルを表示できるように設定
    window_class.hInstance = hInst;                     // インスタンスハンドル
    window_class.lpszClassName = "myWindowClass";     // クラス名
    window_class.lpfnWndProc = WindowsProcedure;        // プロージャ名

    if ( !RegisterClass(&window_class))
        return -1;

    // ウインドウの作成
    CreateWindowW(
        L"myWindowClass",
        WINDOW_NAME,
        WS_OVERLAPPEDWINDOW | WS_VISIBLE,
        100,                                // 表示位置のX座標
        100,                                // 表示位置のY座標
        WINDOW_X_SIZE,                      // 幅
        WINDOW_Y_SIZE,                      // 高さ
        NULL,                               // 親ウインドウのハンドラ
        NULL,                               // メニューのハンドラ
        NULL,                               // ウインドウクラスのインスタンス（再度使う必要がない場合はNULL）
        NULL
    );

    MSG message = {0};

    // GetMessage でメッセージを返せる
    while ( GetMessage(&message, NULL, 0, 0) ) {
        TranslateMessage(&message);
        DispatchMessage(&message);
    }

    return 0;
}


// #include <windows.h>

// LRESULT CALLBACK WndProc(
//     HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam);

// int WINAPI WinMain(
//     HINSTANCE hInstance, HINSTANCE hPrevInstance,
//     LPSTR lpszCmdLine, int nCmdShow)
// {
//     TCHAR szAppName[] = TEXT("TestApp");
//     WNDCLASS wc;
//     HWND hwnd;
//     MSG msg;

//     wc.style         = CS_HREDRAW | CS_VREDRAW;
//     wc.lpfnWndProc   = WndProc;
//     wc.cbClsExtra    = 0;
//     wc.cbWndExtra    = 0;
//     wc.hInstance     = hInstance;
//     wc.hIcon         = LoadIcon(NULL, IDI_APPLICATION);
//     wc.hCursor       = LoadCursor(NULL, IDC_ARROW);
//     wc.hbrBackground = (HBRUSH) (COLOR_WINDOW + 1);
//     wc.lpszMenuName  = NULL;
//     wc.lpszClassName = szAppName;

//     if (!RegisterClass(&wc)) return 0;

//     hwnd = CreateWindow(
//         szAppName, TEXT(""),
//         WS_OVERLAPPEDWINDOW,
//         CW_USEDEFAULT, CW_USEDEFAULT,
//         CW_USEDEFAULT, CW_USEDEFAULT,
//         NULL, NULL,
//         hInstance, NULL);

//     if (!hwnd) return 0;

//     ShowWindow(hwnd, nCmdShow);
//     UpdateWindow(hwnd);

//     while (GetMessage(&msg, NULL, 0, 0) > 0) {
//         TranslateMessage(&msg);
//         DispatchMessage(&msg);
//     }

//     return msg.wParam;
// }

// LRESULT CALLBACK WndProc(
//     HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam)
// {
//     HDC hdc;
//     PAINTSTRUCT ps;

//     switch (uMsg) {
//     case WM_PAINT:
//         hdc = BeginPaint(hwnd, &ps);       // 描画開始
//         Rectangle(hdc, 50, 50, 200, 150);  // 描画
//         EndPaint(hwnd, &ps);               // 描画終了
//         return 0;

//     case WM_DESTROY:
//         PostQuitMessage(0);
//         return 0;
//     }

//     return DefWindowProc(hwnd, uMsg, wParam, lParam);
// }

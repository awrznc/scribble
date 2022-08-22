use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::{UI::WindowsAndMessaging::*, Graphics::Gdi::{UpdateWindow, CreateSolidBrush}}
};


fn main() -> Result<()> {
    let instance = unsafe { GetModuleHandleA(None)? };
    assert!(instance.0 != 0);

    let window_class = s!("window");

    unsafe {
        let wc = WNDCLASSA {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            hInstance: instance,
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hbrBackground: CreateSolidBrush(0x00FFFFFF),
            lpszClassName: window_class,
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);
        assert!(atom != 0);

        let window_handler = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class, s!("main"),
            WS_SYSMENU | WS_MINIMIZEBOX | WS_VISIBLE,
            200, 200, 200, 200,
            None,
            None,
            instance,
            core::ptr::null()
        );
        let window_handler2: HWND = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class, s!("sub"),
            WS_POPUP,
            300, 300, 300, 300,
            window_handler,
            None,
            instance,
            core::ptr::null()
        );

        let mut message = MSG::default();

        let cmd_show = SW_RESTORE;
        ShowWindow(window_handler, cmd_show);
        UpdateWindow(window_handler);
        ShowWindow(window_handler2, SW_HIDE);
        UpdateWindow(window_handler2);

        while GetMessageA(&mut message, HWND(0), 0, 0).into() {
            TranslateMessage(&message);
            DispatchMessageA(&message);
        }

        Ok(())
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match message {
        WM_MOVE => {

            let mut window_text = [0; 8];
            let text_size = unsafe { GetWindowTextA( window, &mut window_text ) };

            if text_size == 4 && &window_text[0..4] == b"main" {
                let mut window_info: WINDOWINFO = Default::default();
                unsafe { GetWindowInfo(window, &mut window_info) };
                let x: i32 = window_info.rcWindow.left;
                let y: i32 = window_info.rcWindow.top;
                println!("{}, {}", x, y);

                let sub_window: HWND = unsafe { FindWindowA( s!("window"), s!("sub") ) };
                unsafe { MoveWindow(sub_window, x+100, y+100, 200, 200, true) };
                unsafe { ShowWindow(sub_window, SW_RESTORE) };
            }

            LRESULT(0)
        }
        WM_DESTROY => {
            println!("WM_DESTROY");
            unsafe {
                KillTimer(window, 1);
                PostQuitMessage(0)
            };
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcA(window, message, wparam, lparam) },
    }
}


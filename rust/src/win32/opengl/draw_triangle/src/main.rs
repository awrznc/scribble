// #![windows_subsystem = "windows"]

use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::*, Win32::Graphics::OpenGL::*,
    Win32::System::LibraryLoader::GetModuleHandleA, Win32::UI::WindowsAndMessaging::*,
};

mod opengl;

fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleA(None).unwrap();

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
            hInstance: instance.into(),
            lpszClassName: PCSTR(b"window\0".as_ptr()),

            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        let hwnd = CreateWindowExA(
            Default::default(),
            s!("window"),
            s!("Hello, World!"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            640,
            480,
            None,
            None,
            instance,
            None,
        );

        let hdc = GetDC(hwnd);
        let (hrc, mut ex) = enable_open_gl(hdc);

        // init
        ex.new_shader();

        loop {
            let mut message = MSG::default();

            if PeekMessageA(&mut message, None, 0, 0, PM_REMOVE).into() {
                TranslateMessage(&message);
                DispatchMessageA(&message);

                if message.message == WM_QUIT {
                    break;
                }
            } else {
                // draw_triangle();
                ex.draw_triangle();

                SwapBuffers(hdc).unwrap();
            }
        }

        disable_open_gl(hwnd, hdc, hrc);
        DestroyWindow(hwnd).unwrap();

        Ok(())
    }
}

fn enable_open_gl(hdc: HDC) -> (HGLRC, opengl::Extend) {
    unsafe {
        let pfd = PIXELFORMATDESCRIPTOR {
            nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
            nVersion: 1,
            dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
            iPixelType: PFD_TYPE_RGBA,
            cColorBits: 24,
            cDepthBits: 16,
            // cStencilBits: 8,
            iLayerType: PFD_MAIN_PLANE.0 as u8,
            ..Default::default()
        };

        let i_format = ChoosePixelFormat(hdc, &pfd);

        SetPixelFormat(hdc, i_format, &pfd).unwrap();

        let old_hrc = wglCreateContext(hdc).unwrap();
        wglMakeCurrent(hdc, old_hrc).unwrap();

        const WGL_CONTEXT_MAJOR_VERSION_ARB: i32 = 0x2091;
        const WGL_CONTEXT_MINOR_VERSION_ARB: i32 = 0x2092;
        const WGL_CONTEXT_FLAGS_ARB: i32 = 0x2094;
        // const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: i32 = 0x00000001;
        const WGL_CONTEXT_ES2_PROFILE_BIT_EXT: i32 = 0x00000004;
        const WGL_CONTEXT_PROFILE_MASK_ARB: i32 = 0x9126;
        let version: [i32; 9] = [
            WGL_CONTEXT_MAJOR_VERSION_ARB,
            3,
            WGL_CONTEXT_MINOR_VERSION_ARB,
            2,
            WGL_CONTEXT_FLAGS_ARB,
            0,
            WGL_CONTEXT_PROFILE_MASK_ARB,
            WGL_CONTEXT_ES2_PROFILE_BIT_EXT,
            0,
        ];

        let ex = opengl::Extend::new();
        let slice: &[i32] = &version;
        let rawptr: *const i32 = &slice[0] as *const i32;
        let hrc = (ex.wglCreateContextAttribsARB)(hdc, HGLRC(0), rawptr);
        // let hrc = (ex.wglCreateContextAttribsARB)(hdc, HGLRC(0), std::ptr::null());
        dbg!(hrc);
        wglMakeCurrent(hdc, hrc).unwrap();
        wglDeleteContext(old_hrc).unwrap();

        (hrc, ex)
    }
}

fn disable_open_gl(hwnd: HWND, hdc: HDC, hrc: HGLRC) {
    unsafe {
        wglMakeCurrent(None, None).unwrap();
        wglDeleteContext(hrc).unwrap();
        ReleaseDC(hwnd, hdc);
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            },
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}

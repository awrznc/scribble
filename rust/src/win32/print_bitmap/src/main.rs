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
            WS_EX_LAYERED,
            window_class, s!("main"),
            WS_SYSMENU | WS_VISIBLE,
            200, 200, 200, 200,
            None,
            None,
            instance,
            core::ptr::null()
        );

        let mut message = MSG::default();

        let cmd_show = SW_RESTORE;
        ShowWindow(window_handler, cmd_show);
        UpdateWindow(window_handler);

        while GetMessageA(&mut message, HWND(0), 0, 0).into() {
            TranslateMessage(&message);
            DispatchMessageA(&message);
        }

        Ok(())
    }
}

use windows::Win32::Graphics::Gdi::*;

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {

    match message {
        WM_CREATE => {
            // make transparent
            unsafe {
                SetLayeredWindowAttributes(window, 0x00FFFFFF, 0xFF, LWA_COLORKEY | LWA_ALPHA);
            }
            LRESULT(0)
        }
        WM_PAINT => {
            println!("WM_PAINT");
            unsafe {

                // === static load ===
                let raw_bmp = include_bytes!("data.bmp");
                let ptr = raw_bmp.as_ptr();
                let bmp_file_header: *const BITMAPFILEHEADER = ptr as *const BITMAPFILEHEADER;
                let bmp_info_header: *const BITMAPINFOHEADER = (ptr as usize + std::mem::size_of::<BITMAPFILEHEADER>()) as *const BITMAPINFOHEADER;
                let bmp_info: *const BITMAPINFO = bmp_info_header as *const BITMAPINFO;
                // println!(
                //     "{}, {}, {}, {}",
                //     (*bmp_info).bmiHeader.biBitCount,
                //     (*bmp_info).bmiHeader.biWidth,
                //     (*bmp_info).bmiHeader.biHeight,
                //     (*bmp_file_header).bfOffBits as u32
                // );

                let bits: *const core::ffi::c_void = (
                    ptr.wrapping_offset((*bmp_file_header).bfOffBits as isize)
                ) as *const _;
                let dc = GetDC(None);
                let bitmap_handler: HBITMAP = CreateDIBitmap(
                    dc,
                    bmp_info_header,
                    CBM_INIT as u32,
                    bits,
                    bmp_info,
                    DIB_RGB_COLORS);
                assert!(bitmap_handler.0 != 0);
                ReleaseDC(window , dc);

                let mut ps: PAINTSTRUCT = PAINTSTRUCT::default();
                let hdc = BeginPaint(window, &mut ps);
                let buffer = CreateCompatibleDC(hdc);
                let gdi_object: HGDIOBJ = SelectObject(buffer, bitmap_handler);
                assert!(!gdi_object.is_invalid());
                let result = BitBlt(
                    hdc, 0, 0,
                    (*bmp_info).bmiHeader.biWidth,
                    (*bmp_info).bmiHeader.biHeight,
                    buffer , 0 , 0 , SRCCOPY );
                assert!(result.0 != 0);
                EndPaint(window, &mut ps);

                // // === dynamic load ===
                // let image_handle: HANDLE = LoadImageA(
                //     None, s!(".\\src\\data.bmp"), IMAGE_BITMAP, 0, 0,
                //     LR_LOADFROMFILE | LR_CREATEDIBSECTION).expect("err");

                // let bitmap_handler_p: *const HBITMAP = (&image_handle) as *const _ as _;
                // let bitmap_handler: HBITMAP = *bitmap_handler_p;
                // // let bitmap_handler: HBITMAP = HBITMAP::default();

                // let mut ps: PAINTSTRUCT = PAINTSTRUCT::default();
                // let hdc = BeginPaint(window, &mut ps);
                // let buffer = CreateCompatibleDC(hdc);
                // SelectObject(buffer, bitmap_handler);

                // BitBlt(hdc , 0 , 0 , 188 , 77 , buffer , 0 , 0 , SRCCOPY);

                // DeleteDC(buffer);
                // EndPaint(window, &mut ps);
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

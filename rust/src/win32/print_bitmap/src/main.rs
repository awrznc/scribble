use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::{
        Graphics::Gdi::{CreateSolidBrush, UpdateWindow},
        UI::WindowsAndMessaging::*,
    },
};

fn main() -> Result<()> {
    let instance = unsafe { GetModuleHandleA(None)? };
    assert!(instance.0 != 0);

    let window_class = s!("window");

    unsafe {
        let wc = WNDCLASSA {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            hInstance: HINSTANCE::from(instance),
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hbrBackground: CreateSolidBrush(COLORREF(0x00FFFFFF)),
            lpszClassName: window_class,
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);
        assert!(atom != 0);

        let window_handler = CreateWindowExA(
            WS_EX_LAYERED,
            window_class,
            s!("main"),
            WS_SYSMENU | WS_VISIBLE,
            200,
            200,
            320 * 4,
            180 * 4,
            None,
            None,
            instance,
            None,
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

static mut BITMAP_HANDLER: HBITMAP = unsafe { core::mem::zeroed() };
static mut BITMAP_INFO: BITMAPINFO = unsafe { core::mem::zeroed() };
static mut DRAWER: Drawer = unsafe { core::mem::zeroed() };
static mut DIB: [u32; 320 * 180] = [0x00FF0000u32; 320 * 180];

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match message {
        WM_CREATE => {
            unsafe {
                BITMAP_HANDLER = get_image();
                DRAWER = Drawer::init(window);
            }
            // make transparent
            unsafe {
                SetLayeredWindowAttributes(
                    window,
                    COLORREF(0x00FFFFFF),
                    0xFF,
                    LWA_COLORKEY | LWA_ALPHA,
                )
                .unwrap();
            }
            LRESULT(0)
        }
        WM_PAINT => {
            let result = measuring_performance::measuring(
                || {
                    unsafe { DRAWER.draw(window) };
                },
                // 1000回ループ
                1000,
            );
            measuring_performance::print(&result);
            LRESULT(0)
        }
        WM_DESTROY => {
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcA(window, message, wparam, lparam) },
    }
}

// ======== ======== ======== ========
// 読み込み方法
// ======== ======== ======== ========

#[cfg(feature = "static_load")]
fn get_image() -> HBITMAP {
    let raw_bmp = include_bytes!("./data.bmp");
    let ptr = raw_bmp.as_ptr();
    let bmp_file_header: *const BITMAPFILEHEADER = ptr as *const BITMAPFILEHEADER;
    let bmp_info_header: *const BITMAPINFOHEADER =
        (ptr as usize + std::mem::size_of::<BITMAPFILEHEADER>()) as *const BITMAPINFOHEADER;
    let bmp_info: *const BITMAPINFO = bmp_info_header as *const BITMAPINFO;
    unsafe {
        BITMAP_INFO = *bmp_info;
    }
    // unsafe {
    //     println!(
    //         "{}, {}, {}, {}",
    //         (*bmp_info).bmiHeader.biBitCount,
    //         (*bmp_info).bmiHeader.biWidth,
    //         (*bmp_info).bmiHeader.biHeight,
    //         (*bmp_file_header).bfOffBits as u32
    //     );
    // }

    unsafe {
        let bits: *const core::ffi::c_void =
            (ptr.wrapping_offset((*bmp_file_header).bfOffBits as isize)) as *const _;
        let dc = GetDC(None);
        let bitmap_handler: HBITMAP = CreateDIBitmap(
            dc,
            Some(bmp_info_header),
            CBM_INIT as u32,
            Some(bits),
            Some(bmp_info),
            DIB_RGB_COLORS,
        );
        assert!(bitmap_handler.0 != 0);

        bitmap_handler
    }
}

#[cfg(feature = "dynamic_load")]
fn get_image() -> HBITMAP {
    unsafe {
        let image_handle: HANDLE = LoadImageA(
            None,
            s!(".\\src\\data.bmp"),
            IMAGE_BITMAP,
            0,
            0,
            LR_LOADFROMFILE | LR_CREATEDIBSECTION,
        )
        .expect("err");

        let bitmap_handler_p: *const HBITMAP = (&image_handle) as *const _ as _;
        *bitmap_handler_p
    }
}

// ======== ======== ======== ========
// 描画対象の扱い
// ======== ======== ======== ========

struct DC {
    hdc: HDC,
    window: HWND,
}

impl DC {
    pub fn new(window: HWND) -> Self {
        Self {
            hdc: unsafe { GetDC(window) },
            window,
        }
    }
}

impl Drop for DC {
    fn drop(&mut self) {
        unsafe {
            ReleaseDC(self.window, self.hdc);
        }
    }
}

struct Drawer {
    buffer: HDC,
}

impl Drawer {
    #[cfg(feature = "ddb")]
    pub fn init(window: HWND) -> Self {
        let hdc = DC::new(window);
        let buffer = unsafe { CreateCompatibleDC(hdc.hdc) };
        let gdi_object: HGDIOBJ = unsafe { SelectObject(buffer, BITMAP_HANDLER) };
        assert!(!gdi_object.is_invalid());
        Self { buffer }
    }

    #[cfg(feature = "dib")]
    pub fn init(window: HWND) -> Self {
        let hdc = DC::new(window);
        let buffer = unsafe { CreateCompatibleDC(hdc.hdc) };
        let gdi_object: HGDIOBJ = unsafe { SelectObject(buffer, BITMAP_HANDLER) };
        assert!(!gdi_object.is_invalid());
        Self { buffer }
    }

    #[cfg(feature = "bit_blt")]
    fn draw(&self, window: HWND) {
        unsafe {
            let mut ps: PAINTSTRUCT = PAINTSTRUCT::default();
            let hdc = BeginPaint(window, &mut ps);
            let result = BitBlt(hdc, 0, 0, 320, 180, self.buffer, 0, 0, SRCCOPY);
            assert!(result.is_ok());
            EndPaint(window, &ps);
        }
    }

    #[cfg(feature = "stretch_blt")]
    fn draw(&self, window: HWND) {
        unsafe {
            let mut ps: PAINTSTRUCT = PAINTSTRUCT::default();
            let hdc = BeginPaint(window, &mut ps);
            let result = StretchBlt(hdc, 0, 0, 1280, 720, self.buffer, 0, 0, 320, 180, SRCCOPY);
            assert!(result == TRUE);
            EndPaint(window, &ps);
        }
    }

    #[cfg(feature = "stretch_di_bits")]
    fn draw(&self, window: HWND) {
        let bmp_header = BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: 320,
            biHeight: -(180),
            // biHeight: height,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: 0,

            // 適当
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrImportant: 0,
            biClrUsed: 0,
        };
        let bmp_info = BITMAPINFO {
            bmiHeader: bmp_header,

            // 適当
            bmiColors: [RGBQUAD::default()],
        };
        unsafe {
            let mut ps: PAINTSTRUCT = PAINTSTRUCT::default();
            let hdc = BeginPaint(window, &mut ps);
            let _ = StretchDIBits(
                hdc,
                0,
                0,
                1280,
                720,
                0,
                0,
                320,
                180,
                Some(DIB.as_ptr() as _),
                &bmp_info,
                DIB_RGB_COLORS,
                SRCCOPY,
            );
            // assert!(result.is_ok());
            EndPaint(window, &ps);
        }
    }
}

impl Drop for Drawer {
    fn drop(&mut self) {
        unsafe {
            DeleteDC(self.buffer);
        }
    }
}

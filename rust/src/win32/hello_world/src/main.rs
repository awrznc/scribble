fn main() {
    let title = std::ffi::CString::new("Title!").unwrap();
    let message = std::ffi::CString::new("Hello, World!").unwrap();
    unsafe {
        windows::Win32::UI::WindowsAndMessaging::MessageBoxA(
            windows::Win32::Foundation::HWND(0),
            windows::core::PCSTR(message.as_ptr() as *const u8),
            windows::core::PCSTR(title.as_ptr() as *const u8),
            windows::Win32::UI::WindowsAndMessaging::MB_OK,
        );
    }
}

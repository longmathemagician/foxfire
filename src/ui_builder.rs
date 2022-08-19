use druid::{Widget, WindowId};

use crate::app_state::*;
use crate::container::*;

#[cfg(windows)]
mod win_api_calls {
    use std::ffi::{c_void, OsStr};
    use std::iter::once;
    use std::os::windows::prelude::OsStrExt;

    use winapi::shared::minwindef::BOOL;
    use winapi::shared::minwindef::DWORD;
    use winapi::shared::minwindef::FALSE;
    use winapi::shared::minwindef::TRUE;
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winapi::um::processthreadsapi::GetCurrentProcessId;
    use winapi::um::winnt::LPCWSTR;
    use winapi::um::winuser::EnumWindows;
    use winapi::um::winuser::GetWindowThreadProcessId;
    use winapi::{
        shared::minwindef::{LPARAM, LPCVOID, WPARAM},
        shared::ntdef::HRESULT,
        shared::windef::HWND,
        um::winuser::{LoadIconW, SendMessageW, ICON_BIG, ICON_SMALL, WM_SETICON},
    };

    extern "system" {
        pub fn DwmSetWindowAttribute(
            h_wnd: HWND,
            dw_attribute: DWORD,
            pv_attribute: LPCVOID,
            cb_attribute: DWORD,
        ) -> HRESULT;
    }

    pub unsafe fn set_icon() {
        extern "system" fn callback(h_wnd: HWND, _l_param: LPARAM) -> BOOL {
            let mut process_id: DWORD = 0;
            unsafe {
                GetWindowThreadProcessId(h_wnd, &mut process_id as *mut DWORD);
                if GetCurrentProcessId() != process_id {
                    return TRUE;
                }
                let icon_name: Vec<u16> = OsStr::new("application_icon")
                    .encode_wide()
                    .chain(once(0))
                    .collect();
                let hicon = LoadIconW(GetModuleHandleW(0 as LPCWSTR), icon_name.as_ptr());
                SendMessageW(h_wnd, WM_SETICON, ICON_SMALL as WPARAM, hicon as LPARAM);
                SendMessageW(h_wnd, WM_SETICON, ICON_BIG as WPARAM, hicon as LPARAM);
            }
            FALSE
        }

        EnumWindows(Some(callback), 0);
    }

    pub unsafe fn set_mica_effect() {
        extern "system" fn callback(hwnd: HWND, _l_param: LPARAM) -> BOOL {
            let mut process_id: DWORD = 0;
            unsafe {
                GetWindowThreadProcessId(hwnd, &mut process_id as *mut DWORD);
                if GetCurrentProcessId() != process_id {
                    return TRUE;
                }

                // set immersive mode
                const DWMWA_USE_IMMERSIVE_DARK_MODE: u32 = 20;
                DwmSetWindowAttribute(
                    hwnd,
                    DWMWA_USE_IMMERSIVE_DARK_MODE,
                    &1 as *const _ as *const c_void,
                    std::mem::size_of::<BOOL>() as u32,
                );

                // set caption background color
                const DWMWA_CAPTION_COLOR: u32 = 35;
                const CAPTION_COLOR_REF: u32 = 0x00FFFFFF;
                DwmSetWindowAttribute(
                    hwnd,
                    DWMWA_CAPTION_COLOR,
                    &CAPTION_COLOR_REF as *const _ as *const c_void,
                    std::mem::size_of::<BOOL>() as u32,
                );

                // set caption text color
                const DWMWA_TEXT_COLOR: u32 = 36;
                const TEXT_COLOR_REF: u32 = 0x00000000;
                DwmSetWindowAttribute(
                    hwnd,
                    DWMWA_TEXT_COLOR,
                    &TEXT_COLOR_REF as *const _ as *const c_void,
                    std::mem::size_of::<BOOL>() as u32,
                );

                // enable mica (win11 builds 22000 - 22494)
                const DWMWA_MICA_EFFECT: u32 = 1029;
                DwmSetWindowAttribute(
                    hwnd,
                    DWMWA_MICA_EFFECT,
                    &1 as *const _ as *const c_void,
                    std::mem::size_of::<BOOL>() as u32,
                );
            }
            FALSE
        }

        EnumWindows(Some(callback), 0);
    }
}

pub fn build_ui() -> impl Widget<AppState> {
    ContainerWidget::new()
}

#[cfg(windows)]
pub fn platform_api_calls(_id: WindowId) {
    unsafe {
        win_api_calls::set_icon();
        win_api_calls::set_mica_effect();
    }
}

#[cfg(not(windows))]
pub fn platform_api_calls(id: WindowId) {}

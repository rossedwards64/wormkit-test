use std::ffi::c_void;

use windows::{
    core::{self, PCWSTR},
    Win32::{
        Foundation::{self, HINSTANCE, HMODULE},
        Storage::FileSystem::{self},
        System::LibraryLoader,
        System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
        UI::{self},
    },
};

const VV1: u32 = 3;
const VV2: u32 = 8;
const VV3: u32 = 1;

const ADDR: u32 = 0xFEEF04BD;

macro_rules! hiword {
    ($l:expr) => {
        $l & 0xffff
    };
}

macro_rules! loword {
    ($l:expr) => {
        ($l >> 16) & 0xfff
    };
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: u32,
    _: *mut (),
) -> core::Result<()> {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            if !check_version(dll_module) {
                unsafe {
                    UI::WindowsAndMessaging::MessageBoxW(
                        Foundation::HWND::default(),
                        core::w!("The version of your game is not 3.8.1."),
                        core::w!("Shit boy detected."),
                        UI::WindowsAndMessaging::MB_OK | UI::WindowsAndMessaging::MB_ICONERROR,
                    );
                }
            } else {
		unsafe {
                    UI::WindowsAndMessaging::MessageBoxW(
                        Foundation::HWND::default(),
                        core::w!("The version of your game is 3.8.1."),
                        core::w!("Genius detected."),
                        UI::WindowsAndMessaging::MB_OK | UI::WindowsAndMessaging::MB_ICONERROR,
                    );
                }
	    }
        }
        DLL_PROCESS_DETACH => detach(),
        _ => (),
    }

    Ok(())
}

fn check_version(_dll_module: HINSTANCE) -> bool {
    let mut correct_version = true;

    unsafe {
        let handle: Option<*mut u32> = Some(&mut 0);
        let buf: &mut [u16] = &mut [];
        LibraryLoader::GetModuleFileNameW(HMODULE::default(), buf);
        let modname = PCWSTR::from_raw(buf.as_ptr());
        let size = FileSystem::GetFileVersionInfoSizeW(modname, handle);

        if size == 0 {
            let buf: *mut c_void = Vec::new().as_mut_ptr();

            let _ = FileSystem::GetFileVersionInfoW(modname, u32::default(), size, buf);

            let info: *mut *mut c_void =
                &mut (&mut FileSystem::VS_FIXEDFILEINFO::default() as *mut _ as *mut c_void);

            let val: PCWSTR = core::w!("\\");
            let len: *mut u32 = &mut 0;

            if FileSystem::VerQueryValueW(buf, val, info, len).as_bool() {
                let info = &mut *(info as *mut FileSystem::VS_FIXEDFILEINFO);

                if info.dwSignature == ADDR
                    && hiword!(info.dwFileVersionMS) == VV1
                    && loword!(info.dwFileVersionMS) == VV2
                    && hiword!(info.dwFileVersionLS) == VV3
                {
                    correct_version = false;
                };
            };
        };
    }
    correct_version
}

fn detach() {
    ()
}

use std::{env, thread};

use crate::{
    core::{
        io::RusticonIo,
        model::{AppPhase, State},
        shared::{DROP_HOLDER, ImportOutcome, RESULT_HOLDER},
    },
    features::{export::export_svg, import::import_file, message::draw_message},
};

#[cfg(target_os = "macos")]
use {
    objc2::rc::Retained,
    objc2::runtime::{AnyClass, AnyObject, Imp, Sel},
    objc2::{MainThreadMarker, ffi, msg_send, sel},
    objc2_app_kit::{NSApplication, NSWindow},
    objc2_foundation::{NSArray, NSString},
};

pub type FileHandle = String;

#[derive(Clone, Default)]
pub struct NativeIo;

impl NativeIo {
    pub fn new() -> Self {
        Self
    }
}

impl RusticonIo for NativeIo {
    fn initial_file_path(&self) -> String {
        env::args()
            .nth(1)
            .unwrap_or_else(|| "favicon.svg".to_string())
    }

    fn initial_phase(&self) -> AppPhase {
        AppPhase::Splash
    }

    fn start_import(&self, path: String) {
        let dropped_path = DROP_HOLDER.lock().unwrap().take();
        let actual_path = dropped_path.unwrap_or(path);

        {
            let mut guard = RESULT_HOLDER.lock().unwrap();
            *guard = None;
        }

        let result_holder_thread = RESULT_HOLDER.clone();

        thread::spawn(move || {
            let result = std::panic::catch_unwind(|| import_file(&actual_path))
                .map_err(|e| format!("Panic in import_file: {:?}", e))
                .and_then(|res| res);

            let mut guard = result_holder_thread.lock().unwrap();
            *guard = Some(result);
        });
    }

    fn launch_drop_ready(&self) -> bool {
        DROP_HOLDER.lock().unwrap().is_some()
    }

    fn take_import_result(&self) -> Option<ImportOutcome> {
        RESULT_HOLDER.lock().unwrap().take()
    }

    fn report_message(&self, msg: &str, color_code: u8) {
        draw_message(msg, color_code);
    }

    fn perform_save(&self, state: &State) {
        let (data, size) = if state.editor.size == 16 {
            (state.editor.canvas16_data.clone(), 16)
        } else {
            (state.editor.canvas8_data.clone(), 8)
        };

        if let Err(err_msg) = export_svg(
            &data,
            &state.editor.palette_colors,
            size,
            size,
            32,
            &state.editor.file_path,
        ) {
            self.report_message(&err_msg, 196);
        }
    }
}

#[cfg(target_os = "windows")]
pub fn setup_windows_drop() {
    use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
    use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
    use windows::Win32::UI::Shell::{DragAcceptFiles, DragFinish, DragQueryFileW, HDROP};
    use windows::Win32::UI::WindowsAndMessaging::{
        CallWindowProcW, GWLP_WNDPROC, SetWindowLongPtrW,
    };

    static HOOKED: AtomicBool = AtomicBool::new(false);
    static ORIG_WNDPROC: AtomicPtr<std::ffi::c_void> = AtomicPtr::new(std::ptr::null_mut());

    if HOOKED.load(Ordering::Relaxed) {
        return;
    }

    let Some(win_arc) = incredible_window_windows::try_window() else {
        return;
    };
    let Ok(win) = win_arc.lock() else {
        return;
    };
    let hwnd = win.hwnd();

    unsafe extern "system" fn drop_subclass_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        const WM_DROPFILES: u32 = 0x0233;
        unsafe {
            if msg == WM_DROPFILES {
                let hdrop = HDROP(wparam.0 as isize);
                let file_count = DragQueryFileW(hdrop, 0xFFFFFFFF, std::ptr::null_mut::<u16>(), 0);
                if file_count > 0 {
                    let len = DragQueryFileW(hdrop, 0, std::ptr::null_mut::<u16>(), 0);
                    if len > 0 {
                        let mut buf = vec![0u16; (len + 1) as usize];
                        DragQueryFileW(hdrop, 0, buf.as_mut_ptr(), len + 1);
                        let path = String::from_utf16_lossy(&buf[..len as usize]);
                        if let Ok(mut guard) = DROP_HOLDER.lock() {
                            *guard = Some(path);
                        }
                    }
                }
                DragFinish(hdrop);
                return LRESULT(0);
            }

            let orig = ORIG_WNDPROC.load(Ordering::SeqCst);
            let orig_proc: Option<unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT> =
                std::mem::transmute(orig);
            CallWindowProcW(orig_proc, hwnd, msg, wparam, lparam)
        }
    }

    unsafe {
        let _ = DragAcceptFiles(hwnd, true);
        let orig = SetWindowLongPtrW(
            hwnd,
            GWLP_WNDPROC,
            drop_subclass_proc as unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT
                as isize,
        );
        ORIG_WNDPROC.store(orig as *mut std::ffi::c_void, Ordering::SeqCst);
    }

    HOOKED.store(true, Ordering::Relaxed);
}

#[cfg(target_os = "macos")]
pub fn setup_macos_hooks() {
    static mut HOOKED: bool = false;
    unsafe {
        if HOOKED {
            return;
        }
        let mtm = MainThreadMarker::new().expect("must be on main thread");
        let app = NSApplication::sharedApplication(mtm);
        let windows = app.windows();

        if windows.len() > 0 {
            let window: Retained<NSWindow> = windows.objectAtIndex(0);
            let view = window.contentView();
            let Some(view) = view else { return };

            let type_string = NSString::from_str("NSFilenamesPboardType");
            let types = NSArray::from_slice(&[&*type_string]);
            let _: () = msg_send![&*window, registerForDraggedTypes: &*types];
            let _: () = msg_send![&*view, registerForDraggedTypes: &*types];

            let cls = view.class() as *const AnyClass as *mut AnyClass;

            extern "C" fn dragging_entered(
                _this: &AnyObject,
                _sel: Sel,
                _sender: &AnyObject,
            ) -> usize {
                1
            }
            let dragging_entered_imp: Imp = std::mem::transmute(
                dragging_entered as extern "C" fn(&AnyObject, Sel, &AnyObject) -> usize,
            );
            let sel_e = sel!(draggingEntered:);
            let _ = ffi::class_addMethod(
                cls as _,
                std::mem::transmute(sel_e),
                dragging_entered_imp,
                "Q@:@\0".as_ptr() as _,
            );

            extern "C" fn dragging_updated(
                _this: &AnyObject,
                _sel: Sel,
                _sender: &AnyObject,
            ) -> usize {
                1
            }
            let dragging_updated_imp: Imp = std::mem::transmute(
                dragging_updated as extern "C" fn(&AnyObject, Sel, &AnyObject) -> usize,
            );
            let sel_u = sel!(draggingUpdated:);
            let _ = ffi::class_addMethod(
                cls as _,
                std::mem::transmute(sel_u),
                dragging_updated_imp,
                "Q@:@\0".as_ptr() as _,
            );

            extern "C" fn perform_drag_operation(
                _this: &AnyObject,
                _sel: Sel,
                sender: &AnyObject,
            ) -> bool {
                unsafe {
                    let pboard: Retained<AnyObject> = msg_send![sender, draggingPasteboard];
                    let type_string = NSString::from_str("NSFilenamesPboardType");
                    let files: Option<Retained<NSArray<NSString>>> =
                        msg_send![&*pboard, propertyListForType: &*type_string];

                    if let Some(files) = files {
                        if files.len() > 0 {
                            let path_ns = files.objectAtIndex(0);
                            let path = path_ns.to_string();
                            if let Ok(mut guard) = DROP_HOLDER.lock() {
                                *guard = Some(path);
                            }
                        }
                    }
                }
                true
            }
            let perform_drag_imp = std::mem::transmute(
                perform_drag_operation as extern "C" fn(&AnyObject, Sel, &AnyObject) -> bool,
            );
            let sel_p = sel!(performDragOperation:);
            let _ = ffi::class_addMethod(
                cls as _,
                std::mem::transmute(sel_p),
                perform_drag_imp,
                "B@:@\0".as_ptr() as _,
            );

            HOOKED = true;
        }
    }
}

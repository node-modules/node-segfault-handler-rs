#[cfg(target_os = "macos")]
use nix::libc::{backtrace, backtrace_symbols_fd};
#[cfg(target_os = "linux")]
use crate::binding::{linux_backtrace, linux_backtrace_symbols_fd};
use nix::sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet, Signal};
use std::backtrace::Backtrace;
use std::ffi::{c_int, c_void};
use crate::binding::{Isolate, StackTrace};

const STACK_LIMIT: usize = 32;

extern "C" fn segfault_action(_sig: c_int, _si: *mut nix::libc::siginfo_t, _: *mut c_void) {
  {
    eprintln!("rust backtrace start");
    let backtrace = Backtrace::force_capture();
    eprintln!("{:#?}", backtrace);
    eprintln!("rust backtrace end");
  }

  #[cfg(target_os = "macos")]
  unsafe {
    eprintln!("c backtrace start");
    let mut array: [*mut c_void; STACK_LIMIT] = [std::ptr::null_mut(); STACK_LIMIT];
    let size = backtrace(array.as_mut_ptr(), STACK_LIMIT as c_int);
    backtrace_symbols_fd(array.as_ptr(), size, 2);
    eprintln!("c backtrace end");
  }
  #[cfg(target_os = "linux")]
  unsafe {
      eprintln!("c backtrace start");
      let mut array: [*mut c_void; STACK_LIMIT] = [std::ptr::null_mut(); STACK_LIMIT];
      let size = linux_backtrace(array.as_mut_ptr(), STACK_LIMIT as c_int);
      linux_backtrace_symbols_fd(array.as_ptr(), size, 2);
      eprintln!("c backtrace end");
  }
  eprintln!("v8 backtrace start");
  print_v8_backtrace();
  eprintln!("v8 backtrace end");
}

fn print_v8_backtrace() {
  let isolate = Isolate::get_isolate().expect("get isolate");

  if let Some(stack_trace) = StackTrace::current_stack_trace(isolate, STACK_LIMIT) {
    let frame_count = stack_trace.get_frame_count();
    for i in 0..frame_count {
      if let Some(frame) = stack_trace.get_frame(isolate, i) {
        let script_name = frame
          .get_script_name_or_source_url()
          .map(|t| t.to_rust_string_lossy(isolate))
          .unwrap_or(String::from("<unknown>"));
        let function_name = frame
          .get_function_name()
          .map(|t| t.to_rust_string_lossy(isolate))
          .unwrap_or(String::from("<anonymous>"));
        // script_name.unwrap()
        let line_number = frame.get_line_number();
        let column_number = frame.get_column();
        eprintln!(
          " {:} ({:}:{:}:{:})",
          function_name, script_name, line_number, column_number
        );
      }
    }
  }
}

pub fn register_segfault_handler() {
  let sig_handler = SigHandler::SigAction(segfault_action);
  let flags = SaFlags::SA_SIGINFO | SaFlags::SA_RESETHAND;
  let sig_action = SigAction::new(sig_handler, flags, SigSet::empty());
  unsafe {
    let _ = sigaction(Signal::SIGSEGV, &sig_action);
  }
}

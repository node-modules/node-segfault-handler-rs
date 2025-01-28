use bitflags::bitflags;
use std::ffi::c_int;
#[cfg(target_os = "linux")]
use std::ffi::c_void;
use std::mem::MaybeUninit;

// copy from v8, v8 binding build with lib v8,
// so v8 can not use for nodejs.
pub type Opaque = [u8; 0];

#[repr(C)]
#[derive(Debug)]
pub struct Isolate(Opaque);

#[repr(C)]
#[derive(Debug)]
pub struct StackTrace(Opaque);

#[repr(C)]
#[derive(Debug)]
pub struct StackFrame(Opaque);

#[repr(C)]
#[derive(Debug)]
pub struct String(Opaque);

#[cfg(target_os = "linux")]
extern "C" {
  pub fn linux_backtrace(buf: *mut *mut c_void, sz: c_int) -> c_int;
  pub fn linux_backtrace_symbols_fd(addrs: *const *mut c_void, sz: c_int, fd: c_int);
}

// extend_v8_binding
#[link(name = "extend_v8_binding")]
extern "C" {
  fn v8__GetIsolate() -> *mut Isolate;

  fn v8__StackTrace__CurrentStackTrace(
    isolate: *const Isolate,
    frame_limit: c_int,
  ) -> *const StackTrace;

  fn v8__StackTrace__GetFrameCount(this: *const StackTrace) -> c_int;

  fn v8__StackTrace__GetFrame(
    this: *const StackTrace,
    isolate: *const Isolate,
    index: u32,
  ) -> *const StackFrame;

  fn v8__StackFrame__GetScriptNameOrSourceURL(this: *const StackFrame) -> *const String;

  fn v8__StackFrame__GetFunctionName(this: *const StackFrame) -> *const String;
  fn v8__StackFrame__GetLineNumber(this: *const StackFrame) -> c_int;
  fn v8__StackFrame__GetColumn(this: *const StackFrame) -> c_int;

  fn v8__String__Length(this: *const String) -> c_int;
  fn v8__String__Utf8Length(this: *const String, isolate: *const Isolate) -> c_int;
  fn v8__String__WriteUtf8(
    this: *const String,
    isolate: *const Isolate,
    buffer: *mut u32,
    length: c_int,
    nchars_ref: *mut c_int,
    options: WriteOptions,
  ) -> c_int;
  fn v8__String__WriteOneByte(
    this: *const String,
    isolate: *const Isolate,
    buffer: *mut u8,
    start: c_int,
    length: c_int,
    options: WriteOptions,
  ) -> c_int;
  fn v8__String__IsOneByte(this: *const String) -> bool;
}

impl Isolate {
  pub fn get_isolate() -> Option<&'static mut Isolate> {
    unsafe { v8__GetIsolate().as_mut() }
  }
}

impl StackTrace {
  #[inline(always)]
  pub fn current_stack_trace(isolate: &Isolate, frame_limit: usize) -> Option<&StackTrace> {
    let frame_limit = frame_limit.try_into().ok()?;
    unsafe { v8__StackTrace__CurrentStackTrace(isolate, frame_limit).as_ref() }
  }

  #[inline(always)]
  pub fn get_frame_count(&self) -> usize {
    unsafe { v8__StackTrace__GetFrameCount(self) as usize }
  }

  /// Returns a StackFrame at a particular index.
  #[inline(always)]
  pub fn get_frame<'s>(&self, isolate: &Isolate, index: usize) -> Option<&'s StackFrame> {
    unsafe { v8__StackTrace__GetFrame(self, isolate, index as u32).as_ref() }
  }
}

impl StackFrame {
  #[inline(always)]
  pub fn get_script_name_or_source_url<'s>(&self) -> Option<&'s String> {
    unsafe { v8__StackFrame__GetScriptNameOrSourceURL(self).as_ref() }
  }

  /// Returns the name of the function associated with this stack frame.
  #[inline(always)]
  pub fn get_function_name<'s>(&self) -> Option<&'s String> {
    unsafe { v8__StackFrame__GetFunctionName(self).as_ref() }
  }

  #[inline(always)]
  pub fn get_line_number(&self) -> usize {
    unsafe { v8__StackFrame__GetLineNumber(self) as usize }
  }

  #[inline(always)]
  pub fn get_column(&self) -> usize {
    unsafe { v8__StackFrame__GetColumn(self) as usize }
  }
}

bitflags! {
  #[derive(Clone, Copy, Default)]
  #[repr(transparent)]
  pub struct WriteOptions: c_int {
    const NO_OPTIONS = 0;
    const HINT_MANY_WRITES_EXPECTED = 1;
    const NO_NULL_TERMINATION = 2;
    const PRESERVE_ONE_BYTE_NULL = 4;
    // Used by WriteUtf8 to replace orphan surrogate code units with the
    // unicode replacement character. Needs to be set to guarantee valid UTF-8
    // output.
    const REPLACE_INVALID_UTF8 = 8;
  }
}

impl String {
  #[inline(always)]
  pub fn length(&self) -> usize {
    unsafe { v8__String__Length(self) as usize }
  }

  #[inline(always)]
  pub fn utf8_length(&self, scope: &Isolate) -> usize {
    unsafe { v8__String__Utf8Length(self, scope) as usize }
  }

  #[inline(always)]
  pub fn is_onebyte(&self) -> bool {
    unsafe { v8__String__IsOneByte(self) }
  }

  #[inline(always)]
  pub fn write_one_byte_uninit(
    &self,
    scope: &Isolate,
    buffer: &mut [MaybeUninit<u8>],
    start: usize,
    options: WriteOptions,
  ) -> usize {
    unsafe {
      v8__String__WriteOneByte(
        self,
        scope,
        buffer.as_mut_ptr() as *mut u8,
        start.try_into().unwrap_or(c_int::MAX),
        buffer.len().try_into().unwrap_or(c_int::MAX),
        options,
      ) as usize
    }
  }

  pub fn write_utf8_uninit(
    &self,
    scope: &Isolate,
    buffer: &mut [MaybeUninit<u8>],
    nchars_ref: Option<&mut usize>,
    options: WriteOptions,
  ) -> usize {
    let mut nchars_ref_int: c_int = 0;
    let bytes = unsafe {
      v8__String__WriteUtf8(
        self,
        scope,
        buffer.as_mut_ptr() as *mut u32,
        buffer.len().try_into().unwrap_or(c_int::MAX),
        &mut nchars_ref_int,
        options,
      )
    };
    if let Some(r) = nchars_ref {
      *r = nchars_ref_int as usize;
    }
    bytes as usize
  }

  pub fn to_rust_string_lossy(&self, scope: &Isolate) -> std::string::String {
    let len_utf16 = self.length();

    // No need to allocate or do any work for zero-length strings
    if len_utf16 == 0 {
      return std::string::String::new();
    }

    let len_utf8 = self.utf8_length(scope);

    // If len_utf8 == len_utf16 and the string is one-byte, we can take the fast memcpy path. This is true iff the
    // string is 100% 7-bit ASCII.
    if self.is_onebyte() && len_utf8 == len_utf16 {
      unsafe {
        // Create an uninitialized buffer of `capacity` bytes. We need to be careful here to avoid
        // accidentally creating a slice of u8 which would be invalid.
        let layout = std::alloc::Layout::from_size_align(len_utf16, 1).unwrap();
        let data = std::alloc::alloc(layout) as *mut MaybeUninit<u8>;
        let buffer = std::ptr::slice_from_raw_parts_mut(data, len_utf16);

        // Write to this MaybeUninit buffer, assuming we're going to fill this entire buffer
        let length = self.write_one_byte_uninit(
          scope,
          &mut *buffer,
          0,
          WriteOptions::NO_NULL_TERMINATION | WriteOptions::REPLACE_INVALID_UTF8,
        );
        debug_assert!(length == len_utf16);

        // Return an owned string from this guaranteed now-initialized data
        let buffer = data as *mut u8;
        return std::string::String::from_raw_parts(buffer, length, len_utf16);
      }
    }

    // SAFETY: This allocates a buffer manually using the default allocator using the string's capacity.
    // We have a large number of invariants to uphold, so please check changes to this code carefully
    unsafe {
      // Create an uninitialized buffer of `capacity` bytes. We need to be careful here to avoid
      // accidentally creating a slice of u8 which would be invalid.
      let layout = std::alloc::Layout::from_size_align(len_utf8, 1).unwrap();
      let data = std::alloc::alloc(layout) as *mut MaybeUninit<u8>;
      let buffer = std::ptr::slice_from_raw_parts_mut(data, len_utf8);

      // Write to this MaybeUninit buffer, assuming we're going to fill this entire buffer
      let length = self.write_utf8_uninit(
        scope,
        &mut *buffer,
        None,
        WriteOptions::NO_NULL_TERMINATION | WriteOptions::REPLACE_INVALID_UTF8,
      );
      debug_assert!(length == len_utf8);

      // Return an owned string from this guaranteed now-initialized data
      let buffer = data as *mut u8;
      std::string::String::from_raw_parts(buffer, length, len_utf8)
    }
  }
}

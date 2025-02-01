#include <iostream>
#include <node.h>
#include <node_buffer.h>
#include <node_version.h>
#include <node_object_wrap.h>
#include "v8.h"
#include <node_api.h>
#include <js_native_api.h>

#ifdef __linux__
#include <execinfo.h>
#endif

#ifdef __linux__
extern "C" {
void linux_backtrace_symbols_fd(void* const* array, int size, int fd) {
  backtrace_symbols_fd(array, size, fd);
}

int  linux_backtrace(void** array, int size) {
  return backtrace(array, size);
}
}
#endif

template <class T>
inline static T* local_to_ptr(v8::Local<T> local) {
    return *local;
}

extern "C" {
v8::Isolate* v8__GetIsolate() {
    return v8::Isolate::GetCurrent();
}

// Note: StackTraceOptions is deprecated, kDetailed is always used
const v8::StackTrace* v8__StackTrace__CurrentStackTrace(v8::Isolate* isolate, int frame_limit) {
    return local_to_ptr(v8::StackTrace::CurrentStackTrace(isolate, frame_limit));
}

int v8__StackTrace__GetFrameCount(const v8::StackTrace& self) {
    return self.GetFrameCount();
}

const v8::StackFrame* v8__StackTrace__GetFrame(const v8::StackTrace& self,
                                                   v8::Isolate* isolate,
                                                   uint32_t index) {
    return local_to_ptr(self.GetFrame(isolate, index));
}

const v8::String* v8__StackFrame__GetScriptNameOrSourceURL(
        const v8::StackFrame& self) {
    return local_to_ptr(self.GetScriptNameOrSourceURL());
}

const v8::String* v8__StackFrame__GetFunctionName(const v8::StackFrame& self) {
    return local_to_ptr(self.GetFunctionName());
}


int v8__StackFrame__GetLineNumber(const v8::StackFrame& self) {
    return self.GetLineNumber();
}

int v8__StackFrame__GetColumn(const v8::StackFrame& self) {
    return self.GetColumn();
}

int v8__String__Length(const v8::String& self) { return self.Length(); }

int v8__String__Utf8Length(const v8::String& self, v8::Isolate* isolate) {
    return self.Utf8Length(isolate);
}

int v8__String__WriteUtf8(const v8::String& self, v8::Isolate* isolate,
                              char* buffer, int length, int* nchars_ref,
                              int options) {
    return self.WriteUtf8(isolate, buffer, length, nchars_ref, options);
}

int v8__String__WriteOneByte(const v8::String& self, v8::Isolate* isolate,
                                 uint8_t* buffer, int start, int length,
                                 int options) {
    return self.WriteOneByte(isolate, buffer, start, length, options);
}

bool v8__String__IsOneByte(const v8::String& self) { return self.IsOneByte(); }
}
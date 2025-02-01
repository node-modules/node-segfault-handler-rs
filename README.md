# Quick Start

## Install

```shell
npm i --save node-segfault-handler-rs
```

```js
const { register } = require('node-segfault-handler-rs');

register();
```

## Log
```shell
rust backtrace start
Backtrace [
    { fn: "std::backtrace_rs::backtrace::libunwind::trace", file: "/rustc/9fc6b43126469e3858e2fe86cafb4f0fd5068869/library/std/src/../../backtrace/src/backtrace/libunwind.rs", line: 116 },
    { fn: "std::backtrace_rs::backtrace::trace_unsynchronized", file: "/rustc/9fc6b43126469e3858e2fe86cafb4f0fd5068869/library/std/src/../../backtrace/src/backtrace/mod.rs", line: 66 },
    { fn: "std::backtrace::Backtrace::create", file: "/rustc/9fc6b43126469e3858e2fe86cafb4f0fd5068869/library/std/src/backtrace.rs", line: 331 },
    { fn: "node_segfault_handler_rs::segfault_handler::segfault_action", file: "./src/segfault_handler.rs", line: 12 },
    { fn: "__os_alloc_slow" },
    { fn: "node_segfault_handler_rs::__napi__cause_sigsegv::{{closure}}", file: "./src/lib.rs", line: 9 },
    { fn: "napi::bindgen_prelude::within_runtime_if_available", file: "/Users/killa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/napi-2.16.13/src/lib.rs", line: 162 },
    { fn: "node_segfault_handler_rs::__napi__cause_sigsegv", file: "./src/lib.rs", line: 9 },
]
rust backtrace end
c backtrace start
0   node-segfault-handler-rs.darwin-arm 0x0000000116c4da28 _ZN24node_segfault_handler_rs16segfault_handler15segfault_action17h10628a5b3a9c0f7bE + 576
1   libsystem_platform.dylib            0x000000019a19ee04 _sigtramp + 56
2   node-segfault-handler-rs.darwin-arm 0x0000000116c4dfcc _ZN24node_segfault_handler_rs21__napi__cause_sigsegv28_$u7b$$u7b$closure$u7d$$u7d$17hd374b5527e7afc97E + 28
3   node-segfault-handler-rs.darwin-arm 0x0000000116c4c994 _ZN4napi15bindgen_prelude27within_runtime_if_available17h144884eeb82fdcd8E + 24
4   node-segfault-handler-rs.darwin-arm 0x0000000116c4ca08 _ZN24node_segfault_handler_rs21__napi__cause_sigsegv17h35cd5920ef74f338E + 32
5   node                                0x0000000100e7cda8 _ZN6v8impl12_GLOBAL__N_123FunctionCallbackWrapper6InvokeERKN2v820FunctionCallbackInfoINS2_5ValueEEE + 108
6   node                                0x00000001010b9bc8 _ZN2v88internal12_GLOBAL__N_119HandleApiCallHelperILb0EEENS0_11MaybeHandleINS0_6ObjectEEEPNS0_7IsolateENS0_6HandleINS0_10HeapObjectEEENS8_INS0_20FunctionTemplateInfoEEENS8_IS4_EEPmi + 728
7   node                                0x00000001010b92c0 _ZN2v88internal21Builtin_HandleApiCallEiPmPNS0_7IsolateE + 192
8   node                                0x0000000101940b24 Builtins_CEntry_Return1_ArgvOnStack_BuiltinExit + 100
9   node                                0x00000001018b83e4 Builtins_InterpreterEntryTrampoline + 260
10  node                                0x00000001018b83e4 Builtins_InterpreterEntryTrampoline + 260
11  node                                0x00000001018b83e4 Builtins_InterpreterEntryTrampoline + 260
12  node                                0x00000001018b650c Builtins_JSEntryTrampoline + 172
13  node                                0x00000001018b61f4 Builtins_JSEntry + 148
14  node                                0x000000010118dbc8 _ZN2v88internal12_GLOBAL__N_16InvokeEPNS0_7IsolateERKNS1_12InvokeParamsE + 2972
15  node                                0x000000010118d014 _ZN2v88internal9Execution4CallEPNS0_7IsolateENS0_6HandleINS0_6ObjectEEES6_iPS6_ + 200
16  node                                0x0000000101067904 _ZN2v88Function4CallENS_5LocalINS_7ContextEEENS1_INS_5ValueEEEiPS5_ + 520
17  node                                0x0000000100e64868 _ZN4node11Environment9RunTimersEP10uv_timer_s + 552
18  node                                0x0000000101894634 uv__run_timers + 40
19  node                                0x0000000101897e94 uv_run + 1372
20  node                                0x0000000100ded6f0 _ZN4node21SpinEventLoopInternalEPNS_11EnvironmentE + 256
21  node                                0x0000000100f033f0 _ZN4node16NodeMainInstance3RunEPNS_8ExitCodeEPNS_11EnvironmentE + 316
22  node                                0x0000000100f03104 _ZN4node16NodeMainInstance3RunEv + 124
23  node                                0x0000000100e8afa0 _ZN4node5StartEiPPc + 640
24  dyld                                0x0000000199de8274 start + 2840
c backtrace end
v8 backtrace start
 <anonymous> (/Users/killa/workspace/projects/node-segfault-handler-rs/node-segfault-handler-rs/test/fixtures/test.js:6:3)
 listOnTimeout (node:internal/timers:581:17)
 processTimers (node:internal/timers:519:7)
v8 backtrace end
```

# For Developer

## Test

Run build before test if modified rust code.
```shell
npm run build
npm run test
```

# Acknowledgements

- Inspired by [node-segfault-handler](https://github.com/ddopson/node-segfault-handler) for its implementation of backtrace.
- Thanks to the [napi-rs](https://github.com/napi-rs/napi-rs) for delivering an amazing Rust development experience.
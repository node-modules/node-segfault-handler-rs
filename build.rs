extern crate napi_build;
extern crate which;

use std::path::PathBuf;

fn main() {
  let node_include_dir = if let Ok(node_dir) = std::env::var("NODE_DIR") {
    PathBuf::from(node_dir).join("include/node")
  } else if let Ok(node_exec_path) = which::which("node") {
    if let Some(node_bin_dir) = node_exec_path.parent() {
      node_bin_dir.join("../include/node")
    } else {
      panic!("node install with out include dir")
    }
  } else {
    panic!("not found node exec, please set NODE_DIR env like ~/.nvm/versions/node/v20.17.0");
  };

  // build binding.cc to extend_v8_binding
  cc::Build::new()
    .cpp(true)
    .flag_if_supported("-std=c++17")
    .flag("-Wno-unused-parameter")
    .include(node_include_dir)
    .file("src/binding.cc")
    .compile("extend_v8_binding");

  println!("cargo:rerun-if-changed=src/binding.cc");

  napi_build::setup();
}

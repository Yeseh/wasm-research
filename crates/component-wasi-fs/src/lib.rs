use anyhow::Result;
use wasi::filesystem::{types, preopens};

wit_bindgen::generate!({
  path: "../../wit",
  world: "fs",
  exports: { world: Fs }
});

struct Fs;
impl Guest for Fs {
    fn run() -> anyhow::Result<(), String> {
      // Get list of preopened directories provided by the host
      let preopens = preopens::get_directories();
      // Assuming we only get one preopened directory
      // Get its file descriptor 
      if let Some(opt) = preopens.first() {
        let discr = &opt.0;
        // Setup flags
        let oflags = types::OpenFlags::CREATE | types::OpenFlags::TRUNCATE;
        let pflags = types::PathFlags::SYMLINK_FOLLOW;
        let dflags = types::DescriptorFlags::WRITE;

        let fd = discr.open_at(pflags, "hello.txt", oflags, dflags).unwrap();
        fd.write("Hello world! From component.".as_bytes(), 0).unwrap();

        return Ok(());
      }

      Err("No preopened directory found".to_string())
    }
}

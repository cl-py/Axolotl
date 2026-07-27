/*

*/

use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

use libbpf_cargo::SkeletonBuilder;

const BPF_PROGRAMS: &[(&str, &str)] = &[
    ("src/bpf/filtering.bpf.c","filtering.skel.rs"),
    ("src/bpf/logging.bpf.c","logging.skel.rs")
];

fn main() {
    let out = PathBuf::from(
            env::var_os("OUT_DIR")
            .expect("OUT_DIR must be set in build script"));

    let _arch = env::var("CARGO_CFG_TARGET_ARCH")
                        .unwrap_or_else(|_| "x86_64".to_string());

    // Path to the vmlinux directory containing arch-specific headers
    let vmlinux_path = ".";

    for &(src, skeleton_name) in BPF_PROGRAMS{

        let mut output_path = out.clone();
        output_path.push(skeleton_name);

        SkeletonBuilder::new()
                        .source(src)
                        .clang_args([
                            OsStr::new("-I"),
                            Path::new(&vmlinux_path).as_os_str(),
                            OsStr::new("-I"),
                            Path::new(".output").as_os_str(),
                        ])
                        .build_and_generate(output_path)
                        .expect("bpf compilation failed");

        println!("cargo:rerun-if-changed={}", src);
    }
}

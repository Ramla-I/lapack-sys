#![feature(io, path)]

use std::old_io as io;
use std::os;

macro_rules! cmd(
    ($name:expr) => (io::process::Command::new($name));
);

macro_rules! fmt(
    ($($arg:tt)*) => (&format!($($arg)*)[]);
);

macro_rules! get(
    ($name:expr) => (os::getenv($name).unwrap());
);

macro_rules! run(
    ($command:expr) => (
        assert!($command.stdout(io::process::InheritFd(1))
                        .stderr(io::process::InheritFd(2))
                        .status().unwrap().success());
    );
);

fn main() {
    let from = Path::new(get!("CARGO_MANIFEST_DIR")).join("lapack");
    let into = Path::new(get!("OUT_DIR"));

    run!(cmd!("cmake").cwd(&into).arg(&from)
                                 .arg("-DCMAKE_Fortran_FLAGS='-O2 -frecursive -fPIC'"));

    run!(cmd!("cmake").cwd(&into).arg("--build").arg(".")
                                 .arg("--target").arg("lapack")
                                 .arg("--")
                                 .arg(fmt!("-j{}", get!("NUM_JOBS"))));

    println!("cargo:rustc-flags=-L {}", into.join("lib").display());
    println!("cargo:rustc-flags=-l lapack:static");
}

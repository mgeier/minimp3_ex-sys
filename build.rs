fn main() {
    let mut build = cc::Build::new();

    if cfg!(feature = "float-output") {
        build.define("MINIMP3_FLOAT_OUTPUT", None);
    }

    build
        .file("minimp3_ex.c")
        .include("minimp3")
        .compile("minimp3_ex");
}

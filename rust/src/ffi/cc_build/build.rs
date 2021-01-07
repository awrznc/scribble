fn main() {
    cc::Build::new()
        .warnings(true)
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-g")
        .file("src/c/color.c")
        .compile("libcolor.a");
}

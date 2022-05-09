fn main() {

    cxx_build::bridge("src/main.rs")
        .file("src/fpng.cpp")
        .flag_if_supported("-msse4")
        .flag_if_supported("-mpclmul")
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-s")
        .compile("fpng");
}
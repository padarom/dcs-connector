fn main() {
    cc::Build::new()
        .cpp(true)
        .file("cpp/proc.cpp")
        .file("cpp/mem.cpp")
        .file("cpp/connector.cpp")
        .compile("dcs_connector.a");
}

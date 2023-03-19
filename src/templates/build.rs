fn main() {
    glib_build_tools::compile_resources(
        &["."],
        "src/templates/resources.gresource.xml",
        "templates.gresource",
    );
}

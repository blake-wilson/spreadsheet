pub fn main() {
    // composite_templates
    glib_build_tools::compile_resources(
        &["src/templates"],
        "src/templates/resources.gresource.xml",
        "tempaltes.gresource",
    );
}

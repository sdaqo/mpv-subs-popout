fn main() {
    glib_build_tools::compile_resources(
        &["resources"],
        "resources/mpv_subs_popout.gresources.xml",
        "mpv_subs_popout.gresources"
    );
}

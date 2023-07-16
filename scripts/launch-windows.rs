use std::env;
use std::process::Command;

fn main() {
    let exe_path = env::current_exe().expect("Failed to get the launcher executable path");
    let launcher_dir = exe_path.parent().expect("Failed to get the launcher directory");

    let gtk3_dir = launcher_dir.join("gtk3-runtime");
    let path = env::var_os("PATH").unwrap_or_default();
    let mut new_path = env::split_paths(&path).collect::<Vec<_>>();
    new_path.insert(0, gtk3_dir);
    let new_path = env::join_paths(new_path).unwrap();
    env::set_var("PATH", &new_path);

    let mpv_subs_popout_exe = launcher_dir.join("bin/mpv-subs-popout.exe");
    Command::new(mpv_subs_popout_exe).spawn().unwrap();
}

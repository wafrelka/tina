use std::process::Command;

fn main()
{
	let git_output = Command::new("git").args(&["rev-parse", "HEAD"]).output().ok();
	let rev = git_output.filter(|o| o.status.success()).and_then(|o| String::from_utf8(o.stdout).ok()).unwrap_or("<none>".to_owned());
	println!("cargo:rustc-env=TINA_REVISION={}", rev);
}

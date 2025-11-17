use crab_shell::prelude::*;

fn main() -> crab_shell::Result<()> {
    println!("Listing current directory:");
    for path in ls(".")?.take(5) {
        println!(" - {}", path.display());
    }

    let rustc_version = cmd("rustc").arg("--version").read()?;
    println!("rustc -> {rustc_version}");
    Ok(())
}

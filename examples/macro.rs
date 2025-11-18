use qshr::qshr;

fn main() -> qshr::Result<()> {
    qshr! {
        println!("== macro powered script ==");
        sh("echo hello via macro").run()?;

        let rustc = cmd("rustc").arg("--version").read()?;
        println!("rustc -> {}", rustc.trim());
        Ok(())
    }
}

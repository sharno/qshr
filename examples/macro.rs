use qshr::qshr;

fn main() -> qshr::Result<()> {
    qshr! {
        println!("== macro powered script ==");
        "echo hello via macro";

        let rustc = cmd("rustc").arg("--version").read()?;
        println!("rustc -> {}", rustc.trim());

        "echo piping through more" | "more";
    }
}

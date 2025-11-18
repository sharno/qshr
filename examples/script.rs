use qshr::prelude::*;

fn main() -> qshr::Result<()> {
    println!("== checking versions ==");
    let rustc = cmd("rustc").arg("--version").stdout_text()?;
    println!("rustc: {}", rustc.trim());

    println!("== listing *.rs in src ==");
    for entry in glob_entries("src/*.rs")? {
        let entry = entry?;
        println!("- {}", entry.path.display());
    }

    println!("== piping command ==");
    let pipeline = sh("echo hello").pipe(sh("more"));
    for line in pipeline.stream_lines()? {
        println!("{}", line?);
    }

    Ok(())
}

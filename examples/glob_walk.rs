use qshr::prelude::*;

fn main() -> qshr::Result<()> {
    let pattern = "src/**/*.rs";
    println!("Rust sources matching {pattern:?}:");

    let entries = glob_entries(pattern)?;
    let mut filtered = 0;
    for entry in filter_extension(entries, "rs") {
        let entry = entry?;
        println!("  {}", entry.path.display());
        filtered += 1;
    }

    println!("Total files: {filtered}");
    Ok(())
}

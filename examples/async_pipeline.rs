#[cfg(feature = "async")]
use crab_shell::prelude::*;
#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> crab_shell::Result<()> {
    let pipeline = sh("echo alpha && echo beta").pipe(sh("more"));
    let lines: crab_shell::Result<Vec<_>> =
        pipeline.stream_lines_async().await?.collect();
    println!("lines: {:?}", lines?);
    Ok(())
}

#[cfg(not(feature = "async"))]
fn main() -> crab_shell::Result<()> {
    println!("Enable the `async` feature to run this example.");
    Ok(())
}

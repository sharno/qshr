#[cfg(feature = "async")]
use crab_shell::prelude::*;
#[cfg(not(feature = "async"))]
use crab_shell::Result;

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> crab_shell::Result<()> {
    let output = sh("echo async example").output_async().await?;
    println!("stdout: {}", output.stdout_string()?);
    Ok(())
}

#[cfg(not(feature = "async"))]
fn main() -> Result<()> {
    println!("Enable the `async` feature to run this example.");
    Ok(())
}

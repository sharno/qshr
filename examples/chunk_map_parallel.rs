fn main() -> qshr::Result<()> {
    #[cfg(feature = "parallel")]
    {
        use qshr::prelude::*;
        let results: Vec<_> = (0..10)
            .into_iter()
            .chunk_map_parallel(3, |chunk| chunk.into_iter().map(|n| n * n).collect())
            .collect();
        println!("Squares via chunk_map_parallel: {results:?}");
    }
    #[cfg(not(feature = "parallel"))]
    {
        println!("Enable the `parallel` feature to run this example.");
    }
    Ok(())
}

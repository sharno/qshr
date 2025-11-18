//! Qshr - a Turtle-inspired ergonomic shell toolkit for Rust.
//!
//! The crate pairs a lazy [`Shell`] iterator abstraction with a small command
//! runner that provides composable pipelines reminiscent of the Haskell Turtle
//! library.  Everything is synchronous, deterministic, and built on top of the
//! Rust standard library to keep the dependency footprint tiny.

mod command;
mod env;
mod error;
mod fs;
mod shell;

pub mod prelude;

pub use command::{Command, CommandOutput, Pipeline, cmd, sh};
pub use env::*;
pub use error::{Error, Result};
pub use fs::{
    PathEntry, WatchEvent, Watcher, append_text, cat, copy_dir, copy_entries, copy_file,
    debounce_watch, filter_extension, filter_modified_since, filter_size, glob, glob_entries, ls,
    ls_detailed, mkdir_all, move_path, read_lines, read_text, rm, temp_file, walk, walk_detailed,
    walk_files, walk_filter, watch, watch_filtered, watch_glob, write_lines, write_text,
};

#[cfg(feature = "async")]
pub use fs::{watch_async, watch_async_stream, watch_filtered_async};
pub use shell::Shell;

/// Convenience macro for writing quick shell-style scripts.
#[macro_export]
macro_rules! qshr {
    ($($body:tt)*) => {{
        use $crate::prelude::*;
        let __qshr_entry = || -> $crate::Result<()> {
            $crate::__qshr_execute! { $($body)* }
        };
        __qshr_entry()
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __qshr_build_pipeline {
    ($cmd:literal) => {
        $crate::sh($cmd)
    };
    ($cmd:literal | $($rest:tt)+) => {{
        $crate::sh($cmd).pipe($crate::__qshr_build_pipeline!($($rest)+))
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __qshr_execute {
    () => {
        Ok(())
    };
    ($first:literal $(| $next:literal)+ ; $($rest:tt)*) => {{
        $crate::__qshr_build_pipeline!($first $(| $next)+).run()?;
        $crate::__qshr_execute! { $($rest)* }
    }};
    ($first:literal $(| $next:literal)+) => {{
        $crate::__qshr_build_pipeline!($first $(| $next)+).run()?;
        Ok(())
    }};
    ($cmd:literal ; $($rest:tt)*) => {{
        $crate::sh($cmd).run()?;
        $crate::__qshr_execute! { $($rest)* }
    }};
    ($cmd:literal) => {{
        $crate::sh($cmd).run()?;
        Ok(())
    }};
    ($stmt:stmt ; $($rest:tt)*) => {{
        $stmt
        $crate::__qshr_execute! { $($rest)* }
    }};
    ($stmt:stmt) => {{
        $stmt
        Ok(())
    }};
    ($expr:expr) => {{
        $expr
    }};
}

/// Convenience module with the most frequently used items.
///
/// ```no_run
/// use qshr::prelude::*;
///
/// fn main() -> qshr::Result<()> {
///     for path in ls(".")? {
///         println!("{}", path?.display());
///     }
///
///     let lines = cmd("cargo").arg("--version").lines()?;
///     for line in lines {
///         println!("cargo: {}", line?);
///     }
///
///     Ok(())
/// }
/// ```

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_basic_map() {
        let mapped: Vec<_> = Shell::from_iter([1, 2, 3]).map(|n| n * n).collect();
        assert_eq!(mapped, vec![1, 4, 9]);
    }

    #[test]
    fn macro_runs_script() -> Result<()> {
        qshr! {
            "echo macro works";
            let output = cmd("rustc").arg("--version").read()?;
            assert!(output.contains("rustc"));
            "echo macro works" | "more";
        }?;
        Ok(())
    }
}

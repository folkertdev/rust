use std::collections::HashMap;

use run_make_support::{cargo, target};

const CRATE: &str = "app";

fn run(alignment: Option<usize>) -> HashMap<String, usize> {
    let mut cmd = cargo();
    cmd.args(&["run", "--target", &target(), "--profile", "dev"]);

    if let Some(alignment) = alignment {
        cmd.env("RUSTFLAGS", &format!("-Zmin-function-alignment={alignment}"));
    }

    let output = cmd.run();
    println!("{}", output.stderr_utf8());
    let output = output.stdout_utf8();

    output
        .lines()
        .map(|line| {
            let (name, address) = line.split_once(' ').unwrap();
            let address = address.parse::<usize>().unwrap();

            // Some targets perform pointer tagging; clear that bit.
            (name.to_string(), address & !0b1)
        })
        .collect()
}

fn main() {
    std::env::set_current_dir(CRATE).unwrap();

    // 1. Application alignment is higher than dependency alignment.

    // `Some` will set `RUSTFLAGS` for all compiled crates.
    let alignment = 4096;
    for (name, address) in run(Some(alignment)) {
        assert!(
            address.is_multiple_of(alignment),
            "{name}: {address} is not a multiple of {alignment}"
        );
    }

    // 2. Application alignment is lower than dependency alignment.

    // `None` will use the per-crate min-function-alignment set in the Cargo.toml.
    let result = run(None);
    for (name, address) in result.iter() {
        let alignment = 32;
        assert!(
            address.is_multiple_of(alignment),
            "{name}: {address} is not a multiple of {alignment}"
        );
    }

    // Symbols from `dep` should be aligned to its default of 2048.
    let dep_symbols = [result["dep::add"], result["X::default_impl1"], result["X::default_impl2"]];
    assert!(dep_symbols.iter().all(|address| address.is_multiple_of(2048)))
}

//! Repository automation that needs sequencing or dependency-aware behavior.

#![forbid(unsafe_code)]

use std::env;
use std::ffi::OsStr;
use std::process::{Command, ExitCode};

const ROOT_PACKAGES: [&str; 3] = [
    "mono-proto-codec",
    "mono-proto-crypto",
    "mono-proto-session",
];
const CODEC_PATCH: [&str; 2] = [
    "--config",
    "patch.crates-io.mono-proto-codec.path=\"crates/mono-proto-codec\"",
];
const CRYPTO_PATCH: [&str; 2] = [
    "--config",
    "patch.crates-io.mono-proto-crypto.path=\"crates/mono-proto-crypto\"",
];
const SESSION_PATCH: [&str; 2] = [
    "--config",
    "patch.crates-io.mono-proto-session.path=\"crates/mono-proto-session\"",
];
const TRANSPORT_PATCH: [&str; 2] = [
    "--config",
    "patch.crates-io.mono-proto-transport.path=\"crates/mono-proto-transport\"",
];
const ENGINE_PATCH: [&str; 2] = [
    "--config",
    "patch.crates-io.mono-proto-engine.path=\"crates/mono-proto-engine\"",
];

fn run(program: &str, arguments: &[&str]) -> Result<(), String> {
    eprintln!("+ {program} {}", arguments.join(" "));
    let status = Command::new(program)
        .args(arguments.iter().map(OsStr::new))
        .status()
        .map_err(|error| format!("failed to start {program}: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{program} exited with {status}"))
    }
}

fn package() -> Result<(), String> {
    for package in ROOT_PACKAGES {
        run("cargo", &["package", "-p", package, "--allow-dirty"])?;
    }
    // These archives cannot be independently verified until the new root
    // package versions exist in the registry. CI still builds their exact
    // publish archives, and release automation verifies them after publishing
    // dependencies in order.
    let dependent_packages: [(&str, &[&str]); 3] = [
        ("mono-proto-transport", &CRYPTO_PATCH),
        (
            "mono-proto-engine",
            &[
                CODEC_PATCH[0],
                CODEC_PATCH[1],
                CRYPTO_PATCH[0],
                CRYPTO_PATCH[1],
                SESSION_PATCH[0],
                SESSION_PATCH[1],
                TRANSPORT_PATCH[0],
                TRANSPORT_PATCH[1],
            ],
        ),
        (
            "mono-proto-testkit",
            &[
                CODEC_PATCH[0],
                CODEC_PATCH[1],
                CRYPTO_PATCH[0],
                CRYPTO_PATCH[1],
                SESSION_PATCH[0],
                SESSION_PATCH[1],
                TRANSPORT_PATCH[0],
                TRANSPORT_PATCH[1],
                ENGINE_PATCH[0],
                ENGINE_PATCH[1],
            ],
        ),
    ];
    for (package, patches) in dependent_packages {
        let mut arguments = vec!["package", "-p", package, "--allow-dirty", "--no-verify"];
        arguments.extend_from_slice(patches);
        run("cargo", &arguments)?;
    }
    Ok(())
}

fn test_layers() -> Result<(), String> {
    for layer in [
        "layer-223",
        "layer-229",
        "layer-230",
        "layer-231",
        "layer-232",
    ] {
        run(
            "cargo",
            &[
                "test",
                "-p",
                "mono-proto",
                "--no-default-features",
                "--features",
                layer,
            ],
        )?;
    }
    Ok(())
}

fn ci() -> Result<(), String> {
    test_layers()?;
    run("cargo", &["fmt", "--all", "--", "--check"])?;
    run(
        "cargo",
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    run(
        "cargo",
        &["nextest", "run", "--workspace", "--all-features"],
    )?;
    run("cargo", &["test", "--workspace", "--all-features", "--doc"])?;
    package()
}

fn help() {
    eprintln!("Usage: cargo xtask <ci|package|test-layers>");
}

fn main() -> ExitCode {
    let result = match env::args().nth(1).as_deref() {
        Some("ci") => ci(),
        Some("package") => package(),
        Some("test-layers") => test_layers(),
        Some("help" | "-h" | "--help") => {
            help();
            return ExitCode::SUCCESS;
        }
        _ => {
            help();
            return ExitCode::from(2);
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("xtask: {error}");
            ExitCode::FAILURE
        }
    }
}

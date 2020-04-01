use sass_rs::{compile_file, Options, OutputStyle};
use std::{
    fs,
    io::{Error, ErrorKind, Result},
    path::PathBuf,
    process::Command,
};
use which::which;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    // COMPILE TS
    let scripts_listing = fs::read_dir("src/scripts")?.collect::<Result<Vec<_>>>()?;

    println!("cargo:rerun-if-changed=src/scripts");
    fs::create_dir_all("target/scripts/")?;
    for script in scripts_listing {
        let inpath: PathBuf = script.path();

        println!("cargo:rerun-if-changed={}", inpath.to_string_lossy());

        println!(
            "{:?}",
            which("tsc").map_err(|e| Error::new(ErrorKind::NotFound, e))?
        );
        let output = Command::new(which("tsc").map_err(|e| Error::new(ErrorKind::NotFound, e))?)
            .args(&[
                "--outFile",
                "target/scripts/temp.js",
                "-t",
                "ES2016",
                &inpath.to_string_lossy(),
            ])
            .output()?;

        println!("k1");

        if output.status.success() {
            fs::write(
                format!(
                    "target/scripts/{}",
                    inpath
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .replace(".ts", ".js.html")
                ),
                format!(
                    "<script>{}</script>",
                    String::from_utf8_lossy(&fs::read("target/scripts/temp.js")?)
                ),
            )?;

            fs::remove_file("target/scripts/temp.js")?;
        } else {
            println!(
                "{}",
                String::from_utf8_lossy(&output.stdout)
                    .replace("\r", "")
                    .split("\n")
                    .filter_map(|x| if x.len() > 0 {
                        Some(format!("cargo:warning={}\n", x))
                    } else {
                        None
                    })
                    .collect::<String>()
            );
        }
    }

    // COMPILE SCSS

    let styles_listing = fs::read_dir("src/styles")?.collect::<Result<Vec<_>>>()?;
    println!("cargo:rerun-if-changed=src/styles");
    fs::create_dir_all("target/styles/")?;
    for style in styles_listing {
        println!("cargo:rerun-if-changed={}", style.path().to_string_lossy());

        let content = compile_file(
            style.path(),
            Options {
                output_style: OutputStyle::Compressed,
                precision: 2,
                indented_syntax: false,
                include_paths: Vec::new(),
            },
        )
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            panic!("{}", e);
        });

        fs::write(
            format!(
                "target/styles/{}",
                style
                    .path()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .replace(".scss", ".css.html")
            ),
            format!("<style>{}</style>", content),
        )?;
    }

    Ok(())
}

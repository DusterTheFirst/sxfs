use sass_rs::{compile_file, Options, OutputStyle};
use std::{
    env::var,
    fs,
    io::{Error, ErrorKind, Result},
    path::PathBuf,
    process::Command,
};
use which::which;

fn main() -> Result<()> {
    let debug = var("PROFILE").unwrap_or_default() == "debug";

    println!("cargo:rerun-if-changed=build.rs");

    // COMPILE TS
    let scripts_listing = fs::read_dir("src/scripts")?.collect::<Result<Vec<_>>>()?;

    println!("cargo:rerun-if-changed=src/scripts");
    fs::create_dir_all("target/scripts/")?;
    for script in scripts_listing {
        let inpath: PathBuf = script.path();

        println!("cargo:rerun-if-changed={}", inpath.to_string_lossy());

        if inpath.is_dir() {
            continue;
        }

        let args = vec![
            "--outFile",
            "target/scripts/temp.js",
            "--alwaysStrict",
            "--strict",
            if debug { "--inlineSourceMap" } else { "" },
            if debug { "--removeComments" } else { "" },
            "--lib",
            "dom,dom.iterable,es2016",
            "-t",
            "ES2016",
            inpath.to_str().unwrap(),
        ]
        .into_iter()
        .filter(|x| x.len() != 0)
        .collect::<Vec<_>>();

        let output = Command::new(which("tsc").map_err(|e| Error::new(ErrorKind::NotFound, e))?)
            .args(&args)
            .output()?;

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
                    "<script>\n{}\n</script>",
                    String::from_utf8_lossy(&fs::read("target/scripts/temp.js")?)
                ),
            )?;

            fs::remove_file("target/scripts/temp.js")?;
        } else {
            panic!(
                "TSC failed on file {:?} with args {:?}\n{}",
                inpath,
                args,
                String::from_utf8_lossy(&output.stdout)
            );
        }
    }

    // COMPILE SCSS

    let styles_listing = fs::read_dir("src/styles")?.collect::<Result<Vec<_>>>()?;
    println!("cargo:rerun-if-changed=src/styles");
    fs::create_dir_all("target/styles/")?;
    for style in styles_listing {
        println!("cargo:rerun-if-changed={}", style.path().to_string_lossy());

        if style.path().is_dir() {
            continue;
        }

        let content = compile_file(
            style.path(),
            Options {
                output_style: if debug {
                    OutputStyle::Expanded
                } else {
                    OutputStyle::Compressed
                },
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
            format!("<style>\n{}\n</style>", content),
        )?;
    }

    Ok(())
}

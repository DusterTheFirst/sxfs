use std::{
    fs,
    io::{Error, ErrorKind, Result},
    path::PathBuf,
    sync::Arc,
};

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    // COMPILE TS
    {
        use swc::{
            common::{
                errors::{ColorConfig, Handler},
                SourceMap,
            },
            config::{CallerOptions, Config, JscConfig, JscTarget, ModuleConfig, Options},
            ecmascript::{
                parser::Syntax,
                transforms::modules::{umd::Config as UMDConfig, util::Config as UtilConfig},
            },
        };

        let scripts_listing = fs::read_dir("src/scripts")?.collect::<Result<Vec<_>>>()?;

        let cm = Arc::<SourceMap>::default();
        let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
        let c = swc::Compiler::new(cm.clone(), handler);

        println!("cargo:rerun-if-changed=src/scripts");
        for script in scripts_listing {
            let inpath: PathBuf = script.path();

            println!("cargo:rerun-if-changed={}", inpath.to_string_lossy());

            let fm = cm.load_file(inpath.as_path()).expect("failed to load file");

            let processed = c
                .process_js_file(
                    fm,
                    &Options {
                        is_module: false,
                        config: Some(Config {
                            minify: Some(true),
                            jsc: JscConfig {
                                target: JscTarget::Es2016,
                                syntax: Some(Syntax::Typescript(Default::default())),
                                ..Default::default()
                            },
                            module: Some(ModuleConfig::Umd(UMDConfig {
                                config: UtilConfig {
                                    strict: true,
                                    no_interop: true,
                                    ..Default::default()
                                },
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                        caller: Some(CallerOptions { name: "h".into() }),
                        ..Default::default()
                    },
                )
                .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;

            fs::create_dir_all("target/scripts/")?;
            fs::write(
                format!(
                    "target/scripts/{}",
                    inpath
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .replace(".ts", ".js.html")
                ),
                format!("<script>{}</script>", processed.code),
            )?;
        }
    }

    // COMPILE SCSS
    {
        use sass_rs::{compile_file, Options, OutputStyle};

        let styles_listing = fs::read_dir("src/styles")?.collect::<Result<Vec<_>>>()?;
        println!("cargo:rerun-if-changed=src/styles");
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

            fs::create_dir_all("target/styles/")?;
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
    }

    Ok(())
}

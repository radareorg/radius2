use cmake::Config;
use copy_dir::copy_dir;
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

pub struct Build {
    source_dir: PathBuf,
    out_dir: PathBuf,
}

pub struct Artifacts {
    include_dir: PathBuf,
    lib_dir: PathBuf,
    lib: String,
}

impl Build {
    pub fn new() -> Build {
        Build {
            source_dir: Path::new(env!("CARGO_MANIFEST_DIR")).join("boolector"),
            out_dir: PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR not set"))
                .join("vendor-build"),
        }
    }

    pub fn prerequisites(&mut self) -> &mut Self {
        // it is not allowed to modify files outside of OUT_DIR,
        // so everything has to be copied to OUT_DIR before the build can be started
        if !self.out_dir.exists() {
            copy_dir(&self.source_dir, &self.out_dir)
                .expect("Unable to copy Boolector sources to OUT_DIR");
        }

        if !self.out_dir.join("deps/install/lib/liblgl.a").exists() {
            self.run_command(
                Command::new("/usr/bin/env")
                    .arg("bash")
                    .arg(self.out_dir.join("contrib/setup-lingeling.sh"))
                    .current_dir(&self.out_dir),
                "Setup Lingeling",
            );
        }

        println!(
            "cargo:rustc-link-search=native={}",
            self.out_dir.join("deps/install/lib").display()
        );
        println!("cargo:rustc-link-lib=static=lgl");
        println!(
            "cargo:include={}",
            self.out_dir.join("deps/install/include").display()
        );
        println!(
            "cargo:lib={}",
            self.out_dir.join("deps/install/lib").display()
        );

        if !self
            .out_dir
            .join("deps/install/lib/libbtor2parser.a")
            .exists()
        {
            self.run_command(
                Command::new("/usr/bin/env")
                    .arg("bash")
                    .arg(self.out_dir.join("contrib/setup-btor2tools.sh"))
                    .current_dir(&self.out_dir),
                "Setup btor2tools",
            );
        }

        self
    }

    pub fn build(&mut self) -> Artifacts {
        // TODO: Unfortunately BUILDDIR is not overwriteable either implement
        // whole `Configure.sh` or find out how to overwrite `$BUILDDIR`
        let mut config = Config::new(&self.out_dir);
        config.profile("Release");
        config.define("ONLY_LINGELING", "ON");

        if target_os() == "windows" {
            config.generator("MSYS Makefiles");
            config.define("IS_WINDOWS_BUILD", "1");
        }

        let target_dir = config.build();

        Artifacts {
            lib_dir: target_dir.join("lib"),
            include_dir: target_dir.join("include"),
            lib: "boolector".to_string(),
        }
    }

    fn run_command(&self, command: &mut Command, desc: &str) {
        println!("running {:?}", command);
        let status = command.status().unwrap();
        if !status.success() {
            panic!(
                "
Error {}:
    Command: {:?}
    Exit status: {}
    ",
                desc, command, status
            );
        }
    }
}

fn target_os() -> String {
    env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS not set")
}

impl Artifacts {
    pub fn include_dir(&self) -> &Path {
        &self.include_dir
    }

    pub fn lib_dir(&self) -> &Path {
        &self.lib_dir
    }

    pub fn lib(&self) -> &String {
        &self.lib
    }

    pub fn print_cargo_metadata(&self) {
        println!("cargo:rustc-link-search=native={}", self.lib_dir.display());
        println!("cargo:rustc-link-lib=static={}", self.lib);
        println!("cargo:include={}", self.include_dir.display());
        println!("cargo:lib={}", self.lib_dir.display());
    }
}

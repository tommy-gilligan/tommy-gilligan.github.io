use std::env;
use std::fs;
use std::path::Path;

markup::define! {
    Home() {
        @markup::doctype()
        html {
            head {
                style {
                    "body { background: #fafbfc; }"
                    "#main { padding: 2rem; }"
                }
            }
            body {
                @Footer { year: 2020 }
            }
        }
    }

    Footer(year: u32) {
        footer {
            "(c) " @year
        }
    }
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("index.html");
    fs::write(dest_path, format!("{}", Home {})).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}

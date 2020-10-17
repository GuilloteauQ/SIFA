use std::fs::File;
use std::io::Write;

use crate::config::Preference;

/// Template for a shell.nix

pub struct NixShell {
    name: String,
    packages: Vec<String>,
}

impl NixShell {
    pub fn new(name: String, packages: Vec<String>) -> Self {
        NixShell { name, packages }
    }

    pub fn generate_file(&self, filename: Option<String>) {
        let packages_string: String = self.packages.join("\n\t\t");
        if let Some(filename) = filename {
            let mut file = File::create(filename).unwrap();

            write!(
                &mut file,
                "with import <nixpkgs> {{}};\n\
                stdenv.mkDerivation {{\n\
                    \tname = \"{}\";\n\
                    \tbuildInputs = [\n\
                    \t\t{}\n\
                    \t];\n\
                }}",
                self.name, packages_string
            )
            .unwrap();
        } else {
            println!(
                "with import <nixpkgs> {{}};\n\
                stdenv.mkDerivation {{\n\
                    \tname = \"{}\";\n\
                    \tbuildInputs = [\n\
                    \t\t{}\n\
                    \t];\n\
                }}",
                self.name, packages_string
            );
        }
    }
}

impl From<Preference> for NixShell {
    fn from(pref: Preference) -> Self {
        NixShell {
            name: pref.name,
            packages: pref.packages,
        }
    }
}

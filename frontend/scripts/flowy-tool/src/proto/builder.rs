use crate::proto::ProtoGen;

#[allow(dead_code)]
pub struct ProtoGenBuilder {
    rust_source_dirs: Option<Vec<String>>,
    flutter_package_lib: Option<String>,
    derive_meta_dir: Option<String>,
}

impl ProtoGenBuilder {
    pub fn new() -> Self {
        ProtoGenBuilder {
            rust_source_dirs: None,
            flutter_package_lib: None,
            derive_meta_dir: None,
        }
    }

    pub fn set_rust_source_dirs(mut self, dirs: Vec<String>) -> Self {
        self.rust_source_dirs = Some(dirs);
        self
    }

    pub fn set_flutter_package_lib(mut self, dir: &str) -> Self {
        self.flutter_package_lib = Some(dir.to_string());
        self
    }

    pub fn set_derive_meta_dir(mut self, dir: &str) -> Self {
        self.derive_meta_dir = Some(dir.to_string());
        self
    }

    pub fn build(self) -> ProtoGen {
        ProtoGen {
            rust_source_dirs: self.rust_source_dirs.unwrap(),
            flutter_package_lib: self.flutter_package_lib.unwrap(),
            derive_meta_dir: self.derive_meta_dir.unwrap(),
        }
    }
}

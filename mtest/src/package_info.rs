use cargo::core::{Package as CargoPackage, SourceId};
use cargo::core::manifest::EitherManifest;
use cargo::sources::PathSource;
use cargo::util::Config;
use cargo::util::toml;
use std::path::PathBuf;

pub struct PackageInfo {
    pub id: String,
    pub dep_root: Option<PathBuf>,
    pub tested_root: Option<PathBuf>,
    pub files: Vec<PathBuf>,
}

impl PackageInfo {
    pub fn new(id: &str, manifest_path: &str, is_dep: bool) -> Self {
        let mut path = PathBuf::from(manifest_path);
        let files = get_package_files(&path);
        if !path.pop() {
            panic!("43");
        }
        let (dep_root, tested_root) = match is_dep {
            true => (Some(path), None),
            false => (None, Some(path)),
        };
        PackageInfo {
            id: id.to_string(),
            dep_root,
            tested_root,
            files,
        }
    }
}

fn get_package_files(src_manifest: &PathBuf) -> Vec<PathBuf> {
    let src = src_manifest.parent().expect("34");
    let source_id = SourceId::for_path(src).expect("32");
    let config = Config::default().expect("30");
    let (either_manifest, _) = toml::read_manifest(src_manifest, &source_id, &config).expect("33");
    let manifest = match either_manifest {
        EitherManifest::Real(manifest) => manifest,
        EitherManifest::Virtual(_) => return vec![src_manifest.clone()],
    };
    PathSource::new(src_manifest, &source_id, &config)
        .list_files(&CargoPackage::new(manifest, src_manifest))
        .expect("31")
}
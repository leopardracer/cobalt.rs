use relative_path::RelativePathBuf;

#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
#[cfg_attr(not(feature = "unstable"), non_exhaustive)]
pub struct SourcePath {
    pub abs_path: std::path::PathBuf,
    pub rel_path: RelativePathBuf,
}

impl SourcePath {
    pub fn from_root(root: &std::path::Path, path: &std::path::Path) -> Option<Self> {
        let abs_path = path.to_owned();
        let rel_path = path.strip_prefix(root).ok()?;
        let rel_path = RelativePathBuf::from_path(rel_path).ok()?;
        Some(Self { abs_path, rel_path })
    }

    pub fn pop(&mut self) -> bool {
        let abs = self.abs_path.pop();
        let rel = self.rel_path.pop();
        assert_eq!(abs, rel);
        abs
    }

    pub fn push(&mut self, name: &str) {
        self.abs_path.push(name);
        self.rel_path.push(name);
    }
}

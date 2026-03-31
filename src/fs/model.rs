#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryKind {
    Directory,
    File,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsEntry {
    pub relative_path: String,
    pub name: String,
    pub kind: EntryKind,
}

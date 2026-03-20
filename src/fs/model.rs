

enum EntryKind {
    Directory,
    File,
}

struct FsEntry {
    relative_path: String,
    name: String,
    kind: EntryKind,
}


enum TreeNode {
    Directory(DirectoryNode),
    File(FileNode),
}

struct DirectoryNode {
    name: String,
    children: Vec<TreeNode>,
}

struct  FileNode {
    name: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeNode {
    Directory(DirectoryNode),
    File(FileNode),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryNode {
    pub name: String,
    pub children: Vec<TreeNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileNode {
    pub name: String,
}

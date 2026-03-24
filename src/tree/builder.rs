use std::path::Path;

use crate::fs::model::{EntryKind, FsEntry};
use crate::shared::path::display_name;
use crate::tree::model::{DirectoryNode, FileNode, TreeNode};

pub fn build_tree(root: &Path, entries: &[FsEntry]) -> TreeNode {
    let root_name = display_name(root).unwrap_or_else(|| ".".to_string());

    let mut root_node = DirectoryNode {
        name: root_name,
        children: Vec::new(),
    };

    for entry in entries {
        insert_entry(&mut root_node, entry);
    }

    TreeNode::Directory(root_node)
}

fn insert_entry(root: &mut DirectoryNode, entry: &FsEntry) {
    let parts: Vec<&str> = entry.relative_path.split('/').collect();
    insert_parts(root, &parts, &entry.kind);
}

fn insert_parts(current: &mut DirectoryNode, parts: &[&str], kind: &EntryKind) {
    if parts.is_empty() {
        return;
    }

    if parts.len() == 1 {
        match kind {
            EntryKind::Directory => {
                if !current.children.iter().any(|child| match child {
                    TreeNode::Directory(dir) => dir.name == parts[0],
                    TreeNode::File(_) => false,
                }) {
                    current.children.push(TreeNode::Directory(DirectoryNode { 
                        name: parts[0].to_string(), 
                        children: Vec::new(),
                    }));
                }
            }
            EntryKind::File => {
                if !current.children.iter().any(|child| match child {
                    TreeNode::File(file) => file.name == parts[0],
                    TreeNode::Directory(_) => false,
                }) {
                    current.children.push(TreeNode::File(FileNode { 
                        name: parts[0].to_string(), 
                    }));
                }
            }
        }

        return;
    }

    let dir_name = parts[0];

    let existing_index = current.children.iter().position(|child| match child {
        TreeNode::Directory(dir) => dir.name == dir_name,
        TreeNode::File(_) => false,
    });

    let index = match existing_index {
        Some(index) => index,
        None => {
            current.children.push(TreeNode::Directory(DirectoryNode { 
                name: dir_name.to_string(), 
                children: Vec::new(),
            }));
            current.children.len() - 1
        }
    };

    if let TreeNode::Directory(dir) = &mut current.children[index] {
        insert_parts(dir, &parts[1..], kind);
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn builds_root_node_with_name() {
        let root = PathBuf::from("/project");
        let entries = vec![];

        let tree = build_tree(&root, &entries);

        match tree {
            TreeNode::Directory(dir) => {
                assert_eq!(dir.name, "project");
                assert!(dir.children.is_empty());
            }
            TreeNode::File(_) => panic!("expected root directory"),
        }
    }

    #[test]
    fn builds_nested_directory_and_file_structure() {
        let root = PathBuf::from("/project");
        let entries = vec![
            FsEntry {
                relative_path: "src".to_string(),
                name: "src".to_string(),
                kind: EntryKind::Directory,
            },
            FsEntry {
                relative_path: "src/main.rs".to_string(),
                name: "main.rs".to_string(),
                kind: EntryKind::File,
            },
        ];

        let tree = build_tree(&root, &entries);

        match tree {
            TreeNode::Directory(root_dir) => {
                assert_eq!(root_dir.children.len(), 1);

                match &root_dir.children[0] {
                    TreeNode::Directory(src_dir) => {
                        assert_eq!(src_dir.name, "src");
                        assert_eq!(src_dir.children.len(), 1);

                        match &src_dir.children[0] {
                            TreeNode::File(file) => assert_eq!(file.name, "main.rs"),
                            _ => panic!("expected file"),
                        }
                    }
                    _ => panic!("expected directory"),
                }
            }
            _ => panic!("expected directory"),

        }
    }

    #[test]
    fn preserves_empty_directory() {
        let root = PathBuf::from("/project");
        let entries = vec![FsEntry {
            relative_path: "empty_dir".to_string(),
            name: "empty_dir".to_string(),
            kind: EntryKind::Directory,
        }];

        let tree = build_tree(&root, &entries);

        match tree {
            TreeNode::Directory(root_dir) => match &root_dir.children[0] {
                TreeNode::Directory(dir) => {
                    assert_eq!(dir.name, "empty_dir");
                    assert!(dir.children.is_empty());
                }
                _ => panic!("expected directory"),
            }
            _ => panic!("expected root directory"),
        }
    }

    #[test]
    fn builds_multiple_siblings() {
        let root = PathBuf::from("/project");
        let entries = vec![
            FsEntry {
                relative_path: "src".to_string(),
                name: "src".to_string(),
                kind: EntryKind::Directory,
            },
            FsEntry {
                relative_path: "Cargo.toml".to_string(),
                name: "Cargo.toml".to_string(),
                kind: EntryKind::File,
            }
        ];

        let tree = build_tree(&root, &entries);

        match tree {
            TreeNode::Directory(root_dir) => {
                assert_eq!(root_dir.children.len(), 2);
            }
            _ => panic!("expected root dir"),
        }
    }
}
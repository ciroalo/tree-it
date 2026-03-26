use crate::render::error::RenderError;
use crate::tree::model::{TreeNode};

pub fn render_tree(root: &TreeNode) -> Result<String, RenderError> {
    match root {
        TreeNode::Directory(dir) => {
            let mut lines = Vec::new();
            lines.push(format!("{}/", dir.name));

            render_directory_children(&dir.children, "", &mut lines);

            Ok(lines.join("\n"))
        }
        TreeNode::File(_) => Err(RenderError::EmptyTree),
    }
}

fn render_directory_children(children: &[TreeNode], prefix: &str, lines: &mut Vec<String>) {
    for (index, child) in children.iter().enumerate() {
        let is_last = index == children.len() - 1;
        let connector = if is_last {"└── "} else {"├── "};

        match child {
            TreeNode::Directory(dir) => {
                lines.push(format!("{prefix}{connector}{}/", dir.name));

                let next_prefix = if is_last {
                    format!("{prefix}   ")
                } else {
                    format!("{prefix}│   ")
                };

                render_directory_children(&dir.children, &next_prefix, lines);
            }
            TreeNode::File(file) => {
                lines.push(format!("{prefix}{connector}{}", file.name));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::model::{DirectoryNode, FileNode, TreeNode};

    #[test]
    fn renders_root_only() {
        let tree = TreeNode::Directory(DirectoryNode { 
            name: "project".to_string() , 
            children: vec![],
        }); 

        let rendered = render_tree(&tree).unwrap();

        assert_eq!(rendered, "project/");
    }

    #[test]
    fn renders_single_file() {
        let tree = TreeNode::Directory(DirectoryNode { 
            name: "project".to_string() , 
            children: vec![TreeNode::File(FileNode { 
                name: "Cargo.toml".to_string(),
            })],
        }); 

        let rendered = render_tree(&tree).unwrap();

        assert_eq!(rendered, "project/\n└── Cargo.toml");
    }

    #[test]
    fn renders_nested_structure() {
        let tree = TreeNode::Directory(DirectoryNode { 
            name: "project".to_string(), 
            children: vec![
                TreeNode::Directory(DirectoryNode { 
                    name: "src".to_string(),
                    children: vec![
                        TreeNode::File(FileNode { name: "main.rs".to_string() }),
                        TreeNode::File(FileNode { name: "lib.rs".to_string() }),
                    ],
                }),
                TreeNode::File(FileNode { name: "Cargo.toml".to_string() }),
            ],
        });

        let rendered = render_tree(&tree).unwrap();

        let expected = "\
project/
├── src/
│   ├── main.rs
│   └── lib.rs
└── Cargo.toml";

        assert_eq!(rendered, expected);
    }

    #[test]
    fn renders_multiple_sibling_directories_and_files() {
        let tree = TreeNode::Directory(DirectoryNode { 
            name: "project".to_string(), 
            children: vec![
                TreeNode::Directory(DirectoryNode { name: "src".to_string(), children: vec![] }),
                TreeNode::Directory(DirectoryNode { name: "tests".to_string(), children: vec![] }),
                TreeNode::File(FileNode { name: "Cargo.toml".to_string() }),
            ],
        });

        let rendered = render_tree(&tree).unwrap();

        let expected = "\
project/
├── src/
├── tests/
└── Cargo.toml";

        assert_eq!(rendered, expected);
    }
}
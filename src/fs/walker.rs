use std::fs;
use std::path::Path;

use crate::filter::evaluator::should_include;
use crate::fs::model::{EntryKind, FsEntry};
use crate::matcher::pattern::CompiledMatcher;
use crate::shared::path::{display_name, normalize_relative_path, PathError};


#[derive(Debug)]
pub enum WalkerError {
    Io(std::io::Error),
    Path(PathError),
}

impl From<std::io::Error> for WalkerError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<PathError> for WalkerError {
    fn from(value: PathError) -> Self {
        Self::Path(value)
    }
}

pub fn walk_filtered(root: &Path, matcher: &CompiledMatcher) -> Result<Vec<FsEntry>, WalkerError> {
    let mut entries = Vec::new();
    walk_recursive(root, root, matcher, &mut entries)?;
    Ok(entries)
}

fn walk_recursive(
    root: &Path,
    current: &Path,
    matcher: &CompiledMatcher,
    entries: &mut Vec<FsEntry>,
) -> Result<(), WalkerError> {
    let mut children = Vec::new();

    for entry_result in fs::read_dir(current)? {
        let entry = entry_result?;
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path)?;

        if metadata.file_type().is_symlink() {
            continue;
        }

        let is_dir = metadata.is_dir();
        let relative_path = normalize_relative_path(root, &path)?;

        if !should_include(&path, &relative_path, is_dir, matcher) {
            continue;
        }

        let name = display_name(&path).unwrap_or_else(|| relative_path.clone());
        let kind = if is_dir {
            EntryKind::Directory
        } else {
            EntryKind::File
        };

        children.push((FsEntry {
            relative_path,
            name, 
            kind, 

        }, path));
    }

    sort_entries(&mut children);

    for (fs_entry, path) in children {
        let is_dir = matches!(fs_entry.kind, EntryKind::Directory);

        entries.push(fs_entry);

        if is_dir {
            walk_recursive(root, &path, matcher, entries)?;
        }
    }

    Ok(())
}

fn sort_entries(entries: &mut [(FsEntry, std::path::PathBuf)]) {
    entries.sort_by(|a, b| match (&a.0.kind, &b.0.kind) {
        (EntryKind::Directory, EntryKind::File) => std::cmp::Ordering::Less,
        (EntryKind::File, EntryKind::Directory) => std::cmp::Ordering::Greater,
        _ => a.0.name.to_lowercase().cmp(&b.0.name.to_lowercase()),
    });
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};
    
    use crate::matcher::compiler::compile_matcher;

    use super::*;

    fn create_temp_dir() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let dir = std::env::temp_dir().join(format!("tree_it_walk_test_{unique}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn walks_visible_entries_only() {
        let dir = create_temp_dir();

        fs::create_dir_all(dir.join("src")).unwrap();
        fs::write(dir.join("src").join("main.rs"), "fn main() {}").unwrap();
        fs::write(dir.join(".env"), "secret").unwrap();

        let matcher = compile_matcher(&[]).unwrap();
        let entries = walk_filtered(&dir, &matcher).unwrap();

        let relative_paths: Vec<String> = entries.iter().map(|e| e.relative_path.clone()).collect();

        assert!(relative_paths.contains(&"src".to_string()));
        assert!(relative_paths.contains(&"src/main.rs".to_string()));
        assert!(!relative_paths.contains(&".env".to_string()));

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn prunes_ignored_directory() {
        let dir = create_temp_dir();

        fs::create_dir_all(dir.join("target")).unwrap();
        fs::write(dir.join("target").join("output.txt"), "ignored").unwrap();
        fs::create_dir_all(dir.join("src")).unwrap();

        let matcher = compile_matcher(&["target/".to_string()]).unwrap();
        let entries = walk_filtered(&dir, &matcher).unwrap();

        let relative_paths: Vec<String> = entries.iter().map(|e| e.relative_path.clone()).collect();

        assert!(relative_paths.contains(&"src".to_string()));
        assert!(!relative_paths.contains(&"target".to_string()));
        assert!(!relative_paths.contains(&"target/output.txt".to_string()));

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn sorts_directories_before_files_case_insensitively() {
        let dir = create_temp_dir();

        fs::create_dir_all(dir.join("b_dir")).unwrap();
        fs::create_dir_all(dir.join("A_dir")).unwrap();
        fs::write(dir.join("z_file.txt"), "z").unwrap();
        fs::write(dir.join("M_file.txt"), "m").unwrap();

        let matcher = compile_matcher(&[]).unwrap();
        let entries = walk_filtered(&dir, &matcher).unwrap();

        let root_entries: Vec<String> = entries
            .iter()
            .filter(|e| !e.relative_path.contains('/'))
            .map(|e| e.name.clone())
            .collect();

        assert_eq!(
            root_entries,
            vec![
                "A_dir".to_string(),
                "b_dir".to_string(),
                "M_file.txt".to_string(),
                "z_file.txt".to_string(),
            ]
        );

        fs::remove_dir_all(dir).unwrap();
    }
}
use std::collections::BTreeMap;
use std::sync::Arc;

use axfs::overlayfs::OverlayFileSystem;
use axfs_ramfs::RamFileSystem;
use axfs_vfs::{VfsDirEntry, VfsError, VfsNodePerm, VfsNodeRef, VfsNodeType, VfsOps};

fn create_dir(root: &VfsNodeRef, path: &str) {
    root.create(path, VfsNodeType::Dir).unwrap();
}

fn create_file(root: &VfsNodeRef, path: &str, data: &[u8]) {
    root.create(path, VfsNodeType::File).unwrap();
    let file = root.clone().lookup(path).unwrap();
    assert_eq!(file.write_at(0, data).unwrap(), data.len());
}

fn read_all(root: &VfsNodeRef, path: &str) -> Vec<u8> {
    let file = root.clone().lookup(path).unwrap();
    let size = file.get_attr().unwrap().size() as usize;
    let mut data = vec![0; size];
    assert_eq!(file.read_at(0, &mut data).unwrap(), size);
    data
}

fn read_dir(root: &VfsNodeRef, path: &str) -> BTreeMap<String, VfsNodeType> {
    let dir = root.clone().lookup(path).unwrap();
    let mut out = BTreeMap::new();
    let mut start = 0;
    loop {
        let mut entries: [VfsDirEntry; 3] = std::array::from_fn(|_| VfsDirEntry::default());
        let count = dir.read_dir(start, &mut entries).unwrap();
        for entry in &entries[..count] {
            let name = std::str::from_utf8(entry.name_as_bytes())
                .unwrap()
                .to_string();
            out.insert(name, entry.entry_type());
        }
        start += count;
        if count < entries.len() {
            break;
        }
    }
    out
}

#[test]
fn readonly_lower_has_general_copy_up_and_whiteout_semantics() {
    axtask::init_scheduler();

    let lower: Arc<dyn VfsOps> = Arc::new(RamFileSystem::new());
    let lower_root = lower.root_dir();
    create_dir(&lower_root, "work");
    create_file(&lower_root, "work/source.txt", b"lower-source");
    create_file(&lower_root, "work/rename-src.txt", b"rename-source");
    create_file(&lower_root, "work/rename-dst.txt", b"old-destination");
    create_file(&lower_root, "work/unlink-open.txt", b"open-lower");
    create_dir(&lower_root, "tree");
    create_dir(&lower_root, "tree/nested");
    create_file(&lower_root, "tree/nested/input.rs", b"fn lower() {}\n");

    let overlay = OverlayFileSystem::new(lower.clone());
    let root = overlay.root_dir();

    assert_eq!(read_all(&root, "/work/source.txt"), b"lower-source");
    let copied = root.clone().lookup("/work/source.txt").unwrap();
    assert!(copied.parent().is_none());
    assert_eq!(copied.write_at(6, b"UPPER").unwrap(), 5);
    assert_eq!(read_all(&root, "/work/source.txt"), b"lower-UPPERe");
    assert_eq!(read_all(&lower_root, "/work/source.txt"), b"lower-source");

    root.create("/work/target", VfsNodeType::Dir).unwrap();
    root.create("/work/target/output.o", VfsNodeType::File)
        .unwrap();
    let output = root.clone().lookup("/work/target/output.o").unwrap();
    output
        .set_perm(VfsNodePerm::from_bits_truncate(0o640))
        .unwrap();
    assert_eq!(output.write_at(0, b"artifact").unwrap(), 8);
    assert_eq!(read_all(&root, "/work/target/output.o"), b"artifact");
    assert_eq!(
        output.get_attr().unwrap().perm().mode(),
        VfsNodePerm::from_bits_truncate(0o640).mode()
    );

    let names = read_dir(&root, "/work");
    assert_eq!(names.get("."), Some(&VfsNodeType::Dir));
    assert_eq!(names.get(".."), Some(&VfsNodeType::Dir));
    assert_eq!(names.get("source.txt"), Some(&VfsNodeType::File));
    assert_eq!(names.get("target"), Some(&VfsNodeType::Dir));
    assert_eq!(names.keys().filter(|name| *name == "source.txt").count(), 1);
    assert!(root.clone().lookup("/work").unwrap().parent().is_some());

    assert_eq!(
        root.rename("/work/source.txt", "/work/target"),
        Err(VfsError::IsADirectory)
    );
    assert_eq!(
        root.rename("/work/target", "/work/source.txt"),
        Err(VfsError::NotADirectory)
    );
    assert_eq!(
        root.rename("/tree", "/work/target"),
        Err(VfsError::DirectoryNotEmpty)
    );
    assert_eq!(root.rename("/missing", "/missing"), Err(VfsError::NotFound));

    root.remove("/work/source.txt").unwrap();
    assert!(matches!(
        root.clone().lookup("/work/source.txt"),
        Err(VfsError::NotFound)
    ));
    assert_eq!(read_all(&lower_root, "/work/source.txt"), b"lower-source");

    let truncate = root.clone().lookup("/work/rename-src.txt").unwrap();
    let rename_peer = root.clone().lookup("/work/rename-src.txt").unwrap();
    truncate.truncate(0).unwrap();
    assert_eq!(read_all(&root, "/work/rename-src.txt"), b"");
    assert_eq!(
        read_all(&lower_root, "/work/rename-src.txt"),
        b"rename-source"
    );
    assert_eq!(truncate.write_at(0, b"rename-source").unwrap(), 13);
    let mut open_data = [0; 13];
    assert_eq!(rename_peer.read_at(0, &mut open_data).unwrap(), 13);
    assert_eq!(&open_data, b"rename-source");

    root.rename("/work/rename-src.txt", "/work/rename-dst.txt")
        .unwrap();
    assert!(matches!(
        root.clone().lookup("/work/rename-src.txt"),
        Err(VfsError::NotFound)
    ));
    assert_eq!(read_all(&root, "/work/rename-dst.txt"), b"rename-source");
    assert_eq!(
        read_all(&lower_root, "/work/rename-dst.txt"),
        b"old-destination"
    );
    open_data.fill(0);
    assert_eq!(truncate.read_at(0, &mut open_data).unwrap(), 13);
    assert_eq!(&open_data, b"rename-source");
    assert_eq!(rename_peer.write_at(0, b"moved-").unwrap(), 6);
    assert_eq!(read_all(&root, "/work/rename-dst.txt"), b"moved--source");
    root.remove("/work/rename-dst.txt").unwrap();
    assert!(matches!(
        root.clone().lookup("/work/rename-dst.txt"),
        Err(VfsError::NotFound)
    ));
    assert_eq!(truncate.write_at(6, b"DETACHED").unwrap(), 8);
    open_data.fill(0);
    assert_eq!(rename_peer.read_at(0, &mut open_data).unwrap(), 13);
    assert_eq!(&open_data, b"moved-DETACHE");

    let open_lower = root.clone().lookup("/work/unlink-open.txt").unwrap();
    root.remove("/work/unlink-open.txt").unwrap();
    let mut lower_data = [0; 10];
    assert_eq!(open_lower.read_at(0, &mut lower_data).unwrap(), 10);
    assert_eq!(&lower_data, b"open-lower");
    assert_eq!(open_lower.write_at(5, b"UPPER").unwrap(), 5);
    lower_data.fill(0);
    assert_eq!(open_lower.read_at(0, &mut lower_data).unwrap(), 10);
    assert_eq!(&lower_data, b"open-UPPER");
    assert!(matches!(
        root.clone().lookup("/work/unlink-open.txt"),
        Err(VfsError::NotFound)
    ));

    let open_nested_dir = root.clone().lookup("/tree/nested").unwrap();
    root.rename("/tree", "/moved-tree").unwrap();
    assert!(matches!(
        root.clone().lookup("/tree"),
        Err(VfsError::NotFound)
    ));
    assert_eq!(
        read_all(&root, "/moved-tree/nested/input.rs"),
        b"fn lower() {}\n"
    );
    assert_eq!(
        read_all(&lower_root, "/tree/nested/input.rs"),
        b"fn lower() {}\n"
    );
    assert_eq!(read_all(&open_nested_dir, "input.rs"), b"fn lower() {}\n");
}

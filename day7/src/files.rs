#[derive(Debug, Copy, Clone)]
struct FileReference(usize);

#[derive(Debug)]
pub struct DirectoryDescriptor {
    pub name: String,
    children: Vec<FileReference>,
}

#[derive(Debug)]
pub enum FileItem {
    File(String, u32),
    Directory(DirectoryDescriptor),
}

#[derive(Debug)]
pub struct FileTree {
    root: FileReference,
    all_files: Vec<FileItem>,
}

impl FileTree {
    pub fn builder() -> FileTreeBuilder {
        let all_files = vec![FileItem::Directory(DirectoryDescriptor {
            name: "/".to_string(),
            children: vec![],
        })];
        let root = FileReference(0);

        let tree = Self { all_files, root };
        FileTreeBuilder {
            tree,
            parents: vec![root],
            current_dir: root,
        }
    }

    pub fn traverse<T, F>(&self, traverser: F) -> Vec<(&FileItem, T)>
    where
        F: Fn(&FileItem, &Vec<T>) -> T,
    {
        let mut results = vec![];
        let result = self.traverse_inner(self.root, &mut results, &traverser);
        results.push((self.get_item(self.root), result));
        results.reverse();
        results
    }

    fn traverse_inner<'a, T, F>(
        &'a self,
        current: FileReference,
        results: &mut Vec<(&'a FileItem, T)>,
        traverser: &F,
    ) -> T
    where
        F: Fn(&FileItem, &Vec<T>) -> T,
    {
        let mut vec_children = vec![];
        let mut vec_results = vec![];

        let current = self.get_item(current);

        if let FileItem::Directory(DirectoryDescriptor { children, .. }) = current {
            for child in children {
                let child_item = self.get_item(*child);
                let result = self.traverse_inner(*child, results, traverser);
                vec_children.push(child_item);
                vec_results.push(result);
            }
        }

        let result = traverser(current, &vec_results);

        if !vec_results.is_empty() {
            let mut vec = vec![];
            for _ in 0..vec_results.len() {
                let result = vec_results.pop().unwrap();
                let child = vec_children.pop().unwrap();
                vec.push((child, result));
            }
            vec.reverse();
            results.append(&mut vec);
        }
        result
    }

    fn get_item(&self, reference: FileReference) -> &FileItem {
        &self.all_files[reference.0]
    }

    fn get_item_mut(&mut self, reference: FileReference) -> &mut FileItem {
        &mut self.all_files[reference.0]
    }
}

pub struct FileTreeBuilder {
    tree: FileTree,
    current_dir: FileReference,
    parents: Vec<FileReference>,
}

impl FileTreeBuilder {
    pub fn cd_root(&mut self) {
        self.current_dir = self.tree.root;
    }

    pub fn cd_parent(&mut self) {
        self.current_dir = self.parents[self.current_dir.0];
    }

    pub fn cd(&mut self, name: &str) {
        let current_dir_item = self.tree.get_item(self.current_dir);
        if let FileItem::Directory(DirectoryDescriptor { children, .. }) = current_dir_item {
            for child in children {
                if let FileItem::Directory(DirectoryDescriptor {
                    name: child_name, ..
                }) = self.tree.get_item(*child)
                {
                    if child_name == name {
                        self.current_dir = *child;
                        return;
                    }
                }
            }
        }
    }

    pub fn mkdir(&mut self, name: &str) {
        let item = FileItem::Directory(DirectoryDescriptor {
            name: name.to_string(),
            children: vec![],
        });
        self.add_item_to_current_dir(item)
    }

    pub fn touch(&mut self, name: &str, size: u32) {
        let item = FileItem::File(name.to_string(), size);
        self.add_item_to_current_dir(item)
    }

    fn add_item_to_current_dir(&mut self, item: FileItem) {
        let reference = FileReference(self.tree.all_files.len());
        self.tree.all_files.push(item);
        self.parents.push(self.current_dir);
        if let FileItem::Directory(DirectoryDescriptor { children, .. }) =
            self.tree.get_item_mut(self.current_dir)
        {
            children.push(reference);
        }
    }

    pub fn build(self) -> FileTree {
        self.tree
    }
}

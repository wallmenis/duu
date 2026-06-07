use std::{collections::HashMap, path::PathBuf, fs::Metadata, os::unix::fs::MetadataExt};

use serde::Serialize;

use crate::{tree::Tree, utils::{get_sizes_recursive, walker}};

#[derive(Serialize,Clone)]
pub struct FlatpakDir
{
    pub label : String,
    pub path : PathBuf,
    pub apps : HashMap<String,u64>,
    pub runtimes : HashMap<String,u64>
}

impl FlatpakDir 
{
    pub fn new() -> Self
    {
        FlatpakDir {
            label : String::from("system"), 
            path: PathBuf::from("/var/lib/flatpak"),
            apps: HashMap::new(),
            runtimes : HashMap::new()
        }
    }
    
    pub fn find_sizes(&mut self)
    {
        let hm = walker(&self.path, Some(true) );
        let t = &mut Tree::new();
        for i in &hm
        {
            Tree::make_tree_from_path(t, i.0, i.1.len() );
        }
        self.find_sizes_with_tree_and_hm(&hm , t);
    }
    
    pub fn find_sizes_with_tree_and_hm(&mut self,  hm : &HashMap<PathBuf,Metadata>,t : &Tree)
    {
        if !t.check_if_contains(&self.path)
        {
            return;
        }
        let apps_path = self.path.join("app");
        let runtimes_path = self.path.join("runtime");
        let apps = t.get_children_as_pathbuf(&apps_path);
        let runtimes = t.get_children_as_pathbuf(&runtimes_path);
        for i in &apps
        {
            self.apps.insert(i.file_name().unwrap_or_default().display().to_string(), get_sizes_recursive(hm, t, i));
        }
        for i in &runtimes
        {
             self.runtimes.insert(i.file_name().unwrap_or_default().display().to_string(), get_sizes_recursive(hm, t, i));
        }
       
    }
}
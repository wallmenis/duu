use std::{collections::HashMap, path::PathBuf, fs::Metadata};

use serde::Serialize;

use crate::{tree::Tree, utils::{get_sizes_recursive_hash_map, walker, walker_tree}};

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
    
    pub fn new_with_params(l : &String, p : &PathBuf) -> Self
    {
        FlatpakDir {
            label : l.clone(), 
            path: p.clone(),
            apps: HashMap::new(),
            runtimes : HashMap::new()
        }
    }
    
    pub fn find_sizes(&mut self)
    {
        let hm = walker(&self.path, true);
        let t = &mut Tree::new();
        t.build_from_hash_map_only_leaf(&hm);
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
            self.apps.insert(i.file_name().unwrap_or_default().display().to_string(), get_sizes_recursive_hash_map(hm, t, i));
        }
        for i in &runtimes
        {
             self.runtimes.insert(i.file_name().unwrap_or_default().display().to_string(), get_sizes_recursive_hash_map(hm, t, i));
        }
       
    }
    
    pub fn find_sizes_using_tree(&mut self)
    {
        let t = walker_tree(&self.path, true );
        self.find_sizes_with_tree(&t);
    }
    
    pub fn find_sizes_with_tree(&mut self,  t : &Tree)
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
            self.apps.insert(i.file_name().unwrap_or_default().display().to_string(), t.get_child_ref(i).unwrap().size);
        }
        for i in &runtimes
        {
            self.runtimes.insert(i.file_name().unwrap_or_default().display().to_string(), t.get_child_ref(i).unwrap().size);
        }
        
    }
    
    pub fn print(&self, size_div : u64)
    {
        println!("------------------------");
        println!("Flatpak directory type : {}", self.label);
        println!("Flatpak directory path : {}", self.path.display());
        println!("Apps :");
        for i in &self.apps
        {
            println!("\t{} : {}", i.0, i.1/size_div);
        }
        println!("Runtimes :");
        for i in &self.runtimes
        {
            println!("\t{} : {}", i.0, i.1/size_div);
        }
    }
}
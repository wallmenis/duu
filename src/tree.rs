use std::{collections::{HashMap, HashSet}, fs::Metadata, path::PathBuf};
//use std::sync::{Mutex,Arc};
use serde::{Serialize, Deserialize};

use crate::utils::{get_parent_dirs, get_sizes_recursive};

#[derive(Clone,Serialize,Deserialize)]
pub struct Tree{
    #[serde(rename="sub-dir")]
    pub hm : HashMap<String, Tree>,
    pub size : u64
}

impl Tree
{
    
    pub fn new() -> Self
    {
        Tree {
            hm : HashMap::new(),
            size : 0
        }
    }
    
    pub fn get_mut_tree(&mut self, s : &String) -> Option<&mut Tree>
    {
        let ret = self.hm.get_mut(s);
        return ret;
    }
    
    pub fn make_tree_from_path(&mut self, pth : &PathBuf, l : u64)
    {
        let c : Vec<_> = pth.components().collect();
        let mut current = self;
        for i in &c
        {
            let k = i.as_os_str().to_os_string().display().to_string(); // I know this is cursed but bare with me here!
            if !current.hm.contains_key(&k)
            {
                current.hm.insert(k.clone(),Tree::new());
            }
            current = current.get_mut_tree(&k).expect("I don't know how this happened!"); // It is not likely to break. I think there is a more efficient way to do the creation.
        }
        current.size = l;
    }
    
    pub fn check_if_contains(&self, pth : &PathBuf) -> bool
    {
        self.get_child_ref(pth).is_some()
    }
    
    pub fn get_child(&self, start : &PathBuf) -> Option<Tree>
    {
        let mut current = self;
        let c : Vec<_> = start.components().collect();
        
        for i in &c
        {
            let k = i.as_os_str().to_os_string().display().to_string();
            if current.hm.contains_key(&k)
            {
                current = &current.hm[&k];
            }
            else {
                return None;
            }
        }
        Some(current.clone())
    }
    
    pub fn get_child_ref(&self, start : &PathBuf) -> Option<&Tree>
    {
        let mut current = self;
        let c : Vec<_> = start.components().collect();
        
        for i in &c
        {
            let k = i.as_os_str().to_os_string().display().to_string();
            if current.hm.contains_key(&k)
            {
                current = &current.hm[&k];
            }
            else {
                return None;
            }
        }
        Some(current)
    }
    
    pub fn get_child_mut_ref(&mut self, start : &PathBuf) -> Option<&mut Tree>
    {
        let mut current = self;
        let c : Vec<_> = start.components().collect();
        
        for i in &c
        {
            let k = i.as_os_str().to_os_string().display().to_string();
            if current.hm.contains_key(&k)
            {
                current = current.get_mut_tree(&k).expect("Already checked");
            }
            else {
                return None;
            }
        }
        Some(current)
    }
    
    pub fn get_children_as_pathbuf(&self, start:&PathBuf) -> Vec<PathBuf>
    {
        let mut v = Vec::new();
        
        if self.check_if_contains(start)
        {
            for i in self.get_child(start).expect("I don't know how this crashed, it is already checked!!!").hm     //already checked before
            {
                v.push(start.join(i.0));
            }
        }
        v
    }
    
    //pub fn build_from_hash_map(&mut self, hm : &HashMap<PathBuf,Metadata>)
    pub fn build_from_hash_map(&mut self, hm : &HashMap<PathBuf,Metadata>)
    {
        // Need to make multithreaded for performance
//         let mhs : Arc<Mutex<HashSet<PathBuf>>>= Arc::new(Mutex::new(HashSet::new()));
//         for i in hm
//         {
//             self.make_tree_from_path(i.0,i.1.len() );
//             let pb = i.0.clone();
//             let mt = i.1.clone();
//             std::thread::spawn( || {
//                 let paths = get_parent_dirs(pb);
//                     for j in &paths
//                     {
//                         if !mhs.lock().unwrap().contains(j)
//                         {
//                             mhs.insert(j.clone());
//                             self.get_child_mut_ref(j).expect("Ah hell nahh").size = get_sizes_recursive(hm,self ,j );
//                         }
//                     }
//                 });
//             
//         }
        
        //println!("\tmaking leaves");
        self.build_from_hash_map_only_leaf(hm);
        let mut hs : HashSet<PathBuf>= HashSet::new();
        //println!("\tmarking paths");
        for i in hm
        {
            let paths : HashSet<PathBuf> = get_parent_dirs(i.0);
            hs.extend(paths);
        }
        println!("\tmarking sizes");
        for j in &hs
        {
            self.get_child_mut_ref(j).expect("Ah hell nahh").size = get_sizes_recursive(hm,self ,j );
        }
    }
    
    pub fn build_from_hash_map_only_leaf(&mut self, hm : &HashMap<PathBuf,Metadata>)
    {
        for i in hm
        {
            self.make_tree_from_path(i.0,i.1.len() );
        }
    }
    
}
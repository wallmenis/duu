use std::{collections::{HashMap, HashSet}, fs::Metadata, os::unix::fs::MetadataExt, path::PathBuf};
//use std::sync::{Mutex,Arc};
use serde::{Serialize, Deserialize};

use crate::utils::{get_parent_dirs, get_sizes_recursive_hash_map, inode_sizes};

use crate::sizes::UNIX_BLOCK_SIZE;

#[derive(Clone,Serialize,Deserialize)]
pub struct Tree{
    #[serde(rename="sub-dir")]
    pub hm : HashMap<String, Tree>,
    pub size : u64,
    #[serde(skip)]
    pub inodes : HashSet<[u64;2]>
}

impl Tree
{
    
    pub fn new() -> Self
    {
        Tree {
            hm : HashMap::new(),
            size : 0,
            inodes : HashSet::new()
        }
    }
    
    pub fn get_mut_tree(&mut self, s : &String) -> Option<&mut Tree>
    {
        let ret = self.hm.get_mut(s);
        return ret;
    }
    
    pub fn get_hm_keys(&self) -> HashSet<String>
    {
        let mut hs = HashSet::new();
        for i in &self.hm
        {
            hs.insert(i.0.clone());
        }
        hs
    }
    
    pub fn make_tree_from_path(&mut self, pth : &PathBuf, l : u64, ino : [u64; 2])
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
        current.inodes.insert(ino);
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
    
    pub fn get_children_as_pathbuf(&self, start:&PathBuf) -> HashSet<PathBuf>
    {
        let mut v = HashSet::new();
        
        if self.check_if_contains(start)
        {
            for i in &self.get_child_ref(start).expect("I don't know how this crashed, it is already checked!!!").hm     //already checked before
            {
                v.insert(start.join(i.0));
            }
        }
        v
    }
    
    pub fn build_from_hash_map(&mut self, hm : &HashMap<PathBuf,Metadata>)
    {
        //println!("\tmaking leaves");
        self.build_from_hash_map_only_leaf(hm);
        let ino = inode_sizes(hm);
        let mut hs : HashSet<PathBuf>= HashSet::new();
        //println!("\tmarking paths");
        for i in hm
        {
            let paths : HashSet<PathBuf> = get_parent_dirs(i.0);
            hs.extend(paths);
        }
        //println!("\tmarking sizes");
        // for j in &hs
        // {
        //     self.get_child_mut_ref(j).expect("Ah hell nahh").size = get_sizes_recursive_hash_map(hm,self ,j );
        // }
        self.get_sizes(&ino);
    }
    
    pub fn get_sizes(&mut self, inodes : &HashMap<[u64;2],u64>)-> &mut Tree
    {
        let hs = &self.get_hm_keys();
        let mut ino : Vec<HashSet<[u64;2]>> = Vec::new();
        let mut sum = 0;
        let mut now = &mut Tree::new();
        for i in hs
        {
             now = self.get_mut_tree(&i).expect("There wouldn't be any hm entries in the original tree for it to reach this").get_sizes(inodes);
              //   += self.get_mut_tree(&i).expect("There wouldn't be any hm entries in the original tree for it to reach this").get_sizes(inodes).size;
             sum += now.size;
             ino.push(now.inodes.clone());
        }
        sum -= Tree::get_size_for_removal(&ino,inodes );
        
        if hs.is_empty()
        {
            return self;
        }
        self.size = sum;
        self
    }
    
    fn get_size_for_removal(ino : &Vec<HashSet<[u64;2]>> ,inodes : &HashMap<[u64;2],u64>) -> u64
    {
        let mut all : HashSet<[u64;2]> = HashSet::new();
        let mut all_with_multiplicities = HashMap::new();
        for i in ino
        {
            all.extend(i);
            for j in i
            {
                *all_with_multiplicities.entry(j).or_insert(0) += 1;
            }
        }
        for i in &all
        {
            *all_with_multiplicities.get_mut(i).expect("This will crash if all is not a copy of the keys of all_with_multiplicities") -= 1;
        }
        for i in &all
        {
            if all_with_multiplicities[i] <= 0
            {
                all_with_multiplicities.remove(i);
            }
        }
        let mut sum = 0;
        for i in all_with_multiplicities.keys()
        {
            sum += inodes[*i];
        }
        sum
    }
    
    
    pub fn build_from_hash_map_only_leaf(&mut self, hm : &HashMap<PathBuf,Metadata>)
    {
        for i in hm
        {
            let mut siz = UNIX_BLOCK_SIZE;
            match nix::sys::statvfs::statvfs(i.0)
            {
                Ok(o) => {siz = o.fragment_size();},
                Err(e) => {eprintln!("file {} modified on scanning: {}", i.0.display(), e);}
            };
            self.make_tree_from_path(i.0,i.1.blocks()*siz, [i.1.dev(), i.1.ino()]);
        }
    }
    
}
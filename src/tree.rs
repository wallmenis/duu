use std::{collections::HashMap, path::{PathBuf}};
use serde::{Serialize, Deserialize};

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
    
    pub fn make_tree_from_path(t : &mut Tree, pth : &PathBuf, l : u64)
    {
        let c : Vec<_> = pth.components().collect();
        //let s = pth.display().to_string();
        //let p : Vec<_> = s.split('/').collect();
        let mut current = t;
        //let new = &mut Tree::new();
        for i in &c
        {
            let k = i.as_os_str().to_os_string().display().to_string(); // I know this is cursed but bare with me here!
            if !current.hm.contains_key(&k)
            {
                current.hm.insert(k.clone(),Tree::new());
            }
            current = current.get_mut_tree(&k).expect("I don't know how this happened!"); // It is not likely to break. I think there is a more efficient way to do the creation.
            //current = current.get_mut_tree(i.to_string()).unwrap_or(new);
        }
        current.size = l;
    }
    
    pub fn check_if_contains(&self, pth : &PathBuf) -> bool
    {
        self.get_child(pth).is_some()
    }
    
    pub fn get_child(&self, start : &PathBuf) -> Option<Tree>
    {
        let mut current = self;
        let c : Vec<_> = start.components().collect();
        // let s = start.display().to_string();
        // let p : Vec<_> = s.split('/').collect();
        
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
    
    pub fn get_children_as_pathbuf(&self, start:&PathBuf) -> Vec<PathBuf>
    {
        let mut v = Vec::new();
        
        if self.check_if_contains(start)
        {
            for i in self.get_child(start).expect("I don't know how this crashed, it is already checked!!!").hm     //already checked before
            {
                //v.push(PathBuf::from(start.display().to_string() + "/" + i.0.as_str()));
                v.push(start.join(i.0));
            }
        }
        v
    }
    
}
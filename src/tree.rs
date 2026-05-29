
use std::{collections::HashMap, path::{PathBuf}};

#[derive(Clone)]
pub struct Tree{
    pub hm : HashMap<String, Tree>,
}

impl Tree
{
    
    pub fn new() -> Self
    {
        Tree {
            hm : HashMap::new(),
        }
    }
    
    pub fn get_mut_tree(&mut self, s : String) -> Option<&mut Tree>
    {
        let ret = self.hm.get_mut(&s);
        return ret;
    }
    
    pub fn make_tree_from_path(t : &mut Tree, pth : &PathBuf)
    {
        let s = pth.display().to_string();
        let p : Vec<_> = s.split('/').collect();
        let mut current = t;
        //let new = &mut Tree::new();
        for i in &p
        {
            if !current.hm.contains_key(*i)
            {
                current.hm.insert(i.to_string(),Tree::new());
            }
            current = current.get_mut_tree(i.to_string()).unwrap(); // It is not likely to break. I think there is a more efficient way to do the creation.
            //current = current.get_mut_tree(i.to_string()).unwrap_or(new);
            
        }
    }
    #[allow(dead_code)]
    pub fn print(&self)
    {
        if !self.hm.is_empty()
        {
            for i in &self.hm
            {
                print!("/{}",i.0);
                i.1.print();
                println!();
            }
            
        }
    }
    
    pub fn check_if_contains(&self, pth : &PathBuf) -> bool
    {
        match self.get_leaf(pth)
        {
            Some(_)=>true,
            None => false
        }
    }
    
    pub fn get_leaf(&self, start : &PathBuf) -> Option<Tree>
    {
        let mut current = self;
        let s = start.display().to_string();
        let p : Vec<_> = s.split('/').collect();
        
        for i in &p
        {
            if current.hm.contains_key(*i)
            {
                current = &current.hm[*i];
            }
            else {
                return None;
            }
        }
        Some(current.clone())
    }
    
    pub fn get_leaves_as_pathbuf(&self, start:&PathBuf) -> Vec<PathBuf>
    {
        let mut v = Vec::new();
        
        if self.check_if_contains(start)
        {
            for i in self.get_leaf(start).unwrap().hm     //already checked before
            {
                //v.push(PathBuf::from(start.display().to_string() + "/" + i.0.as_str()));
                v.push(start.join(i.0));
            }
        }
        v
    }
}
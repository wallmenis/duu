
// use serde::Deserialize;
// use std::io::{BufReader, Read};

//use std::os::unix::net::UnixStream;

use std::{collections::HashMap, fs::Metadata, os::unix::fs::MetadataExt, path::{PathBuf, Path}};

use jwalk::WalkDir;
use sysinfo::Disks;


#[derive(Clone)]
struct Tree{
  hm : HashMap<String, Tree>,
}

impl Tree
{
  
  fn new() -> Self
  {
    Tree {
      hm : HashMap::new(),
    }
  }
  
  fn get_mut_tree(&mut self, s : String) -> Option<&mut Tree>
  {
    let ret = self.hm.get_mut(&s);
    return ret;
  }
  
  fn make_tree_from_path(t : &mut Tree, pth : &PathBuf)
  {
    let s = pth.to_str();
    let p : Vec<_> = s.unwrap().split('/').collect();
    let mut current = t;
    for i in &p
    {
      if !current.hm.contains_key(*i)
      {
        current.hm.insert(i.to_string(),Tree::new());
      }
      current = current.get_mut_tree(i.to_string()).unwrap();
    }
  }
  #[allow(dead_code)]
  fn print(&self)
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
  
  fn check_if_contains(&self, pth : PathBuf) -> bool
  {
    match self.get_leaf(pth)
    {
      Some(_)=>true,
      None => false
    }
  }
  
  fn get_leaf(&self, start : PathBuf) -> Option<Tree>
  {
    let mut current = self;
    let s = start.to_str();
    let p : Vec<_> = s.unwrap().split('/').collect();
    
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
  
  fn get_leaves_as_pathbuf(&self, start:PathBuf) -> Vec<PathBuf>
  {
    let mut v =  Vec::new();
    
    if self.check_if_contains(start.clone())
    {
      for i in self.get_leaf(start.clone()).unwrap().hm
      {
        v.push(PathBuf::from(start.display().to_string() + "/" + i.0.as_str()));
      }
    }
    v
  }
}


fn get_ult_parent(pth : &PathBuf) -> PathBuf
{
  let mut f = std::fs::metadata(pth).unwrap();
  let d = f.dev();
  let mut now : PathBuf = pth.clone();
  let mut prev : PathBuf = pth.clone();
  
  while f.dev() == d && prev != PathBuf::from("/")
  {
    prev = now.clone();
    now = match now.parent(){
          Some(o) => o,
          None => Path::new("/")
        }
        .to_path_buf();
    f = std::fs::metadata(&now).unwrap();
  }
  
  prev.clone()
}

fn walker(p : PathBuf) -> HashMap<PathBuf, Metadata>
{
  let mut hm : HashMap<PathBuf, Metadata> = HashMap::new();
  let walk = WalkDir::new(p); //.parallelism(jwalk::Parallelism::Serial);
  let final_walk = walk.into_iter().filter_map(|pth| pth.ok());
  
  for pth in final_walk{
    let f = match std::fs::metadata(&pth.path()){
      Ok(fle) => fle,
      Err(_) =>{
        continue
      }
    };
    let l = f;
    if l.is_file()
    {
      hm.insert(pth.path().clone(),l.clone());
    }
  }
  hm
}

fn inode_deduplicator_single_path(hm : &HashMap<PathBuf, Metadata>) -> HashMap<[u64; 2],PathBuf>
{
  let mut inode_hm : HashMap<[u64; 2],PathBuf> = HashMap::new();
  for i in hm{
    let inode = [i.1.dev(),i.1.ino()];
    let path = i.0.clone();
    inode_hm.insert(inode,path);
  }
  inode_hm
}

#[allow(dead_code)]
fn inode_deduplicator(hm : &HashMap<PathBuf, Metadata>) -> HashMap<[u64; 2],Vec<PathBuf>>
{
  let mut inode_hm : HashMap<[u64; 2],Vec<PathBuf>> = HashMap::new();
  for i in hm{
    let inode = [i.1.dev(),i.1.ino()];
    let path = i.0.clone();
    let mut ino : Vec<PathBuf> = Vec::new();
    if inode_hm.contains_key(&[i.1.dev(),i.1.ino()])
    {
      ino = inode_hm[&inode].clone();
    }
    ino.push(path);
    inode_hm.insert(inode,ino);
  }
  inode_hm
}

fn inode_sizes(hm : &HashMap<PathBuf, Metadata>) -> HashMap<[u64; 2],u64> 
{
  let mut inode_size : HashMap<[u64; 2],u64> = HashMap::new();
  for i in hm{
    let inode = [i.1.dev(),i.1.ino()];
    inode_size.insert(inode,i.1.len());
    //inode_size.insert(inode,512*i.1.blocks());
  }
  inode_size
}

fn main(){
  
  let hm = walker(PathBuf::from("/"));
  let inode_size = inode_sizes(&hm);
  
  
  
  println!("Finished parsing the files");
  let inode_hm = inode_deduplicator_single_path(&hm);
  
  let a = &mut Tree::new();
  
  for i in &hm
  {
    Tree::make_tree_from_path(a, i.0); 
  }
  println!("Finished indexng the files");

  let mut dev_mnt : HashMap<u64,PathBuf> = HashMap::new();
  
  let mut sum_per_dev : HashMap<u64,u64> = HashMap::new();
  for i in &inode_size
  {
    if sum_per_dev.contains_key(&i.0[0])
    {
      sum_per_dev.insert(i.0[0],i.1+sum_per_dev[&i.0[0]]);
    }
    else {
      sum_per_dev.insert(i.0[0],i.1.clone());
    }
    if !dev_mnt.contains_key(&i.0[0])
    {
      dev_mnt.insert(i.0[0],inode_hm[i.0].clone());
    }
    
  }
  let mut final_dev_mnt = dev_mnt.clone();
  for i in &dev_mnt
  {
    final_dev_mnt.insert(i.0.clone(),get_ult_parent(&i.1));
  }
  
  println!("Finished counting the sizes");
  
  for i in &sum_per_dev
  {
    println!("{} {}", final_dev_mnt[i.0].display(), i.1/(1024*1024));
  }
  println!("----------");
  
  
    
  
  println!("{}", a.check_if_contains(PathBuf::from("/home/wallmenis")));
  
  let v = a.get_leaves_as_pathbuf(PathBuf::from("/home/wallmenis"));
  
  for i in &v
  {
    println!("{}", i.display());
    if hm.contains_key(i)
    {
      println!("{}",hm[i].len());
    }
  }
  
  
  println!("----------");
  for i in &Disks::new_with_refreshed_list()
  {
    //println!("{} {}",(i.usage().read_bytes)/(1024*1024),i.total_space()/(1024*1024));
    println!("{} {}",(i.total_space() - i.available_space())/(1024*1024),i.total_space()/(1024*1024));
  }
  
}

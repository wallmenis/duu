
// use serde::Deserialize;
// use std::io::{BufReader, Read};

//use std::os::unix::net::UnixStream;

use std::{collections::HashMap, path::PathBuf};

use sysinfo::Disks;


mod tree;
mod utils;
use tree::Tree;
use utils::*;




fn main(){
  
  let hm = walker(&PathBuf::from("/"), Some(true));
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
  
  
    
  
  println!("{}", a.check_if_contains(&PathBuf::from("/home/wallmenis")));
  
  let v = a.get_leaves_as_pathbuf(&PathBuf::from("/home/wallmenis"));
  
  
  
  for i in &v
  {
    println!("{}", i.display());
    println!("{}",get_sizes_recursive(&hm,a ,i)/(1024*1024));
  }
  
  
  println!("----------");
  for i in &Disks::new_with_refreshed_list()
  {
    //println!("{} {}",(i.usage().read_bytes)/(1024*1024),i.total_space()/(1024*1024));
    println!("{} {}",(i.total_space() - i.available_space())/(1024*1024),i.total_space()/(1024*1024));
  }
  
}

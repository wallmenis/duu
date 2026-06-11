use std::{collections::HashMap, os::unix::fs::MetadataExt, path::PathBuf};
use clap::Parser;
//use sysinfo::Disks;
use nix::unistd::{getuid, User};

mod flatpak;
mod arguments;
mod tree;
mod utils;
mod jsoning;

use tree::Tree;
use utils::*;
use flatpak::FlatpakDir;
use arguments::*;



fn main(){
  
  let args = Ar::parse();
  
  let s_path = PathBuf::from(args.path);
  let sup_path = match s_path.canonicalize()
  {
    Ok(o) => o,
    Err(e) =>
    {
      eprint!("{}",e);
      std::process::exit(1);
    }
  };
  
  let siz = string_to_data_size(&args.size).value();
  
  let hm = walker(&sup_path, Some(true));
  
  println!("Finished parsing the files");
  
  let a = &mut Tree::new();
  
  //a.build_from_hash_map_only_leaf(&hm);
  a.build_from_hash_map(&hm);
  
  println!("Finished indexng the files");
  
  if args.flatpak
  {
    let mut f = FlatpakDir::new();
    if a.check_if_contains(&f.path)
    {
      f.find_sizes_with_tree_and_hm(&hm ,a );
    }
    else
    {
      f.find_sizes();
    }

    if f.path.exists()
    {
      f.print(siz);
    }
    
    let user_dir = PathBuf::from(format!("/home/{}/.local/share/flatpak",User::from_uid(getuid()).unwrap().unwrap().name));
    
    let mut u = FlatpakDir::new();
    u.label = String::from("user");
    u.path = user_dir;
    if a.check_if_contains(&u.path)
    {
      u.find_sizes_with_tree_and_hm(&hm ,a );
    }
    else
    {
      u.find_sizes();
    }
    
    if u.path.exists()
    {
      u.print(siz);
    }
    
    let user_dir = PathBuf::from(format!("/home/{}/.var",User::from_uid(getuid()).unwrap().unwrap().name));
    
    let mut u = FlatpakDir::new();
    u.label = String::from("user-cache");
    u.path = user_dir;
    if a.check_if_contains(&u.path)
    {
      u.find_sizes_with_tree_and_hm(&hm ,a );
    }
    else
    {
      u.find_sizes();
    }
    
    if u.path.exists()
    {
      u.print(siz);
    }
    
  }
 
  println!("------------------------");
  // for i in &Disks::new_with_refreshed_list()
  // {
  //   println!("{} : {} out of {}",i.mount_point().display(),(i.total_space() - i.available_space())/siz,i.total_space()/siz);
  // }
  // let mut ihm : HashMap<u64, PathBuf> = HashMap::new();
  // for i in &hm
  // {
  //   ihm.insert(i.1.dev(), get_ult_parent(i.0));
  // }
  let f = std::fs::read_to_string("/proc/self/mounts").expect("This is not a linux environment");
  let ft : Vec<_> = f.lines().collect();
  for i in ft
  {
    let fs : Vec<_> = i.split_whitespace().collect();
    let filesystem = fs[0];
    let mountpoint = fs[1];
    let stats = nix::sys::statvfs::statvfs(mountpoint).expect("Failed to get stats");
    let mut d = 0;
    if stats.blocks() != 0
    {
      d = 100 - ((stats.blocks_free()*100)/stats.blocks());
    }
    if filesystem.len()>7
    {
      println!("{}\t {}\t\t {} {}%", filesystem,mountpoint, stats.blocks()*stats.fragment_size()/siz, d);
    }
    else {
      
      println!("{}\t\t {}\t\t {} {}%", filesystem,mountpoint, stats.blocks()*stats.fragment_size()/siz, d );
    }
    
  }
  //a.print_json();
}

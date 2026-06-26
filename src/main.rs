use std::{collections::HashMap, os::unix::fs::MetadataExt, path::PathBuf};
use clap::Parser;
//use sysinfo::Disks;
use nix::unistd::{getuid, User};

mod container;
mod df;
mod flatpak;
mod arguments;
mod tree;
mod utils;
mod jsoning;

use tree::Tree;
use utils::*;
use flatpak::FlatpakDir;
use arguments::*;

use df::DFEntry;



fn main(){
  
  let args = Ar::parse();
  let mut a = Tree::new();
  
  let siz = string_to_data_size(&args.size).value();
  println!("Sizes in {}", string_to_data_size(&args.size).to_string());
  
  
  if !args.path.is_empty()
  {
    println!("------------------------");
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
    
    a = walker_tree(&sup_path, true);
    a.print(args.depth,&sup_path, siz);
  }

  
  
  
  if args.flatpak
  {
    let mut f = FlatpakDir::new();
    if a.check_if_contains(&f.path)
    {
      f.find_sizes_with_tree(&a);
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
      u.find_sizes_with_tree(&a);
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
    
    let mut uc = FlatpakDir::new();
    u.label = String::from("user-cache");
    u.path = user_dir;
    if a.check_if_contains(&u.path)
    {
      uc.find_sizes_with_tree(&a);
    }
    else
    {
      uc.find_sizes();
    }
    
    if uc.path.exists()
    {
      uc.print(siz);
    }
    
  }
 
  println!("------------------------");
  
  let f = std::fs::read_to_string("/proc/self/mounts").expect("This is not a linux environment");
  let mut sizes_vec : Vec<DFEntry> = Vec::new();
  let ft : Vec<_> = f.lines().collect();
  for i in ft
  {
    let fs : Vec<_> = i.split_whitespace().collect();
    let filesystem = fs[0];
    let mountpoint = fs[1];
    let stats = nix::sys::statvfs::statvfs(mountpoint).expect("Failed to get stats");
    let mut s : DFEntry = DFEntry::new();
    s.filesystem = filesystem.to_string();
    s.path = PathBuf::from(mountpoint);
    s.blocks = stats.blocks();
    s.used = stats.blocks() - stats.blocks_free();
    s.available = stats.blocks_available();
    s.blk_sz = stats.fragment_size();
    s.sd = string_to_data_size(&args.size);
    sizes_vec.push(s);
  }
  DFEntry::df(&sizes_vec);
}

use std::{collections::HashMap, os::unix::fs::MetadataExt, path::PathBuf};
use clap::Parser;
//use sysinfo::Disks;
use nix::unistd::{getuid, User};

mod sizes;
mod flatpak;
mod arguments;
mod tree;
mod utils;
mod jsoning;

use tree::Tree;
use utils::*;
use flatpak::FlatpakDir;
use arguments::*;

use sizes::Size;



fn main(){
  
  let args = Ar::parse();
  let a = &mut Tree::new();
  let mut hm : HashMap<PathBuf, std::fs::Metadata> = HashMap::new();
  if !args.path.is_empty()
  {
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
    
    hm = walker(&sup_path, true);
    
    println!("Finished parsing the files");
    
    
    
    //a.build_from_hash_map_only_leaf(&hm);
    a.build_from_hash_map(&hm);
  }

  let siz = string_to_data_size(&args.size).value();
  
  
  println!("Finished indexng the files");
  
  println!("Sizes in {}", string_to_data_size(&args.size).to_string());
  
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
  let mut sizes_vec : Vec<Size> = Vec::new();
  let ft : Vec<_> = f.lines().collect();
  for i in ft
  {
    let fs : Vec<_> = i.split_whitespace().collect();
    let filesystem = fs[0];
    let mountpoint = fs[1];
    let stats = nix::sys::statvfs::statvfs(mountpoint).expect("Failed to get stats");
    let mut s : Size = Size::new();
    s.filesystem = filesystem.to_string();
    s.path = PathBuf::from(mountpoint);
    s.blocks = stats.blocks();
    s.used = stats.blocks() - stats.blocks_free();
    s.available = stats.blocks_available();
    s.blk_sz = stats.fragment_size();
    s.sd = string_to_data_size(&args.size);
    sizes_vec.push(s);
  }
  Size::df(&sizes_vec);
  //a.print_json();
}

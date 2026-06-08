use std::path::PathBuf;
use clap::Parser;
use sysinfo::Disks;
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
  
  let e_siz = string_to_data_size(&args.size);
  
  let siz = e_siz as u64;
  
  let hm = walker(&sup_path, Some(true));
  //let inode_size = inode_sizes(&hm);
  
  
  
  println!("Finished parsing the files");
 // let inode_hm = inode_deduplicator_single_path(&hm);
  
  let a = &mut Tree::new();
  
  a.build_from_hash_map_only_leaf(&hm);
  
  println!("Finished indexng the files");
  
  //a.build_from_hash_map(&hm);
  
  //println!("Finished indexng the files");

//   let mut dev_mnt : HashMap<u64,PathBuf> = HashMap::new();
//   
//   let mut sum_per_dev : HashMap<u64,u64> = HashMap::new();
//   for i in &inode_size
//   {
//     if sum_per_dev.contains_key(&i.0[0])
//     {
//       sum_per_dev.insert(i.0[0],i.1+sum_per_dev[&i.0[0]]);
//     }
//     else {
//       sum_per_dev.insert(i.0[0],i.1.clone());
//     }
//     if !dev_mnt.contains_key(&i.0[0])
//     {
//       dev_mnt.insert(i.0[0],inode_hm[i.0].clone());
//     }
//     
//   }
//   let mut final_dev_mnt = dev_mnt.clone();
//   for i in &dev_mnt
//   {
//     final_dev_mnt.insert(i.0.clone(),get_ult_parent(&i.1));
//   }
//   
//   println!("Finished counting the sizes");
//   
//   for i in &sum_per_dev
//   {
//     println!("{} {}", final_dev_mnt[i.0].display(), i.1/siz);
//   }
  
  
//   println!("{}", a.check_if_contains(&PathBuf::from("/home/wallmenis")));
//   
//   let v = a.get_children_as_pathbuf(&PathBuf::from("/home/wallmenis"));
  
  
  // for i in &v
  // {
  //   println!("{}", i.display());
  //   println!("{}",get_sizes_recursive(&hm,a ,i)/(siz));
  // }
  
  
  
  
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

    f.print(siz);
    
    let mut u = FlatpakDir::new();
    u.label = String::from("user");
    u.path = PathBuf::from(format!("/home/{}/.local/share/flatpak",User::from_uid(getuid()).unwrap().unwrap().name));
    if a.check_if_contains(&u.path)
    {
      u.find_sizes_with_tree_and_hm(&hm ,a );
    }
    else
    {
      u.find_sizes();
    }
    
    u.print(siz);
    
  }
 
  
  for i in &Disks::new_with_refreshed_list()
  {
    println!("{} : {} out of {}",i.mount_point().display(),(i.total_space() - i.available_space())/siz,i.total_space()/siz);
  }
  //a.print();
}

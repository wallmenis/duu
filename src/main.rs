
// use serde::Deserialize;
// use std::io::{BufReader, Read};

//use std::os::unix::net::UnixStream;

use std::{collections::HashMap, fs::Metadata, os::unix::fs::MetadataExt, path::{PathBuf, Path}};

use jwalk::WalkDir;
use sysinfo::Disks;

fn get_dev_mountpoint() -> HashMap<u64,String>
{
  let mut map : HashMap<u64, String> = HashMap::new();
  let m = std::fs::read_to_string("/proc/mounts").expect("Failed to read /proc/mounts");
  for i in m.lines()
  {
    let s : Vec<_> = i.split_whitespace().collect();
    let mt = match std::fs::metadata(s[1]){
      Ok(f) => f,
      Err(e) => {
        eprintln!("{}", e);
        continue
      }
    }; 
    map.insert(mt.dev(),s[1].to_string());
  }
  map
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

fn main(){
  
  let mut hm : HashMap<PathBuf, Metadata> = HashMap::new();
  let mut inode_hm : HashMap<[u64; 2],Vec<PathBuf>> = HashMap::new();
  let mut inode_size : HashMap<[u64; 2],u64> = HashMap::new();
  let walk = WalkDir::new("/"); //.parallelism(jwalk::Parallelism::Serial);
  let final_walk = walk.into_iter().filter_map(|pth| pth.ok());
  
  for pth in final_walk{ 
    //println!("Looking at {}", pth.path().display());
      let f = match std::fs::metadata(&pth.path()){
        Ok(fle) => fle,
        Err(_) =>{
          // if e.kind() == std::io::ErrorKind::PermissionDenied
          // {
          //   // eprintln!("Permission Denied on {}", pth.path().display());
          //   continue;
          // }
          continue
        }
      };
      let l = f;
      if l.is_file()
      {
        hm.insert(pth.path().clone(),l.clone());
      }
  }
  println!("Finished parsing the files");
  
  for i in &hm{
    let inode = [i.1.dev(),i.1.ino()];
    let path = i.0.clone();
    let mut ino : Vec<PathBuf> = Vec::new();
    if inode_hm.contains_key(&[i.1.dev(),i.1.ino()])
    {
      ino = inode_hm[&inode].clone();
    }
    ino.push(path);
    ino.sort_by_key(|a| a.as_os_str().len());
    inode_hm.insert(inode,ino);
    inode_size.insert(inode,i.1.len());
  }

  println!("Finished indexng the files");

  let mut dev_mnt : HashMap<u64,PathBuf> = HashMap::new();
  
  let mut sum_per_dev : HashMap<u64,u64> = HashMap::new();
  //let mut min_foo : usize = std::usize::MAX;
  for i in &inode_size
  {
    if sum_per_dev.contains_key(&i.0[0])
    {
      sum_per_dev.insert(i.0[0],i.1+sum_per_dev[&i.0[0]]);
    }
    else {
      sum_per_dev.insert(i.0[0],i.1.clone());
    }
    if !dev_mnt.contains_key(&i.0[0]) //|| inode_hm[i.0][0].as_os_str().len() < min_foo
    {
      dev_mnt.insert(i.0[0],inode_hm[i.0][0].clone());
      //min_foo = inode_hm[i.0][0].as_os_str().len();
    }
    
  }
  let mut final_dev_mnt = dev_mnt.clone();
  for i in &dev_mnt
  {
    final_dev_mnt.insert(i.0.clone(),get_ult_parent(&i.1));
  }
  
  println!("Finished counting the sizes");
  
  //let dev_map = get_dev_mountpoint();
  
  for i in &sum_per_dev
  {
    /*if dev_map.contains_key(i.0)
    {
      println!("{} {}", dev_map[i.0], i.1/(1024*1024));
    }
    else {*/
    println!("{} {}", final_dev_mnt[i.0].display(), i.1/(1024*1024));
    //}
    
    
  }
  println!("----------");
  for i in &Disks::new_with_refreshed_list()
  {
    println!("{} {}",(i.usage().read_bytes)/(1024*1024),i.total_space()/(1024*1024));
    println!("{} {}",(i.total_space() - i.available_space())/(1024*1024),i.total_space()/(1024*1024));
  }
  
}


// use serde::Deserialize;
// use std::io::{BufReader, Read};

//use std::os::unix::net::UnixStream;

use std::{collections::HashMap, fs::File, path::PathBuf};

use jwalk::WalkDir;

fn main(){
  
  let mut hm : HashMap<PathBuf, u64> = HashMap::new();
  let walk = WalkDir::new("/"); //.parallelism(jwalk::Parallelism::Serial);
  let final_walk = walk.into_iter().filter_map(|pth| pth.ok());
  
  
  
  for pth in final_walk{ 
    //println!("Looking at {}", pth.path().display());
    let mut l = 0;
    if !pth.path().is_symlink() || pth.depth() != 0
    {
      let f = match File::open(pth.path()){
        Ok(fle) => fle,
        Err(e) =>{
          if e.kind() == std::io::ErrorKind::PermissionDenied
          {
            // eprintln!("Permission Denied on {}", pth.path().display());
            continue;
          }
          continue
        }
      };
      l = f.metadata().expect("Corrupted metadata.").len();
    }
    
    hm.insert(pth.path(),l);
  }
  
  let mut sum : u64 = 0;
  for i in hm{
    // println!("{} {}" , p ,i.1); 
    sum = sum + i.1;
  }
  println!("{}", sum);
}

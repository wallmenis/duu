use std::{collections::HashMap, fs::Metadata, os::unix::fs::MetadataExt, path::{PathBuf, Path}};

use jwalk::WalkDir;

use crate::tree::Tree;


pub fn get_ult_parent(pth : &PathBuf) -> PathBuf
{
    
    let mut now : PathBuf = pth.clone();
    let mut prev : PathBuf = pth.clone();
    match std::fs::metadata(pth)
    {
        Ok(o) =>{
            let d = o.dev();
            let mut f = o;
            while f.dev() == d && prev != PathBuf::from("/")
            {
                prev = now.clone();
                now = match now.parent(){
                    Some(o) => o,
                    None => Path::new("/")
                }
                .to_path_buf();
                f = match std::fs::metadata(&now){
                    Ok(out) => out,
                    Err(e) => {
                        eprintln!("In get_ult_parent while digging: {}",e);
                        prev = pth.clone();
                        break
                    }
                };
            }
        }
        Err(e) => {eprintln!("In get_ult_parent: {}",e);}
    }
    prev.clone()
}

pub fn walker(p : &PathBuf, suppress_errors : Option<bool>) -> HashMap<PathBuf, Metadata>
{
    let mut hm : HashMap<PathBuf, Metadata> = HashMap::new();
    let walk = WalkDir::new(p); //.parallelism(jwalk::Parallelism::Serial);
    let final_walk = walk.into_iter().filter_map(|pth| pth.ok());
    
    for pth in final_walk{
        if pth.path_is_symlink() {
            continue;
        }
        let f = match std::fs::metadata(&pth.path()){
            Ok(fle) => fle,
            Err(e) =>{
                match suppress_errors
                {
                    Some(b) =>
                    {
                        if !b 
                        {
                            eprintln!("In walker: {} {}",e, &pth.path().display());
                        }
                    },
                    None =>
                    {
                        eprintln!("In walker: {} {}",e, &pth.path().display());
                    }
                }
                
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

pub fn inode_deduplicator_single_path(hm : &HashMap<PathBuf, Metadata>) -> HashMap<[u64; 2],PathBuf>
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
pub fn get_sizes_recursive_no_dedup(hm : &HashMap<PathBuf, Metadata>, t : &Tree, start : &PathBuf) -> u64
{
    let mut sum = 0;
    let ct = t.get_leaf(start);
    if ct.is_some()
    {
        let h = ct.unwrap().hm;   //is checked before
        if h.is_empty()
        {
            return hm[start].len();
        }
        for i in &h
        {
            //let current = PathBuf::from( start.display().to_string()+ "/" + i.0);
            let current = start.join(i.0);
            sum += get_sizes_recursive_no_dedup(hm,t ,&current );
        }
    }
    sum
}

pub fn get_sizes_recursive(hm : &HashMap<PathBuf, Metadata>, t : &Tree, start : &PathBuf) -> u64
{
    let inode_bin : &mut HashMap<[u64; 2], u64> = &mut HashMap::new();
    get_sizes_recursive_inode_bin(hm,t ,start , inode_bin)
}

pub fn get_sizes_recursive_inode_bin(hm : &HashMap<PathBuf, Metadata>,
                                 t : &Tree,
                                 start : &PathBuf,
                                 inode_bin : &mut HashMap<[u64; 2], u64>) -> u64
{
    let mut sum = 0;
    let ct = t.get_leaf(start);
    if ct.is_some()
    {
        let h = ct.unwrap().hm;     //is also checked before
        if h.is_empty()
        {
            let mut len = 0;
            let inode = [hm[start].dev(),hm[start].ino()];
            if !inode_bin.contains_key(&inode)
            {
                len = hm[start].len();
                inode_bin.insert(inode, len);
            }
            return len;
        }
        for i in &h
        {
            // let current = PathBuf::from( start.display().to_string()+ "/" + i.0);
            let current = start.join(i.0);
            sum += get_sizes_recursive_inode_bin(hm,t ,&current , inode_bin);
        }
    }
    sum
}
                                 
#[allow(dead_code)]
pub fn inode_deduplicator(hm : &HashMap<PathBuf, Metadata>) -> HashMap<[u64; 2],Vec<PathBuf>>
{
    let mut inode_hm : HashMap<[u64; 2],Vec<PathBuf>> = HashMap::new();
    for i in hm{
        let inode = [i.1.dev(),i.1.ino()];
        let path = i.0.clone();    
        inode_hm.entry(inode).or_insert(Vec::new()).push(path);
    }
    inode_hm
}
                                 
pub fn inode_sizes(hm : &HashMap<PathBuf, Metadata>) -> HashMap<[u64; 2],u64> 
{
    let mut inode_size : HashMap<[u64; 2],u64> = HashMap::new();
    for i in hm{
        let inode = [i.1.dev(),i.1.ino()];
        inode_size.insert(inode,i.1.len());
        //inode_size.insert(inode,512*i.1.blocks());
    }
    inode_size
}
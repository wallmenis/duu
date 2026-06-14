use std::{collections::{HashMap, HashSet}, fs::Metadata, os::unix::fs::MetadataExt, path::{Path, PathBuf}};

use jwalk::WalkDir;

use crate::tree::Tree;

use crate::sizes::UNIX_BLOCK_SIZE;


#[allow(dead_code)]
pub fn get_ult_parent(pth : &PathBuf) -> PathBuf
{
    let mut now : PathBuf = pth.clone();
    let mut prev : PathBuf = pth.clone();
    if !pth.is_symlink()
    {
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
    }
    prev.clone()
}



pub fn walker(p : &PathBuf, suppress_errors: bool) -> HashMap<PathBuf, Metadata>
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
                    false =>
                    {
                        eprintln!("In walker: {} {}",e, &pth.path().display())
                    }
                    true => {}
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

#[allow(dead_code)]
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
    let ct = t.get_child_ref(start);
    if ct.is_some()
    {
        let h = &ct.unwrap().hm;   //is checked before
        if h.is_empty()
        {
            return hm[start].blocks()*UNIX_BLOCK_SIZE;
        }
        for i in h
        {
            let current = start.join(i.0);
            sum += get_sizes_recursive_no_dedup(hm,t ,&current );
        }
    }
    sum
}
#[allow(dead_code)]
pub fn get_sizes_recursive_hash_map(hm : &HashMap<PathBuf, Metadata>, t : &Tree, start : &PathBuf) -> u64
{
    let inode_bin : &mut HashSet<[u64; 2]> = &mut HashSet::new();
    get_sizes_recursive_hash_map_inode_bin(hm,t ,start , inode_bin)
}
#[allow(dead_code)]
pub fn get_sizes_recursive_hash_map_inode_bin(hm : &HashMap<PathBuf, Metadata>,
                                 t : &Tree,
                                 start : &PathBuf,
                                 inode_bin : &mut HashSet<[u64; 2]>) -> u64
{
    let mut sum = 0;
    let ct = t.get_child_ref(start);
    if ct.is_some()
    {
        let h = &ct.unwrap().hm;     //is also checked before
        if h.is_empty()
        {
            let mut len = 0;
            let inode = [hm[start].dev(),hm[start].ino()];
            if !inode_bin.contains(&inode)
            {
                // len = hm[start].len();
                len = UNIX_BLOCK_SIZE*hm[start].blocks();
                inode_bin.insert(inode);
            }
            return len;
        }
        for i in h
        {
            let current = start.join(i.0);
            sum += get_sizes_recursive_hash_map_inode_bin(hm,t ,&current , inode_bin);
        }
    }
    sum
}

pub fn get_parent_dirs(p : &PathBuf) -> HashSet<PathBuf>
{
    let mut v = HashSet::new();
    let mut pth = PathBuf::from("/");
    for i in p.components()
    {
        pth=pth.join(i);
        v.insert(pth.clone());
    }
    v
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
        // let mut siz = UNIX_BLOCK_SIZE;
        // match nix::sys::statvfs::statvfs(i.0)
        // {
        //     Ok(o) => {siz = o.fragment_size();},
        //     Err(e) => {eprintln!("file {} modified on scanning: {}", i.0.display(), e);}
        // };
        inode_size.insert(inode,i.1.blocks()*UNIX_BLOCK_SIZE);
        //inode_size.insert(inode,512*i.1.blocks());
    }
    inode_size
}
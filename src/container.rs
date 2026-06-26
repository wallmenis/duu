use std::{collections::HashMap, io::Write, path::PathBuf};

use serde::Serialize;

use crate::tree::Tree;

#[derive(Serialize,Clone)]
pub struct ContainerCon
{
    pub container_type : String,
    pub socket : String,
    pub containers : HashMap<String,u64>,
    pub images : HashMap<String, u64>,
    #[serde(skip)]
    is_daemon_running : bool,
    #[serde(skip)]
    pub container_socket_service : String,
    #[serde(skip)]
    pub requires_root : bool,
    #[serde(skip)]
    pub group: String,
    #[serde(skip)]
    pub storage: String
}

impl ContainerCon
{
    pub fn new() -> Self
    {
        ContainerCon { 
            container_type: "podman".to_string(),
            socket: "/run/podman/podman.socket".to_string(),
            containers: HashMap::new(),
            images: HashMap::new(),
            is_daemon_running: false,
            container_socket_service: "podman.socket".to_string(),
            requires_root: true,
            group: "root".to_string(),
            storage: "/var/lib/containers/storage".to_string()
        }
    }
    
    pub fn new_with_params(ct : &String, s : &String, css: &String, rr :bool, g : &String, st: &String) -> Self
    {
        ContainerCon { 
            container_type: ct.clone(),
            socket: s.clone(),
            containers: HashMap::new(),
            images: HashMap::new(),
            is_daemon_running: false,
            container_socket_service: css.clone(),
            requires_root: rr,
            group: g.clone(),
            storage: st.clone()
            
        }
    }
    
    pub fn connect(&mut self) -> Result<(),String>
    {
        let cgid = match nix::unistd::Group::from_name(&self.group)
        {
            Ok(o) => match o
            {
                Some(i) => i.gid.as_raw() as i64,
                None => {-1}
            },
            Err(_) => {-2}
        };
        if  cgid < 0 
        {
            return Err("Group non existant or not configured correctly.".to_string());
        }
        if !(self.requires_root && 
            (nix::unistd::getuid().is_root() ||
            nix::unistd::getegid().as_raw() as i64 == cgid))
        {
            return Err("insufficient privileges.".to_string());
        }
        
        let sock_ = std::os::unix::net::UnixStream::connect(&self.socket);
        
        if sock_.is_err()
        {
            return Err("Daemon not loaded. Couldn't connect to the socket".to_string());
        }
        
        let sock = sock_.unwrap();
        
        let request = "GET";
        
        
        Ok(())
    }
    
    pub fn find_sizes_with_tree(&self, t: &Tree) -> u64
    {
        match t.get_child_ref(&PathBuf::from(&self.storage))
        {
            Some(o) => o.size,
            None => 0
        }
    }
}
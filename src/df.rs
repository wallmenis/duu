use std::path::PathBuf;

use crate::arguments::DataSizes;

pub const UNIX_BLOCK_SIZE : u64 = 512;

pub struct DFEntry
{
    pub filesystem : String,
    pub path : PathBuf,
    pub blocks : u64,
    pub used : u64,
    pub available : u64,
    pub blk_sz : u64,
    pub sd : DataSizes
}

impl DFEntry
{
    pub fn new() -> Self
    {
        DFEntry { filesystem: String::new(), path: PathBuf::new(), blocks:0 , used:0 , available:0 ,blk_sz: UNIX_BLOCK_SIZE, sd : DataSizes::KB }
    }
    
    fn use_perc(&self)->u64
    {
        if self.blocks == 0
        {
            return 0;
        }
        ((self.blocks - self.available)*100)/self.blocks
        //(self.used*100)/self.blocks
    }
    
    fn get_blocks(&self) -> u64
    {
        (self.blocks*self.blk_sz)/self.sd.value()
    }
    
    fn get_used(&self) -> u64
    {
        (self.used*self.blk_sz)/self.sd.value()
    }
    
    fn get_avail(&self) -> u64
    {
        (self.available*self.blk_sz)/self.sd.value()
    }
    
    fn print_one_line(&self, tabs : [u64; 5])
    {
        let mut t : [String; 5] = [String::new(),String::new(),String::new(),String::new(),String::new()];
        // let mut t : [String; 5] = [String::new(); 5];
        for i in 0..5
        {
            for _ in (0 as u64)..tabs[i]
            {
                t[i as usize].push_str(" ");
            }
        }
        
        println!("{}{}{}{}{}{}{}{}{}{}{}%",
                 self.filesystem,t[0],
                 self.path.display(),t[1],
                 self.get_blocks(),t[2],
                 self.get_used(),t[3],
                 self.get_avail(),t[4],
                 self.use_perc());
        
    }
    
    fn calculate_spaces(&self, spaces: [u64; 5]) -> [u64; 5]
    {
        let mut t : [u64; 5] = [0;5];
        t[0] = DFEntry::subtract_tab(spaces[0], &self.filesystem );
        t[1] = DFEntry::subtract_tab(spaces[1], &self.path.display().to_string() );
        t[2] = DFEntry::subtract_tab(spaces[2], &self.get_blocks().to_string() );
        t[3] = DFEntry::subtract_tab(spaces[3], &self.get_used().to_string() );
        t[4] = DFEntry::subtract_tab(spaces[4], &self.get_avail().to_string() );
        t
    }
    
    fn subtract_tab(i: u64, s : &String) ->u64
    {
        let intabs = s.len();
        if (i as usize) > intabs
        {
            return (i as usize - intabs) as u64;
        }
            
        0
    }
    #[allow(dead_code)]
    fn get_max_space(&self) -> u64
    {
        
        let mut max = 0;
        max = if self.filesystem.len() > max {self.filesystem.len()} else {max} ;
        max = if self.path.display().to_string().len() > max {self.path.display().to_string().len()} else {max} ;
        max = if self.get_blocks().to_string().len() > max {self.get_blocks().to_string().len()} else {max} ;
        max = if self.get_used().to_string().len() > max {self.get_used().to_string().len()} else {max} ;
        max = if self.get_avail().to_string().len() > max {self.get_avail().to_string().len()} else {max} ;
        max as u64
    }
    
    fn get_sizes(&self) -> [u64;5]
    {
        let mut t = [0;5];
        t[0] = self.filesystem.len() as u64;
        t[1] = self.path.display().to_string().len() as u64;
        t[2] = self.get_blocks().to_string().len() as u64;
        t[3] = self.get_used().to_string().len() as u64;
        t[4] = self.get_avail().to_string().len() as u64;
        t
    }
    
    pub fn df(items : &Vec<DFEntry>)
    {
        let mut max_tabbs = [0; 5];
        for i in items
        {
            let siz = i.get_sizes();
            for j in 0..5
            {
                max_tabbs[j] = if siz[j] > max_tabbs[j] {siz[j]} else {max_tabbs[j]}
            }
            
        }
        
        for i in 0..5
        {
            max_tabbs[i] += 1;
        }
        
        for i in items
        {
            i.print_one_line(i.calculate_spaces(max_tabbs));
        }
        
    }
}
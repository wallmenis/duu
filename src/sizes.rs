use std::path::PathBuf;

pub struct Size
{
    pub filesystem : String,
    pub path : PathBuf,
    pub blocks : u64,
    pub used : u64,
    pub available : u64,
    pub use_perc : u64
}

impl Size
{
    pub fn new() -> Self
    {
        Size { filesystem: String::new(), path: PathBuf::new(), blocks:0 , used:0 , available:0 , use_perc:0  }
    }
    
    fn print_one_line(&self, tabs : [u64; 5])
    {
        let mut t : [String; 5] = [String::new(),String::new(),String::new(),String::new(),String::new()];
        for i in tabs
        {
            for _ in (0 as u64)..i
            {
                t.get_mut(i as usize).unwrap().push_str("\t");
            }
        }
        
        println!("{}{}{}{}{}{}{}{}{}{}{}",
                 self.filesystem,t[0],
                 self.path.display(),t[1],
                 self.blocks,t[2],
                 self.used,t[3],
                 self.available,t[4],
                 self.use_perc);
        
    }
    
    fn calculate_spaces(&self, spaces: [u64; 5]) -> [u64; 5]
    {
        let mut t : [u64; 5] = [0,0,0,0,0];
        t[0] = Size::subtract_tab(spaces[0], &self.filesystem );
        t[0] = Size::subtract_tab(spaces[0], &self.path.display().to_string() );
        t[0] = Size::subtract_tab(spaces[0], &self.blocks.to_string() );
        t[0] = Size::subtract_tab(spaces[0], &self.used.to_string() );
        t[0] = Size::subtract_tab(spaces[0], &self.available.to_string() );
        t
    }
    
    fn subtract_tab(i: u64, s : &String) ->u64
    {
        let intabs = s.len()/4;
        if i as usize - intabs > 0
        {
            return (i as usize - intabs) as u64;
        }
            
        0
    }
    
    pub fn df()
    {
        
    }
}
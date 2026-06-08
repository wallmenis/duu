use clap::Parser;

#[derive(Parser)]
pub struct Ar
{
    #[arg(default_value=".")]
    pub path : String,
    
    #[arg(short,long,default_value="MB")]
    pub size : String,
    
    #[arg(short, long)]
    pub flatpak : bool,
    
    #[arg(short,long)]
    pub containers : bool
}

#[repr(u64)]
pub enum DataSizes {
    B = 1,
    KB = 1024,
    MB = 1024*1024,
    GB = 1024*1024*1024
}

pub fn string_to_data_size(s : &String) -> DataSizes
{
    if s.to_uppercase() == "KB".to_string()
    {
        return DataSizes::KB;
    }
    if s.to_uppercase() == "MB".to_string()
    {
        return DataSizes::MB;
    }
    if s.to_uppercase() == "GB".to_string()
    {
        return DataSizes::GB;
    }
    DataSizes::B
}
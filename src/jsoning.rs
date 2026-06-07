use crate::{flatpak::FlatpakDir, tree::Tree};

impl Tree
{
    pub fn print(&self)
    {
        let str_out =  match serde_json::to_string_pretty(self)
        {
            Ok(o) => o,
            Err(e) => e.to_string()
        };
        println!("{}",str_out);
    }
    
    pub fn return_json(&self) -> String
    {
        match serde_json::to_string(self)
        {
            Ok(o) => o,
            Err(_) => String::from("{}")
        }
    }
}

impl FlatpakDir
{
    pub fn print(&self)
    {
        let str_out =  match serde_json::to_string_pretty(self)
        {
            Ok(o) => o,
            Err(e) => e.to_string()
        };
        println!("{}",str_out);
    }
    
    pub fn return_json(&self) -> String
    {
        match serde_json::to_string(self)
        {
            Ok(o) => o,
            Err(_) => String::from("{}")
        }
    }
}
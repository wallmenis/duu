use clap::Parser;

#[derive(Parser)]
pub struct Ar
{
    #[arg(default_value="/")]
    pub path : String
}
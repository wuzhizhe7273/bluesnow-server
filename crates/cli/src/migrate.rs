#[derive(clap::Subcommand)]
pub enum MigrateCommands{
    Up,
    Down,
    Init,
    Export,
    Import
}

impl MigrateCommands{
    pub async fn execute(self){
        match self {
            Self::Up=>{
                todo!()
            },
            Self::Down=>{
                todo!()
            },
            Self::Init=>{
              todo!()  
            },
            Self::Export=>{
                todo!()
            },
            Self::Import=>{
                todo!()
            }
        }
    }
}
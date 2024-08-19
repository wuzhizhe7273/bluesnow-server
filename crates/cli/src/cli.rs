use crate::migrate::MigrateCommands;

#[derive(clap::Subcommand)]
pub enum Commands{
    /// start server
    Start,
    /// data migrate commands
    #[command(subcommand)]
    Migrate(MigrateCommands)
}
#[derive(clap::Parser)]
#[command(version,about,long_about= None )]
pub struct Cli{
    #[command(subcommand)]
    command:Option<Commands>
}

impl Cli{
    pub async fn run(self)->anyhow::Result<()>{
        if let Some(command)=self.command{
            match command {
                Commands::Start=>{
                    let server_config=server::ServerConfig::new();
                    let server = server::Server::new(server_config);
                    server.run().await?;
                },
                Commands::Migrate(migrate_command)=>{
                    migrate_command.execute().await;
                }
            }
        }
        Ok(())
    }
}
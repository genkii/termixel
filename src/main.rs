use clap::Parser;
use clap::Subcommand;
use termixel::config_path;
use termixel::render_sprite;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Sprite { name: String },

    List,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Sprite { name } => {
            let config_path = config_path::config_dir().ok_or("Failed to find config dir")?;

            if !config_path.exists() {
                println!("Creating config dir ...");
                std::fs::create_dir_all(&config_path)?;
            }
            let path = config_path.join(format!("{}.png", name.to_lowercase()));

            if !path.exists() {
                println!("Sprite with name '{}' not found.", name);
                return Ok(());
            }
            render_sprite::render_sprite(&path)?;
        }

        Commands::List => {
            let config_path = config_path::config_dir().ok_or("Failed to find config dir")?;

            if !config_path.exists() {
                std::fs::create_dir_all(&config_path)?;
            }
            let mut sprites = std::fs::read_dir(&config_path)?;

            if sprites.next().is_none() {
                println!("No sprites found");
                return Ok(());
            }
            for sprite in sprites {
                println!("{}", sprite?.file_name().to_string_lossy());
            }
        }
    }

    Ok(())
}

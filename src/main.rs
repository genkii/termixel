use termixel::config_path;
use termixel::render_sprite;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        Some("sprite") => {
            let name = args.next().ok_or("Missing sprite name")?;
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

        Some("list") => {
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

        Some("help") | Some("-h") | Some("--help") | None => {
            println!(
                "
                termixel - A terminal sprite renderer

                COMMANDS:
                    sprite <name>    Render a sprite
                    list             List available sprites

                OPTIONS:
                    -h, --help       Print help
                    -V, --version    Print version

            "
            )
        }

        Some("--version") | Some("-V") => {
            println!("termixel {}", "v0.1.0");
        }

        Some(command) => {
            eprintln!("Unknown command: {command}");
            eprintln!("Run 'termixel --help' for usage.");
            std::process::exit(2);
        }
    }

    Ok(())
}

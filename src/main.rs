use clap::Parser;

mod compression;
mod wolf3d;

//Wolfenstein 3D Viewer
#[derive(Parser)]
#[clap(author, about, version, long_about = None)]
enum Args {
    ///View Wolfenstein 3D assets
    View {
        ///Wolfenstein 3D path
        path: String,
    },
}

fn main() {
    let args = Args::parse();
    match args {
        Args::View { path } => {
            let result = wolf3d::read_gamemaps(&path);
            match result {
                Ok(gamemaps) => {
                    println!("Ok");
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}

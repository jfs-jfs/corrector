use clap::Parser;
use corrector::{api::APICorrector, fixer::Fixer};

#[derive(Debug, Parser)]
#[command(about = "Corrector ortogràfic de català per a la terminal. (Powered by SofCatalà)", long_about = None)]
struct CorrectorArgs {
    #[arg(help = "Text a corretgir")]
    text: String,

    #[arg(
        short,
        long,
        help = "No preguntis, corretgeix amb el més probable",
        default_value_t = false
    )]
    fuck_it: bool,

    #[arg(
        short,
        long,
        help = "No facis res, només imprimeix el resultat en json",
        default_value_t = false
    )]
    json: bool,
}

fn main() {
    let args = CorrectorArgs::parse();

    let response = match APICorrector::run_query(&args.text) {
        Ok(response) => response,
        Err(error) => {
            eprintln!("{:#?}", error);
            return;
        }
    };

    match (args.fuck_it, args.json) {
        // Normal behaviour => Fix it interactively
        (false, false) => println!("{}", Fixer::interactive(&args.text, response.matches())),

        // Fix it with the first suggestions and spit back the corrected text
        (true, false) => println!("{}", Fixer::all(&args.text, response.matches())),

        // Json flag activated => only print JSON
        (_, true) => println!("{}", serde_json::to_string(&response).unwrap()),
    }
}

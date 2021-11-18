use clap::Parser;
use hyphenation::{Language, Load, Standard};
use regex::Regex;
use textwrap::{Options, fill};
use wordnik::Client;

static WORDNIK_API_KEY_NAME: &str = "WORDNIK_API_KEY";

#[derive(Clone, Debug, Parser)]
struct Opts {
    word: String,
}

fn main() {
    dotenv::dotenv().ok();

    if let Err(e) = run(&Opts::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run(opts: &Opts) -> wordnik::Result<()> {
    const COL_WIDTH: usize = 120;

    let client = Client::new(dotenv::var(WORDNIK_API_KEY_NAME).expect("wtf is your key?"));
    let definitions = client.definitions(&opts.word)?;
    let definitions = definitions.iter().filter_map(|def| def.text.as_ref());

    format(&opts.word, definitions, terminal_width().unwrap_or(COL_WIDTH).min(COL_WIDTH));
    Ok(())
}

fn format<T: AsRef<str>, D>(word: &str, definitions: D, width: usize)
where
    D: Iterator<Item = T>
{
    let xref_remover = Regex::new(r#"</?.+?>"#).unwrap();
    let hyphenator = Standard::from_embedded(Language::EnglishUS).unwrap();
    let options = Options::new(width)
        .initial_indent(" - ")
        .subsequent_indent("   ")
        .word_splitter(hyphenator);

    println!("{}", word);

    for def in definitions {
        let text = xref_remover.replace_all(def.as_ref(), "");
        println!("{}", fill(&text, &options));
    }
}

fn terminal_width() -> Option<usize> {
    term_size::dimensions_stdout().map(|(width, _height)| width)
}

use clap::Parser;
use regex::Regex;
use tabled::{MaxWidth, Modify, Rows, Style, Table, Tabled};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// 検索したい単語
    word: String,

    /// 検索モード。前方一致（1）、後方一致（2）、完全一致（3）、あいまい（4）
    #[clap(short, long, default_value_t = 4)]
    searchway: u8,
}

#[derive(Tabled)]
struct WordSet {
    id: String,
    word: String,
    class: String,
    meaning: String,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    // Generate search URL
    let args = Args::parse();
    let url_prefix = "http://cblle.tufs.ac.jp/dic/ru/v_search_list.php?serchTxt=";
    let url_suffix = "&searchWayID=";
    let url = format!(
        "{}{}{}{}",
        url_prefix, args.word, url_suffix, args.searchway
    );

    // Get HTML
    let body = match page_get(&url) {
        Ok(page) => page,
        Err(why) => {
            panic!("{:?}", why)
        }
    };

    // Parse HTML (for <form> block, which include word list)
    let dom = tl::parse(&body, tl::ParserOptions::default()).expect("HTML string too long");
    let parser = dom.parser();
    let form = dom
        .query_selector("form")
        .expect("cannot find form block")
        .nth(2)
        .expect("cannot find 2nd form block")
        .get(parser)
        .expect("cannot parse block")
        .inner_html(parser);

    // Parse HTML (for <p> elements, each of which include single word)
    let innerdom = tl::parse(&form, tl::ParserOptions::default()).expect("HTML string too long");
    let innerparser = innerdom.parser();
    let wordlist = innerdom.query_selector("p").expect("cannot find wordlist");

    // Regex
    let nbsp = Regex::new(r"(&nbsp;)+").unwrap(); // "&nbsp;"
    let brac_close = Regex::new(r"\]").unwrap(); // "]"
    let extra_delim = Regex::new(r"\],\[").unwrap(); // "],["

    let mut wordsets: Vec<WordSet> = Vec::new();

    // Wordlist
    for line in wordlist {
        // Parse wordlist
        let line_parsed = line
            .get(innerparser)
            .expect("cannot parse line")
            .inner_text(innerparser);
        let delimadded = nbsp.replace_all(&line_parsed, ",");
        let delimadded_brac = brac_close.replace_all(&delimadded, "],");
        let delimrmed = extra_delim.replace(&delimadded_brac, "][");

        let mut splited: Vec<&str> = delimrmed.split(",").collect();

        if splited.len() < 4 {
            for _i in splited.len()..4 {
                splited.push("");
            }
        }

        let newword = WordSet {
            id: splited[0].to_string(),
            word: splited[1].to_string(),
            class: splited[2].to_string(),
            meaning: splited[3].to_string(),
        };

        wordsets.push(newword);
    }

    let table = Table::new(wordsets)
        .with(Style::modern()) // use pseudo-table style
        .with(Modify::new(Rows::new(1..)).with(MaxWidth::truncating(20))); // limit each row's width to 10

    println!("{}", table);

    Ok(())
}

fn page_get(url: &str) -> Result<String> {
    let resp = ureq::get(url).call();
    match resp {
        Ok(page) => match page.into_string() {
            Ok(input) => return Ok(input),
            Err(why) => return Err(Box::new(why)),
        },
        Err(why) => return Err(Box::new(why)),
    };
}

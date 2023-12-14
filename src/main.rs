use clap::{Command, Arg, ArgAction};
use pulldown_cmark::{html::push_html, Event, Parser};
use maud::html;

fn wrap_html(s:&str, css:Option<&String>)->String{
    let res = html!{
        (maud::DOCTYPE)
            html {
                head {
                    meta charset="utf-8";
                    @if let Some(s) = css{
                        link rel="stylesheet" type="text/css" href=(s) {}
                    }
                }
                body {
                    (maud::PreEscaped(s))
                }
            }
    };
    res.into_string()
}

fn main() {
    let matches = Command::new("mdrend")
        .version("0.1")
        .author("Froilan Miranda")
        .about("Renders markdown as you like")
        .arg(Arg::new("input").help("Sets the input file").required(true))
        .arg(Arg::new("wrap").help("Wrap in html").short('w').action(ArgAction::SetTrue))
        .arg(Arg::new("event").help("Print events").short('e').action(ArgAction::SetTrue))
        .arg(Arg::new("css").help("Link to css").long("css"))
        .get_matches();

    println!("Input = {:?}", matches.get_one::<String>("input"));
    let infile = std::fs::read_to_string(matches.get_one::<String>("input").unwrap()).expect("Could not read file");

    let mut res = String::new();
    let ps = Parser::new(&infile);

    let ps: Vec<Event> = ps.into_iter().collect();
    if matches.get_flag("event"){
        for p in &ps {
            println!("{:?}", p);
        }
    }
    push_html(&mut res, ps.into_iter());
    if matches.get_flag("wrap"){
        res = wrap_html(&res, matches.get_one::<String>("css"));
    }
    /*
    let clap = clap_app!(mdrend =>
        (version:crate_version!())
        (author:"Froilan Miranda")
        (about:"Renders markdown as you like")
        (@arg input: + required "Sets the input file")
        (@arg wrap: -w "Wrap in html")
        (@arg event: -e "Print events")
        (@arg css: --css +takes_value "Link to css")
    )
    .get_matches();

    println!("Input = {:?}", clap.value_of("input"));
    let infile = std::fs::read_to_string(clap.value_of("input").unwrap()).expect("Could not read file");
    let mut res = String::new();
    let ps = Parser::new(&infile);

    let ps: Vec<Event> = ps.into_iter().collect();
    if clap.is_present("event"){

    for p in &ps {
        println!("{:?}", p);
    }
    }
    push_html(&mut res, ps.into_iter());

    if clap.is_present("wrap"){
        res = wrap_html(&res, clap.value_of("css"));
    }
    */
    println!("{}", res);
}

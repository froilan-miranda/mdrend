use clap::{clap_app, crate_version};

fn main() {
    let clap = clap_app!(mdrend =>
        (version:crate_version!())
        (author:"Froilan Miranda")
        (about:"Renders markdown as you like")
        (@arg input: + required "Sets the input file")
    )
    .get_matches();

    println!("Input = {:?}", clap.value_of("input"));
    println!("done")
}

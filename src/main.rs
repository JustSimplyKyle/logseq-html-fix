use std::fs;

use argh::FromArgs;
use color_eyre::eyre::eyre;
use visdom::Vis;

#[derive(FromArgs, PartialEq, Eq)]
/// Struct representing command-line arguments
struct Arguments {
    /// path to the input file
    #[argh(option, short = 'i')]
    input: String,

    /// path to the output file
    #[argh(option, short = 'o')]
    output: String,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args: Arguments = argh::from_env();

    let str = String::from_utf8(fs::read(args.input)?)?;

    let root = Vis::load(str).map_err(|x| eyre!(x))?;
    root.find("#root")
        .set_attr("style", Some("padding-left:50px;padding-right:50px;"));
    root.find("#main-container")
        .remove_class("is-left-sidebar-open")
        .add_class("is-left-sidebar-close");
    let html = root.outer_htmls();
    std::fs::write(args.output, html)?;
    Ok(())
}

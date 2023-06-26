use colored::Colorize;

pub fn get_banner() {

    let banner: String = "
    RustFinder v1.0.0 - by oppsec
    GitHub Account Information Gathering
    ".yellow().bold().italic().to_string();

    return println!("{}", banner);
}
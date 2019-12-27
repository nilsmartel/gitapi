mod repos;
use structopt::StructOpt;

fn main() {
    let args = Arguments::from_args();
    match args {
        Arguments::Repos { user } => repos::print_repos(&user),
    };
}

#[derive(StructOpt, Debug)]
#[structopt(about = "easy interface into the GitHub API")]
enum Arguments {
    Repos {
        #[structopt()]
        user: String,
    },
}

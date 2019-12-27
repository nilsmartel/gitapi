mod repo;
use structopt::StructOpt;

fn main() {
    let args = Arguments::from_args();
    match args {
        Arguments::Repo { user } => repo::print_repos(&user),
    };
}

#[derive(StructOpt, Debug)]
#[structopt(about = "easy interface into the GitHub API")]
enum Arguments {
    Repo {
        #[structopt()]
        user: String,
    },
}

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cars {
    #[clap(long, value_parser)]
    agent_path: Option<String>,
}

// Agent path might be something like: "MusicalChairsLib/checkpoints/latest-step000000999424"

fn main() {
    let args = Cars::parse();
    musical_cars_lib::run(args.agent_path);
}

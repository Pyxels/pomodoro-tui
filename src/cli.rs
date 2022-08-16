use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {

    /// Duration of the work segment (minutes)
    #[clap(short, long, default_value_t = 25)]
    pub work: i64,

    /// Duration of the short rest segment (minutes)
    #[clap(short, long, default_value_t = 5)]
    pub small_rest: i64,

    /// Duration of the long rest segment (minutes)
    #[clap(short, long, default_value_t = 35)]
    pub large_rest: i64,
}

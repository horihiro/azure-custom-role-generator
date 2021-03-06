// use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "Azure Custom Role generator",
  about = "A command line tool to generate Azure Custom Role Definition template"
)]
pub struct CommandLineArgs {
  /// Specify base role definition file path to append actions.
  #[structopt(short = "a", long = "append-to", default_value)]
  pub base_definition_filepath: String,

  /// Prints inputs from stdin to stderr
  #[structopt(long = "debug")]
  pub debug: bool,
}

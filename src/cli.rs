// use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "Azure Custom Role generator",
  about = "A command line tool to generate Azure Custom Role Definition template"
)]
pub struct CommandLineArgs {
  // /// Storage Account name
  // #[structopt(short = "a", long = "account-name", required_unless = "")]
  // pub account_name: String,

  // /// Blob Container name
  // #[structopt(short = "c", long = "container-name", required_unless = "")]
  // pub container_name: String,

  // /// Blob Path template
  // #[structopt(short = "b", long = "blob-path-template", required_unless = "")]
  // pub blob_path: String,

  // / Blob type "AppendBlob", "BlockBlob" or "PageBlob"
  //
  // #[structopt(short = "t", long = "blob-type", default_value = "AppendBlob" )]
  // pub blob_type: String,
}

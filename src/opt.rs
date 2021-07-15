use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone, Serialize, Deserialize, Default)]
#[structopt()]
pub struct Opt {}

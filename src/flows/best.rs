use crate::flows;
use crate::flows::core::Variant;
use crate::structures::option::Config;
use std::error::Error;

pub fn main(query: String, args: Vec<String>, config: Config) -> Result<(), Box<dyn Error>> {
    if args.is_empty() {
        flows::core::main(Variant::Filter(query), config, false)
    } else {
        flows::aux::abort("passing arguments to 'navi best'", 201)
    }
}

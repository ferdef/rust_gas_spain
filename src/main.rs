mod gas_structures;
mod gas_prices;
mod gas_files;

use std::io::Error;
use gas_prices::{retrieve_last_gas_info};

#[tokio::main] 
async fn main() -> Result<(), Error> {

    let gas_info = retrieve_last_gas_info().await?;

    println!("{:#?}", gas_info);

    Ok(())
}


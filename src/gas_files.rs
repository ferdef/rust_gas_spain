use std::io::Error;
use super::gas_structures::FileStruct;

pub fn store(data: &FileStruct)  -> Result<(), Error>{
    std::fs::write(
        "file.data",
        serde_json::to_string_pretty(data).unwrap(),
    )?;

    println!("File Stored");

    Ok(())
}

pub fn load() -> Result<FileStruct, Error> {
    let data = {
        let data = std::fs::read_to_string(&"file.data")?;

        // Load the MissyFoodSchedule structure from the string.
        serde_json::from_str::<FileStruct>(&data).unwrap()
    };

    println!("File loaded");

    Ok(data)
}
use std::io::Error;
use std::collections::HashMap;

use super::gas_structures::{Gas, Entry, FileStruct};
use super::gas_files::{load, store};

pub async fn retrieve_gas_data() -> Result<Gas, reqwest::Error> {
    let todos: Gas = reqwest::Client::new()
        .get("https://sedeaplicaciones.minetur.gob.es/ServiciosRESTCarburantes/PreciosCarburantes/EstacionesTerrestres/")
        .send()
        .await?
        .json()
        .await?;

    Ok(todos)
}

pub fn group_by_region(gas: &Gas) -> Result<FileStruct, Error> {
    let mut regions: HashMap<String, Vec<Entry>> = HashMap::new();
    

    for entry in &gas.lista_eess_precio {
        if !regions.contains_key(&entry.provincia) {
            regions.insert(entry.provincia.to_owned(), Vec::new());
        }
        let v = regions.get_mut(&entry.provincia).unwrap();
        let new_entry: Entry = entry.clone();
        v.push(new_entry);
    }

    let result = FileStruct {
        date: gas.fecha.to_owned(),
        regions: regions
    };

    Ok(result)
}

pub async fn retrieve_by_region() -> Result<FileStruct, Error> {
    let result = match retrieve_gas_data().await {
        Ok(data) => match group_by_region(&data) {
            Ok(data_by_region) => data_by_region,
            Err(error) => panic!("Error grouping the  information {:?}", error)
        },
        Err(error) => panic!("Error retrieving that information {:?}", error)
    };

    Ok(result)
}

/*
* Process:
* - Look for a file
*   - It exists, load it
*   - Check if date is today
*       - If it's today, keep it
*       - If not, fail
*   - File does not exist, fail
* - If failed, retrieve new info
*   - Store new file
*/
pub async fn retrieve_last_gas_info() -> Result<FileStruct, Error> {
    let file_data = match load() {
        Ok(file_data) => {
            if check_date(&file_data) {
                return Ok(file_data);
            }

            match retrieve_by_region().await {
                Ok(net_data) => {
                    store(&net_data)?;
                    net_data
                },
                Err(_) => file_data
            }
        }
        Err(_) => {
            let net_data = retrieve_by_region().await?;
            store(&net_data)?;
            net_data
        }
    };

    Ok(file_data)
}

fn check_date(data: &FileStruct) -> bool {
    let date = chrono::Local::now().naive_local().date();
    let data_date = chrono::NaiveDateTime::parse_from_str(&data.date, "%d/%m/%Y %H:%M:%S").unwrap().date();

    data_date < date
}
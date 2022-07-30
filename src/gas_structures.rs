use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    #[serde(rename = "Localidad")]
    pub localidad: String,
    #[serde(rename = "Provincia")]
    pub provincia: String,
    #[serde(rename = "Precio Gasolina 95 E5")]
    pub precio_gasolina_95: String,
    #[serde(rename = "Precio Gasoleo B")]
    pub precio_gasoleo_b: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gas {
    #[serde(rename = "Fecha")]
    pub fecha: String,
    #[serde(rename = "ListaEESSPrecio")]
    pub lista_eess_precio: Vec<Entry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStruct {
    pub date: String,
    pub regions: HashMap<String, Vec<Entry>>,
}
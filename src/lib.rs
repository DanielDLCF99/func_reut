use serde_json::Value;
use proxy_wasm::types::*;
use log::{debug, error, info, trace, warn};

pub fn transform(input: String, field: String) -> String {
    info!("transform function");
    //Se deserializa el json
    warn!("The deserialization process is in progress...");
    let mut v: Value = match serde_json::from_str(input.as_str()) {
        Ok(val) => val,
        Err(e) => {
            error!("Has ocurred an error: {}", e);
            return input;
        }
    };
    //se busca que exista un campo con el nombre configurado en API Manager
    if let Some(_field_value) = v.get(field.as_str()) {
        info!("transform function field found!");
        v[field] = serde_json::Value::String("Earth".to_owned());
    } else {
        info!("Field not found");
    }
    //Inyección campo "legion": "Jedi" en body
    v["legion"] = serde_json::Value::String("Jedi".to_owned());
    v.to_string()
}

//determinar nivel de logs
pub fn select_log_level(log: String) -> LogLevel {
    trace!("The select_log_level function is in execution...");
    let log = match log.as_str() {
        "trace" => LogLevel::Trace,
        "warning" => LogLevel::Warn,
        "debug" => LogLevel::Debug,
        "error" => LogLevel::Error,
        _ => LogLevel::Info
    };
    return log;
    
}

//función para manejar query params
pub fn valores_query(path: &str) {
    //se separan cada uno de los query parameters
    let queries: Vec<&str> = path.split("&").collect();
    let mut queries_arreglo: Vec<String> = Vec::new();
    for i in queries {
        //se separa cada uno de los valores
        let mut queries_parte: Vec<&str> = i.split("=").collect();
        if let Some(exemplo) = queries_parte.iter().position(|&field| field=="exemplo").and_then(|i| Some(i+1)) {
            debug!("Valor query param example: {}", queries_parte[exemplo]);
            queries_parte[exemplo] = "2";
        } 
        if let Some(prueba) = queries_parte.iter().position(|&field| field =="prueba").and_then(|i| Some(i+1)) {
            debug!("Valor query param example: {}", queries_parte[prueba]);
            queries_parte[prueba] = "DanielDeLaCruz";
        }
        if let Some(test) = queries_parte.iter().position(|&field| field == "test").and_then(|i| Some(i+1)) {
            debug!("Valor query param test: {}", queries_parte[test]);
            queries_parte[test] = "python";
        }
        //se une el query parameter
        let query_modificada = queries_parte.join("=");
        queries_arreglo.push(query_modificada);
    }
    //se une toda la sección de 'query parameters'
    let resultado = queries_arreglo.join("&");
    info!("Queries modificados: {}", resultado);
}
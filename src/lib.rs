use pyo3::{prelude::*, PyTypeInfo};
use pyo3::exceptions::PyValueError;

fn validate_ip(user_input : &str) -> PyResult<Vec<(String, bool)>>{
    let mut is_correct = true;

    let mut periods = 0;

    for c in user_input.chars() {
        //LOGIC TO CHECK EACH SECTION OF THE IP TO MAKE SURE ITS NOT BELOW 1 
        //OR OVER 255
    }

    let scanned_result : Result<Vec<(String, bool)>, String>;

    if is_correct {
        scanned_result = scan_port(user_input);
    }
    else {
        return Err(PyValueError::new_err("User given ip is invalid"))
    }

    match(scanned_result) {
        Ok(value) => Ok(value),
        Err(e) => Err(PyValueError::new_err(format!("{}", e.to_string()))),
    }
}

fn scan_port(user_input : &str) -> Result<Vec<(String, bool)>, String> {
    //ADD CODE HERE TO SCAN PORT AND MAKE OUTPUT FOR DISPLAYING ON FRONT END
    Ok(Vec::new())
}
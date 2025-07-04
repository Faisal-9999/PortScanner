use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::exceptions::PyValueError;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::time::Duration;
use std::net::{IpAddr, Ipv4Addr};


mod dbhandler;
use dbhandler::DBHandler;

#[pyfunction]
fn validate_ip(user_input : &str) -> PyResult<Vec<(u8, bool)>>{

    let mut periods: u8 = 0;
    let mut ip_quadrant: u128 = 0;
    let mut ip : Vec<u8> = Vec::new();

    for c  in user_input.chars() {

        if c == '.' {

            periods += 1;

            if ip_quadrant > 255 {
                Err(PyValueError::new_err("User given ip is invalid"))?
            }

            ip.push(ip_quadrant as u8);

            ip_quadrant = 0;
            continue;
        }

        
        if (c as u32) < 48 || (c as u32) > 57 {
            Err(PyValueError::new_err("User given ip is invalid"))? 
        }
        
        ip_quadrant = (ip_quadrant * 10) + (u128::from(c) - u128::from('0'));
    }

    if ip_quadrant > 255 {
        return Err(PyValueError::new_err("User given ip is invalid"));
    }

    ip.push(ip_quadrant as u8);

    if periods != 3 {
        Err(PyValueError::new_err("User given ip is invalid"))?
    }

    Ok(scan_port(ip))
}

fn scan_port(ip : Vec<u8>) -> Vec<(u8, bool)> {

    let ports = vec![20, 22, 25, 53, 80, 443];
    let mut result : Vec<(u8, bool)> = Vec::new();

    let address = IpAddr::V4(Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3]));

    for port in ports {
        let addr = SocketAddr::new(address, port);
        let open = TcpStream::connect_timeout(&addr, Duration::from_secs(1)).is_ok();
        result.push((port as u8, open));
    }

    DBHandler::save_to_db(result.clone()).unwrap();

    result
}


#[pymodule]
fn portscanner(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(validate_ip, m)?)?;
    Ok(())
}
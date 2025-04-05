use pyo3::prelude::*;
use pyo3_stub_gen::{
    define_stub_info_gatherer, derive::gen_stub_pyclass_enum, derive::gen_stub_pyfunction,
};

#[gen_stub_pyclass_enum]
#[derive(FromPyObject)]
pub enum IntOrStr {
    #[pyo3(transparent, annotation = "str")]
    String(String),
    #[pyo3(transparent, annotation = "int")]
    Int(isize),
}

#[pyfunction]
#[gen_stub_pyfunction]
pub fn is_odd(n: IntOrStr) -> PyResult<bool> {
    match n {
        IntOrStr::String(s) => Ok(is_odd_rs_core::is_odd(s)),
        IntOrStr::Int(i) => Ok(is_odd_rs_core::is_odd(i.to_string())),
    }
}

#[pyfunction]
#[gen_stub_pyfunction]
pub fn is_even(n: IntOrStr) -> PyResult<bool> {
    match n {
        IntOrStr::String(s) => Ok(is_odd_rs_core::is_even(s)),
        IntOrStr::Int(i) => Ok(is_odd_rs_core::is_even(i.to_string())),
    }
}

#[pymodule]
pub fn is_odd_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_odd, m)?)?;
    m.add_function(wrap_pyfunction!(is_even, m)?)?;
    Ok(())
}
define_stub_info_gatherer!(stub_info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_odd() {
        assert!(is_odd(IntOrStr::Int(1)).unwrap());
        assert!(is_odd(IntOrStr::String("1".to_string())).unwrap());
        assert!(!is_odd(IntOrStr::Int(2)).unwrap());
        assert!(!is_odd(IntOrStr::String("2".to_string())).unwrap());
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(IntOrStr::Int(2)).unwrap());
        assert!(is_even(IntOrStr::String("2".to_string())).unwrap());
        assert!(!is_even(IntOrStr::Int(1)).unwrap());
        assert!(!is_even(IntOrStr::String("1".to_string())).unwrap());

        let stub = stub_info();
        if let Ok(stub) = stub {
            stub.generate().unwrap();
        }
    }
}

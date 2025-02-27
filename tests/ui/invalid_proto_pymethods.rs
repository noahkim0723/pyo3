//! Check that some magic methods edge cases error as expected.
//!
//! For convenience use #[pyo3(name = "__some_dunder__")] to create the methods,
//! so that the function names can describe the edge case to be rejected.

use pyo3::prelude::*;

#[pyclass]
struct MyClass {}

//
// Argument counts
//

#[pymethods]
impl MyClass {
    #[pyo3(name = "__truediv__")]
    fn truediv_expects_one_argument(&self) -> PyResult<()> {
        Ok(())
    }
}

#[pymethods]
impl MyClass {
    #[pyo3(name = "__truediv__")]
    fn truediv_expects_one_argument_py(&self, _py: Python<'_>) -> PyResult<()> {
        Ok(())
    }
}

//
// Forbidden attributes
//

#[pymethods]
impl MyClass {
    #[pyo3(name = "__bool__", signature = ())]
    fn signature_is_forbidden(&self) -> bool {
        true
    }
}

#[pymethods]
impl MyClass {
    #[pyo3(name = "__bool__", text_signature = "")]
    fn text_signature_is_forbidden(&self) -> bool {
        true
    }
}

fn main() {}

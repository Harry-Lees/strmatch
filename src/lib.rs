use levenshtein::levenshtein;
use pyo3::prelude::{pyfunction, pymodule, wrap_pyfunction, PyModule, PyResult, Python};

#[pyfunction]
#[pyo3(name = "distance")]
fn distance_py(n: &str, m: &str) -> usize {
    levenshtein(n, m)
}

#[pyfunction]
#[pyo3(name = "ratio")]
fn ratio_py(n: &str, m: &str) -> f64 {
    let distance = levenshtein(n, m) as f64;
    let max = std::cmp::max(n.chars().count(), m.chars().count()) as f64;
    1.0 - (distance / max)
}

#[pymodule]
fn strmatch(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(distance_py))?;
    m.add_wrapped(wrap_pyfunction!(ratio_py))?;
    Ok(())
}

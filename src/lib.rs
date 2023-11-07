use std::thread::spawn;

use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    println!("test");
    for idx in vec![1, 2, 3, 4].iter() {
        println!("start: {}", idx);
        let _idx_cl = idx.clone();
        let res = spawn(move || {
            println!("threaded: {}", _idx_cl);
        });
        res.join().unwrap();
    };
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn mandelbrot(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

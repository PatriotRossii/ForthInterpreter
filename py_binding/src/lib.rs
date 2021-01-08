use cpython::{PyResult, PyList, NoArgs, Python, py_module_initializer, py_fn, ToPyObject};
use forth_interpreter::ForthInterpreter;

use std::{sync::Mutex};
use once_cell::sync::{Lazy};

static INTERPRETER: Lazy<Mutex<ForthInterpreter>> = Lazy::new(|| Mutex::new(ForthInterpreter::new()));

py_module_initializer!(py_forth, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "execute_line", py_fn!(py, execute_line(line: &str)))?;
    m.add(py, "dump_stack", py_fn!(py, dump_stack()))?;
    Ok(())
});

fn execute_line(_: Python, line: &str) -> PyResult<NoArgs> {
    INTERPRETER.lock().unwrap().execute_line(line);
    Ok(NoArgs)
}

fn dump_stack(py: Python) -> PyResult<PyList> {
    Ok(INTERPRETER.lock().unwrap().get_stack_dump().to_py_object(py))
}
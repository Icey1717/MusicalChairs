use crate::ai;

use entity_gym_rs::agent::TrainEnvBuilder;
use entity_gym_rs::low_level::py_vec_env::PyVecEnv;
use pyo3::prelude::*;

#[derive(Clone)]
#[pyclass]
pub struct Config;

#[pymethods]
impl Config {
    #[new]
    fn new() -> Self {
        Config
    }
}

#[pyfunction]
fn create_env(config: Config, num_envs: usize, threads: usize, first_env_index: u64) -> PyVecEnv {
    TrainEnvBuilder::default()
        .entity::<ai::PlayerState>()
        .entity::<ai::Rectangle>()
        .action::<ai::Direction>()
        .build(
            config,
            super::run_headless,
            num_envs,
            threads,
            first_env_index,
        )
}

#[pymodule]
fn musical_cars_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_env, m)?)?;
    m.add_class::<Config>()?;
    Ok(())
}

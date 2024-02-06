pub struct DataReader {
    paths: Vec<String>
}

impl DataReader {
    pub fn from_paths(paths: Vec<String>) -> Self { Self { paths } }
    pub fn get_return_vectors(&self) -> Vec<Vec<f64>> {
        Vec::new()
    }
}
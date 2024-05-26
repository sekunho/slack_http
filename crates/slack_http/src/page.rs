#[derive(Debug)]
pub struct Page<T> {
    pub(crate) results: Vec<T>,
    pub(crate) cursor: Option<String>,
}

impl<T> Page<T> {
    pub fn results(&self) -> &[T] {
        self.results.as_ref()
    }

    pub fn cursor(&self) -> Option<&str> {
        self.cursor.as_ref().map(|c| c.as_str())
    }

    pub fn new(results: Vec<T>, cursor: Option<String>) -> Self {
        Self { results, cursor }
    }
}

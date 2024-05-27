use serde::Deserialize;

#[derive(Debug)]
pub struct Cursor(pub Option<String>);

#[derive(Debug)]
pub struct Page<T> {
    pub(crate) results: Vec<T>,
    pub(crate) cursor: Cursor,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMetadata {
    pub next_cursor: String,
}

impl<T> Page<T> {
    pub fn results(&self) -> &[T] {
        self.results.as_ref()
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn new(results: Vec<T>, cursor: Cursor) -> Self {
        Self { results, cursor }
    }
}

impl From<ResponseMetadata> for Cursor {
    fn from(value: ResponseMetadata) -> Self {
        if value.next_cursor == "" {
            Self(None)
        } else {
            Self(Some(value.next_cursor))
        }
    }
}

impl Cursor {
    pub fn as_str(&self) -> &str {
        match &self.0 {
            Some(cursor) => cursor.as_str(),
            None => "",
        }
    }
}

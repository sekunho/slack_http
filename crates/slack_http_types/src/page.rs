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
        if value.next_cursor.is_empty() {
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

#[derive(Clone, Copy)]
pub struct Limit(u16);

impl Default for Limit {
    fn default() -> Self {
        Self(100)
    }
}

impl Limit {
    pub fn new(limit: u16) -> Option<Self> {
        if limit <= 1_000 {
            Some(Self(limit))
        } else {
            None
        }
    }

    pub fn get(&self) -> u16 {
        self.0
    }
}

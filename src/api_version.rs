extern crate default;

const DEFAULT_API_VERSION: i64 = 6;

#[derive(Debug)]
pub enum ApiVersion {
    V(i64),
}

impl ApiVersion {
    pub fn to_i64(&self) -> i64 {
        match self {
            ApiVersion::V(v) => *v,
        }
    }
}

impl Default for ApiVersion {
    fn default() -> Self { ApiVersion::V(DEFAULT_API_VERSION) }
}

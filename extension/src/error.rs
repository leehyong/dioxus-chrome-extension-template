#[derive(Debug)]
pub enum SpiderError {
    NotTheSameCategoryXpath(String)
}


pub type SpiderResult<T> = std::result::Result<T, SpiderError>;
use regex::Regex;

#[derive(Debug)]
pub struct CompiledPattern {
    pub raw: String,
    pub directory_only: bool,
    pub regex: Regex,
}

#[derive(Debug)]
pub struct CompiledMatcher {
    pub patterns: Vec<CompiledPattern>,
}



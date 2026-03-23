
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EffectiveIgnoreConfig {
    pub patterns: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeJob {
    pub label: String,
    pub effective_ignore: EffectiveIgnoreConfig,
}
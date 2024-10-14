const DEFAULT_SHELL: &str = "/bin/bash";

#[derive(Debug, Clone)]
pub struct BackendSettings {
    pub shell: String,
    pub args: Vec<String>,
}

impl BackendSettings {
    pub fn new<S: Into<String>, A: Into<Vec<String>>>(
        shell: S,
        args: A,
    ) -> Self {
        Self {
            shell: shell.into(),
            args: args.into(),
        }
    }
}

impl Default for BackendSettings {
    fn default() -> Self {
        Self {
            shell: DEFAULT_SHELL.to_string(),
            args: vec![],
        }
    }
}

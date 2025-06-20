use std::path::Path;

// Configuración de la aplicación
pub struct Config {
    pub db_path: String,
    pub backup_path: Option<String>,
    pub default_password_length: usize,
    pub auto_save: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_path: "vault.db".to_string(),
            backup_path: None,
            default_password_length: 16,
            auto_save: true,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_db_path(mut self, path: &str) -> Self {
        self.db_path = path.to_string();
        self
    }
    
    pub fn get_db_path(&self) -> &str {
        &self.db_path
    }
    
    pub fn db_exists(&self) -> bool {
        Path::new(&self.db_path).exists()
    }
}

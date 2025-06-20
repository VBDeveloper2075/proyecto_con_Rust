mod db;
mod crypto;
mod gui;
mod config;

fn main() {
    // Cargar configuración
    let config = crate::config::Config::new();
    
    // Solicitar clave maestra
    let master_key = crate::crypto::prompt_master_key();
    
    // Inicializar base de datos
    let conn = crate::db::init_db(&config.get_db_path())
        .expect("No se pudo abrir la base de datos");
    
    // Lanzar GUI
    crate::gui::run(conn, master_key);
}
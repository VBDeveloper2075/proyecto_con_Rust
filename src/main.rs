mod db;
mod crypto;
mod gui;

fn main() {
    // Solicitar clave maestra
    let master_key = crate::crypto::prompt_master_key();
    // Inicializar base de datos
    let conn = crate::db::init_db("vault.db").expect("No se pudo abrir la base de datos");
    // Lanzar GUI
    crate::gui::run(conn, master_key);
} 
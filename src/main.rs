mod db;
mod crypto;
mod ui;

use anyhow::{Context, Result};
use db::{Database, Password};
use std::{fs, path::PathBuf};
use serde::{Deserialize, Serialize};

const DB_FILE_NAME: &str = "passwords.db";
const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Serialize, Deserialize)]
struct Config {
    master_password_hash: String,
}

fn get_app_dir() -> PathBuf {
    let mut dir = std::env::current_exe().unwrap();
    dir.pop(); // Elimina el nombre del ejecutable
    dir
}

fn config_path() -> PathBuf {
    get_app_dir().join(CONFIG_FILE_NAME)
}

fn db_path() -> PathBuf {
    get_app_dir().join(DB_FILE_NAME)
}

fn load_config() -> Result<Config> {
    let config_path = config_path();
    if !config_path.exists() {
        println!("Primera ejecución: configurando contraseña maestra.");
        let master_password = ui::prompt_for_new_master_password();
        let master_password_hash = crypto::hash_master_password(&master_password)?;
        
        let config = Config {
            master_password_hash,
        };
        
        let config_str = serde_json::to_string(&config).context("Error al serializar la configuración")?;
        fs::write(&config_path, config_str).context("Error al guardar la configuración")?;
        
        return Ok(config);
    }
    
    let config_str = fs::read_to_string(&config_path).context("Error al leer la configuración")?;
    let config: Config = serde_json::from_str(&config_str).context("Error al deserializar la configuración")?;
    
    Ok(config)
}

fn authenticate() -> Result<bool> {
    let config = load_config()?;
    let master_password = ui::prompt_for_master_password();
    
    crypto::verify_master_password(&master_password, &config.master_password_hash)
}

fn main() -> Result<()> {
    // Autenticación
    match authenticate() {
        Ok(true) => println!("Autenticación exitosa."),
        Ok(false) => {
            println!("Contraseña incorrecta.");
            return Ok(());
        }
        Err(e) => {
            eprintln!("Error durante la autenticación: {:?}", e);
            return Ok(());
        }
    }
    
    // Inicializar base de datos
    let db = Database::new(db_path())?;
    
    loop {
        match ui::main_menu() {
            0 => add_password(&db)?,
            1 => generate_random_password(&db)?,
            2 => list_all_passwords(&db)?,
            3 => search_passwords(&db)?,
            4 => view_password_details(&db)?,
            5 => modify_password(&db)?,
            6 => delete_password(&db)?,
            7 | _ => break,
        }
    }
    
    println!("¡Gracias por usar el gestor de contraseñas!");
    Ok(())
}

fn add_password(db: &Database) -> Result<()> {
    println!("\n--- Añadir Nueva Contraseña ---");
    
    let service = ui::prompt_for_service();
    let username = ui::prompt_for_username();
    let password = ui::prompt_for_password();
    let notes = ui::prompt_for_notes();
    
    let new_password = Password {
        id: None,
        service,
        username,
        password,
        notes,
    };
    
    db.add_password(&new_password)?;
    println!("Contraseña añadida exitosamente.");
    
    Ok(())
}

fn generate_random_password(db: &Database) -> Result<()> {
    println!("\n--- Generar Contraseña Aleatoria ---");
    
    let service = ui::prompt_for_service();
    let username = ui::prompt_for_username();
    let length = ui::prompt_for_password_length();
    let password = crypto::generate_password(length);
    let notes = ui::prompt_for_notes();
    
    println!("Contraseña generada: {}", password);
    
    if ui::prompt_confirm("¿Guardar esta contraseña?") {
        let new_password = Password {
            id: None,
            service,
            username,
            password,
            notes,
        };
        
        db.add_password(&new_password)?;
        println!("Contraseña guardada exitosamente.");
    }
    
    Ok(())
}

fn list_all_passwords(db: &Database) -> Result<()> {
    println!("\n--- Todas las Contraseñas ---");
    
    let passwords = db.get_all_passwords()?;
    ui::display_passwords(&passwords);
    
    Ok(())
}

fn search_passwords(db: &Database) -> Result<()> {
    println!("\n--- Buscar Contraseñas ---");
    
    let query = ui::prompt_for_search();
    let passwords = db.search_passwords(&query)?;
    ui::display_passwords(&passwords);
    
    Ok(())
}

fn view_password_details(db: &Database) -> Result<()> {
    println!("\n--- Ver Detalles de Contraseña ---");
    
    let id = ui::prompt_for_id();
    if let Some(password) = db.get_password(id)? {
        ui::display_password(&password);
    } else {
        println!("No se encontró ninguna contraseña con ese ID.");
    }
    
    Ok(())
}

fn modify_password(db: &Database) -> Result<()> {
    println!("\n--- Modificar Contraseña ---");
    
    let id = ui::prompt_for_id();
    if let Some(mut password) = db.get_password(id)? {
        ui::display_password(&password);
        
        if ui::prompt_confirm("¿Modificar esta contraseña?") {
            password.service = ui::prompt_for_service();
            password.username = ui::prompt_for_username();
            password.password = ui::prompt_for_password();
            password.notes = ui::prompt_for_notes();
            
            db.update_password(&password)?;
            println!("Contraseña actualizada exitosamente.");
        }
    } else {
        println!("No se encontró ninguna contraseña con ese ID.");
    }
    
    Ok(())
}

fn delete_password(db: &Database) -> Result<()> {
    println!("\n--- Eliminar Contraseña ---");
    
    let id = ui::prompt_for_id();
    if let Some(password) = db.get_password(id)? {
        ui::display_password(&password);
        
        if ui::prompt_confirm("¿Estás seguro de que deseas eliminar esta contraseña?") {
            db.delete_password(id)?;
            println!("Contraseña eliminada exitosamente.");
        }
    } else {
        println!("No se encontró ninguna contraseña con ese ID.");
    }
    
    Ok(())
}

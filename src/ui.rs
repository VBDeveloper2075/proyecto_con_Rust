use crate::db::Password;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::io;

pub fn prompt_for_service() -> String {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Nombre del servicio")
        .interact()
        .unwrap()
}

pub fn prompt_for_username() -> String {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Nombre de usuario")
        .interact()
        .unwrap()
}

pub fn prompt_for_password() -> String {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Contraseña")
        .interact()
        .unwrap()
}

pub fn prompt_for_notes() -> Option<String> {
    let notes = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Notas (opcional)")
        .allow_empty(true)
        .interact()
        .unwrap();
    
    if notes.is_empty() {
        None
    } else {
        Some(notes)
    }
}

pub fn prompt_for_master_password() -> String {
    rpassword::prompt_password("Contraseña maestra: ").unwrap()
}

pub fn prompt_for_new_master_password() -> String {
    let password = rpassword::prompt_password("Nueva contraseña maestra: ").unwrap();
    let confirm = rpassword::prompt_password("Confirmar contraseña maestra: ").unwrap();
    
    if password != confirm {
        println!("{}", "Las contraseñas no coinciden. Inténtalo de nuevo.".red());
        return prompt_for_new_master_password();
    }
    
    password
}

pub fn prompt_for_password_length() -> usize {
    Input::<usize>::with_theme(&ColorfulTheme::default())
        .with_prompt("Longitud de la contraseña")
        .default(16)
        .interact()
        .unwrap()
}

pub fn prompt_for_search() -> String {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Buscar")
        .interact()
        .unwrap()
}

pub fn prompt_for_id() -> i64 {
    Input::<i64>::with_theme(&ColorfulTheme::default())
        .with_prompt("ID")
        .interact()
        .unwrap()
}

pub fn prompt_confirm(message: &str) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(message)
        .default(false)
        .interact()
        .unwrap()
}

pub fn display_password(password: &Password) {
    println!();
    println!("ID: {}", password.id.unwrap_or(0).to_string().cyan());
    println!("Servicio: {}", password.service.green());
    println!("Usuario: {}", password.username.yellow());
    println!("Contraseña: {}", password.password.red());
    if let Some(ref notes) = password.notes {
        println!("Notas: {}", notes.blue());
    }
    println!();
}

pub fn display_passwords(passwords: &[Password]) {
    if passwords.is_empty() {
        println!("{}", "No se encontraron contraseñas.".yellow());
        return;
    }
    
    println!("\n{:<5} {:<30} {:<30}", "ID".cyan(), "Servicio".green(), "Usuario".yellow());
    println!("{}", "-".repeat(65));
    
    for password in passwords {
        println!(
            "{:<5} {:<30} {:<30}",
            password.id.unwrap_or(0).to_string().cyan(),
            password.service.green(),
            password.username.yellow()
        );
    }
    println!();
}

pub fn main_menu() -> usize {
    let options = vec![
        "Agregar nueva contraseña",
        "Generar contraseña aleatoria",
        "Ver todas las contraseñas",
        "Buscar contraseñas",
        "Ver detalles de contraseña",
        "Modificar contraseña",
        "Eliminar contraseña",
        "Salir",
    ];
    
    println!("\n{}", "GESTOR DE CONTRASEÑAS".bold().cyan());
    println!("{}", "=".repeat(20));
    
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Seleccione una opción")
        .items(&options)
        .default(0)
        .interact()
        .unwrap()
}

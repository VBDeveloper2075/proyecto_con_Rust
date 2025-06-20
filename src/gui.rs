use rusqlite::Connection;
use crate::db::{PasswordEntry, insert_entry, list_entries, delete_entry, search_entries};
use crate::crypto::{encrypt, decrypt};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use winapi::um::winuser::{OpenClipboard, EmptyClipboard, SetClipboardData, CloseClipboard};
use winapi::um::winbase::{GlobalAlloc, GlobalLock, GlobalUnlock, GlobalFree};
use winapi::um::winuser::CF_UNICODETEXT;
use winapi::um::winnt::HANDLE;
use winapi::um::minwinbase::GMEM_MOVEABLE;

pub fn run(conn: Connection, master_key: String) {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Gestor de Contraseñas",
        options,
        Box::new(|_cc| Box::new(PasswordApp::new(conn, master_key))),
    ).unwrap();
}

pub struct PasswordApp {
    conn: Connection,
    master_key: String,
    entries: Vec<PasswordEntry>,
    // Form fields
    app: String,
    url: String,
    usuario: String,
    mail: String,
    con: String,
    notas: String,
    search_query: String,
    error: Option<String>,
}

impl PasswordApp {    pub fn new(conn: Connection, master_key: String) -> Self {
        let entries = list_entries(&conn).unwrap_or_default();
        Self {
            conn,
            master_key,
            entries,
            app: String::new(),
            url: String::new(),
            usuario: String::new(),
            mail: String::new(),
            con: String::new(),
            notas: String::new(),
            search_query: String::new(),
            error: None,
        }
    }

    fn add_entry(&mut self) {
        match encrypt(&self.master_key, &self.con) {
            Ok(enc_con) => {
                let entry = PasswordEntry::new(
                    &self.app, &self.url, &self.usuario, &self.mail, &enc_con, &self.notas
                );
                if let Err(e) = insert_entry(&self.conn, &entry) {
                    self.error = Some(format!("Error al guardar: {}", e));
                } else {
                    self.entries = list_entries(&self.conn).unwrap_or_default();
                    self.app.clear(); self.url.clear(); self.usuario.clear();
                    self.mail.clear(); self.con.clear(); self.notas.clear();
                    self.error = None;
                }
            }
            Err(e) => self.error = Some(format!("Error cifrando: {}", e)),
        }
    }

    fn delete_entry(&mut self, id: &str) {
        if let Err(e) = delete_entry(&self.conn, id) {
            self.error = Some(format!("Error al borrar: {}", e));
        } else {
            self.entries = list_entries(&self.conn).unwrap_or_default();
        }
    }

    fn copy_to_clipboard(&mut self, text: &str) -> Result<(), String> {
        // Convertir el string a UTF-16 para Windows
        let text_utf16: Vec<u16> = OsStr::new(text)
            .encode_wide()
            .chain(std::iter::once(0)) // Null-termination
            .collect();
        
        unsafe {
            // Abrir el portapapeles
            if OpenClipboard(ptr::null_mut()) == 0 {
                return Err("No se pudo abrir el portapapeles".to_string());
            }
            
            // Vaciar el portapapeles
            EmptyClipboard();
            
            // Asignar memoria global
            let size = text_utf16.len() * std::mem::size_of::<u16>();
            let h_glob = GlobalAlloc(GMEM_MOVEABLE, size);
            if h_glob.is_null() {
                CloseClipboard();
                return Err("Error al asignar memoria".to_string());
            }
            
            // Copiar el texto a la memoria global
            let p_glob = GlobalLock(h_glob) as *mut u16;
            if p_glob.is_null() {
                GlobalFree(h_glob);
                CloseClipboard();
                return Err("Error al bloquear memoria".to_string());
            }
            
            ptr::copy_nonoverlapping(text_utf16.as_ptr(), p_glob, text_utf16.len());
            
            GlobalUnlock(h_glob);
            
            // Establecer los datos en el portapapeles
            let h_result = SetClipboardData(CF_UNICODETEXT, h_glob);
            
            CloseClipboard();
            
            if h_result.is_null() {
                GlobalFree(h_glob);
                return Err("No se pudo establecer el contenido del portapapeles".to_string());
            }
        }
        
        Ok(())
    }
}

impl eframe::App for PasswordApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Gestor de Contraseñas");
            if let Some(err) = &self.error {
                ui.colored_label(egui::Color32::RED, err);
            }
            ui.separator();
            ui.label("Agregar nueva entrada:");
            ui.horizontal(|ui| {
                ui.label("App:"); ui.text_edit_singleline(&mut self.app);
                ui.label("URL:"); ui.text_edit_singleline(&mut self.url);
            });
            ui.horizontal(|ui| {
                ui.label("Usuario:"); ui.text_edit_singleline(&mut self.usuario);
                ui.label("Mail:"); ui.text_edit_singleline(&mut self.mail);
            });
            ui.horizontal(|ui| {
                ui.label("Contraseña:"); ui.text_edit_singleline(&mut self.con);
                ui.label("Notas:"); ui.text_edit_singleline(&mut self.notas);
            });
            if ui.button("Guardar").clicked() {
                self.add_entry();
            }            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Buscar:");
                ui.text_edit_singleline(&mut self.search_query);
                if ui.button("Buscar").clicked() {
                    if self.search_query.is_empty() {
                        self.entries = list_entries(&self.conn).unwrap_or_default();
                    } else {
                        match search_entries(&self.conn, &self.search_query) {
                            Ok(results) => self.entries = results,
                            Err(e) => self.error = Some(format!("Error en búsqueda: {}", e)),
                        }
                    }
                }
                if ui.button("Mostrar todo").clicked() {
                    self.search_query.clear();
                    self.entries = list_entries(&self.conn).unwrap_or_default();
                }
            });
            ui.separator();
            ui.label("Entradas guardadas:");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for entry in &self.entries {
                    ui.group(|ui| {
                        ui.label(format!("App: {} | Usuario: {} | Mail: {}", entry.app, entry.usuario, entry.mail));
                        ui.label(format!("URL: {}", entry.url));
                        ui.label(format!("Notas: {}", entry.notas));                        let password_result = decrypt(&self.master_key, &entry.con);
                        if let Ok(pass) = &password_result {
                            ui.horizontal(|ui| {
                                ui.label(format!("Contraseña: {}", pass));
                                if ui.button("📋 Copiar").clicked() {
                                    match self.copy_to_clipboard(pass) {
                                        Ok(_) => self.error = None,
                                        Err(e) => self.error = Some(format!("Error al copiar: {}", e)),
                                    }
                                }
                            });
                        } else {
                            ui.colored_label(egui::Color32::RED, "Error descifrando contraseña");
                        }
                        ui.horizontal(|ui| {
                            if ui.button("Eliminar").clicked() {
                                self.delete_entry(&entry.id);
                            }
                        });
                    });
                    ui.separator();
                }
            });
        });
    }
}
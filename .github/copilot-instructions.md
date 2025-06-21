<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

# Gestor de Contraseñas en Rust

Este proyecto es un gestor de contraseñas desarrollado en Rust que utiliza SQLite para el almacenamiento de datos. La aplicación está diseñada para ser portable, con todo empaquetado en un único ejecutable.

## Estructura del proyecto

- **src/main.rs**: Punto de entrada principal de la aplicación y lógica de control
- **src/db.rs**: Módulo para la interacción con la base de datos SQLite
- **src/crypto.rs**: Funciones de criptografía y seguridad
- **src/ui.rs**: Interfaz de usuario y funciones de entrada/salida

## Convenciones de código

- Usar nombres descriptivos para funciones y variables
- Documentar funciones y módulos con comentarios
- Manejar errores con anyhow y thiserror
- Seguir las convenciones del estilo de código de Rust

## Notas de seguridad

- Las contraseñas deben estar protegidas con hashing adecuado
- La contraseña maestra utiliza Argon2 para hash
- La base de datos SQLite se almacena localmente

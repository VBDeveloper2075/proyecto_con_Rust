# VECLA - Gestor de Contraseñas Portable

Vecla es un gestor de contraseñas portable y seguro desarrollado en Rust. Este programa te permite almacenar y gestionar tus contraseñas de manera cifrada, todo en un único archivo de base de datos SQLite.

## Características

- Almacenamiento cifrado de contraseñas usando AES-256-GCM
- Interfaz gráfica sencilla e intuitiva
- Búsqueda rápida de credenciales
- Generación de contraseñas seguras
- Copia de contraseñas al portapapeles
- Sin instalación, completamente portable

## Cómo usar

1. Ejecuta el archivo `vecla.exe`
2. Introduce tu clave maestra (esta clave se usará para cifrar y descifrar tus contraseñas)
3. Usa la interfaz para añadir, buscar o eliminar entradas de contraseñas

## Compilación

Para compilar el proyecto:

```
cargo build --release
```

El ejecutable se generará en la carpeta `target/release`.

## Seguridad

- Todas las contraseñas se almacenan cifradas con AES-256-GCM
- La clave maestra nunca se guarda, solo se usa para cifrar y descifrar
- La base de datos SQLite está cifrada

## Estructura del Proyecto

- `main.rs` - Punto de entrada de la aplicación
- `crypto.rs` - Módulo para cifrado y descifrado
- `db.rs` - Gestión de la base de datos
- `gui.rs` - Interfaz gráfica
- `config.rs` - Configuración de la aplicación

## Licencia

Este proyecto es software libre.

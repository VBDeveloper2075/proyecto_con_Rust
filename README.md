# Gestor de Contraseñas en Rust

Una aplicación de gestión de contraseñas portable desarrollada en Rust que utiliza SQLite como base de datos.

## Características

- Almacenamiento seguro de contraseñas utilizando SQLite
- Protección con contraseña maestra usando algoritmo Argon2
- Generación de contraseñas aleatorias y seguras
- Búsqueda rápida de credenciales
- Interfaz de línea de comandos interactiva
- Aplicación portable (todo en un único ejecutable)

## Compilación

Para compilar el proyecto desde el código fuente:

```bash
cargo build --release
```

El ejecutable generado estará en `target/release/password_manager.exe`.

## Uso

1. Ejecute la aplicación: `password_manager.exe`
2. En el primer uso, se le pedirá crear una contraseña maestra
3. En los siguientes usos, deberá ingresar esta contraseña maestra para acceder
4. Use el menú interactivo para gestionar sus contraseñas

### Opciones del menú

- **Agregar nueva contraseña**: Almacena manualmente un nuevo conjunto de credenciales
- **Generar contraseña aleatoria**: Crea una contraseña segura de la longitud deseada
- **Ver todas las contraseñas**: Muestra una lista con todas las contraseñas guardadas
- **Buscar contraseñas**: Encuentra credenciales según el servicio o nombre de usuario
- **Ver detalles de contraseña**: Muestra todos los detalles de una contraseña específica
- **Modificar contraseña**: Actualiza una contraseña existente
- **Eliminar contraseña**: Borra permanentemente una contraseña
- **Salir**: Cierra la aplicación

## Seguridad

- La contraseña maestra se almacena con hash Argon2
- La base de datos SQLite se guarda en el mismo directorio que el ejecutable

## Portabilidad

Esta aplicación está diseñada para ser portable. Puede copiar el ejecutable junto con los archivos `passwords.db` y `config.json` a un dispositivo USB para llevar sus contraseñas de forma segura.

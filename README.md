# 🔒 VECLA - Gestor de Contraseñas Portable

<div align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/SQLite-07405E?style=for-the-badge&logo=sqlite&logoColor=white" alt="SQLite" />
  <img src="https://img.shields.io/badge/AES--256-4285F4?style=for-the-badge&logo=google-cloud&logoColor=white" alt="AES-256" />
  <img src="https://img.shields.io/badge/Portable-00C7B7?style=for-the-badge&logo=netlify&logoColor=white" alt="Portable" />
  <br/>
  <strong>Una solución portable y segura para gestionar tus contraseñas</strong>
</div>

<br/>

## 📋 Contenido
- [Descripción](#-descripción)
- [Características](#-características)
- [Requisitos](#-requisitos)
- [Instalación](#-instalación)
- [Uso](#-uso)
- [Seguridad](#-seguridad)
- [Estructura del Proyecto](#-estructura-del-proyecto)
- [Capturas de Pantalla](#-capturas-de-pantalla)
- [Licencia](#-licencia)
- [Contribuciones](#-contribuciones)

<br/>

## 📝 Descripción

**Vecla** es un gestor de contraseñas portable y seguro desarrollado en Rust. Este programa te permite almacenar y gestionar tus contraseñas de manera cifrada, todo en un único archivo de base de datos SQLite. Ideal para llevar en un pendrive o usar en múltiples dispositivos sin necesidad de instalación.

<br/>

## ✨ Características

- 🛡️ **Cifrado avanzado**: Almacenamiento seguro con AES-256-GCM
- 🖥️ **Interfaz gráfica**: Diseño sencillo e intuitivo
- 🔍 **Búsqueda rápida**: Encuentra rápidamente tus credenciales
- 🎲 **Generación de contraseñas**: Crea contraseñas seguras con un solo clic
- 📋 **Integración con portapapeles**: Copia contraseñas con un clic
- 📱 **Portabilidad total**: Sin instalación, ejecuta desde cualquier lugar
- 🔄 **Base de datos unificada**: Todo en un solo archivo portable

<br/>

## 🛠️ Requisitos

- Sistema operativo: Windows 7/8/10/11
- Espacio en disco: Menos de 10 MB
- Memoria RAM: Mínimo 50 MB

<br/>

## 📥 Instalación

### Opción 1: Descargar el ejecutable precompilado

1. Descarga `vecla.exe` desde la sección de [Releases](https://github.com/tuusuario/vecla/releases)
2. ¡Listo para usar! No se requiere instalación

### Opción 2: Compilar desde el código fuente

```bash
# Clonar el repositorio
git clone https://github.com/tuusuario/vecla.git
cd vecla

# Compilar en modo release
cargo build --release

# El ejecutable estará en target/release/vecla.exe
```

<br/>

## 📖 Uso

### Primer inicio

1. Ejecuta `vecla.exe`
2. Introduce tu clave maestra (¡Recuérdala bien! No hay recuperación)
3. La base de datos `vault.db` se creará automáticamente

### Gestión de contraseñas

- **Añadir una nueva entrada**:
  1. Rellena los campos (App, URL, Usuario, etc.)
  2. Usa el botón "Generar" para crear una contraseña segura
  3. Haz clic en "Guardar"

- **Buscar entradas**:
  1. Escribe en el campo de búsqueda
  2. Pulsa "Buscar"
  3. Para ver todas las entradas, haz clic en "Mostrar todo"

- **Copiar contraseñas**:
  1. Haz clic en "📋 Copiar" junto a la contraseña
  2. Pega en la aplicación deseada (Ctrl+V)

- **Eliminar entradas**:
  1. Haz clic en "Eliminar" en la entrada que deseas borrar

<br/>

## 🔐 Seguridad

- **Cifrado AES-256-GCM**: Estándar de cifrado de grado militar
- **Clave maestra**: Nunca se almacena, solo se usa para cifrar/descifrar
- **Derivación de claves**: Algoritmo mejorado para máxima seguridad
- **Base de datos cifrada**: Los datos están protegidos en reposo
- **Portapapeles seguro**: Interacción segura con el portapapeles del sistema

<br/>

## 🏗️ Estructura del Proyecto

```
vecla/
├── src/
│   ├── main.rs      # Punto de entrada de la aplicación
│   ├── crypto.rs    # Funciones de cifrado y seguridad
│   ├── db.rs        # Gestión de la base de datos SQLite
│   ├── gui.rs       # Interfaz gráfica con eframe/egui
│   └── config.rs    # Gestión de configuración
├── .vscode/         # Configuración para VSCode
├── Cargo.toml       # Dependencias y metadatos
└── Cargo.lock       # Versiones exactas de dependencias
```

<br/>

## 🖼️ Capturas de Pantalla

> *Aquí irían capturas de pantalla de la aplicación en uso*

<br/>

## 📜 Licencia

Este proyecto es software libre y de código abierto bajo los términos de la licencia MIT.

<br/>

## 👥 Contribuciones

¡Las contribuciones son bienvenidas! Si deseas contribuir:

1. Haz un fork del repositorio
2. Crea una rama para tu característica (`git checkout -b feature/amazing-feature`)
3. Commitea tus cambios (`git commit -m 'Add: amazing feature'`)
4. Haz push a la rama (`git push origin feature/amazing-feature`)
5. Abre un Pull Request

---

<div align="center">
  <sub>Desarrollado con ❤️ en Rust</sub>
</div>

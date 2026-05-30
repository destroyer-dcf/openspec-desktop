## Why

Actualmente la app siempre arranca con un tamaño base y no recuerda el tamaño elegido por el usuario al cerrar. Esto empeora la experiencia de escritorio porque obliga a redimensionar la ventana en cada inicio.

## What Changes

- Guardar ancho y alto de la ventana principal al cerrarse la aplicación.
- Restaurar esas dimensiones al siguiente arranque antes de mostrar contenido interactivo.
- Definir validaciones de seguridad para evitar dimensiones inválidas o fuera de límites mínimos.
- Mantener compatibilidad con el mínimo de ventana ya definido por la app.

## Capabilities

### New Capabilities
- `persistencia-dimension-ventana`: Persistir y restaurar dimensiones de la ventana principal entre sesiones.

### Modified Capabilities
- `tema-aplicacion`: Extender el comportamiento de persistencia de preferencias de UI para incluir geometría de ventana en el arranque.

## Impact

- Backend Tauri (Rust): lectura/escritura de dimensiones y aplicación en ventana principal.
- Configuración local/estado de aplicación: nuevo dato persistido.
- UX de escritorio: inicio de la app en el último tamaño usado por el usuario.

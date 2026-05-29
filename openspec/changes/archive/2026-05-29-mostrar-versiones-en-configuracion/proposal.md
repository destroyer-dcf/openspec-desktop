## Why

Falta visibilidad operativa sobre versiones instaladas: al diagnosticar incidencias o diferencias de comportamiento no se ve rápidamente qué versión de OpenSpec CLI ni qué versión de la app de escritorio está ejecutando el usuario. Mostrar ambas en Configuración reduce fricción de soporte y validación.

## What Changes

- Añadir un panel superior en el modal de Configuración que muestre:
  - Versión de OpenSpec CLI (consola)
  - Versión de la aplicación de escritorio
- Obtener versión de OpenSpec CLI invocando el comando de versión del binario disponible.
- Obtener versión de la app desde metadatos de build/runtime.
- Definir fallback claro cuando OpenSpec CLI no esté disponible.

## Capabilities

### New Capabilities
- `panel-versiones-configuracion`: Panel informativo de versiones en la parte superior de Configuración.

### Modified Capabilities
- `configuracion-densidad-y-color`: Extiende el modal con bloque de información técnica de versiones.
- `tema-aplicacion`: Mantiene coherencia visual del nuevo panel en temas light/dark.

## Impact

- Backend Tauri (nuevo comando para devolver versiones).
- `SettingsModal.svelte` (nuevo panel superior de versiones).
- Tipos/frontend para transportar y renderizar datos de versión.

## 1. Metadata de About en Tauri

- [x] 1.1 Localizar en `src-tauri/src/lib.rs` (o módulo equivalente) la construcción actual de `AboutMetadata`.
- [x] 1.2 Añadir el campo de copyright con valor exacto `Copyright Destroyer 2026`.
- [x] 1.3 Añadir el campo de website/repositorio con valor exacto `https://github.com/destroyer-dcf/openspec-desktop`.

## 2. Consistencia de identidad y validación funcional

- [x] 2.1 Verificar que el nombre de producto se mantiene como `OpenSpec Desktop` en About tras los cambios.
- [x] 2.2 Revisar que el panel de versiones/configuración no entra en conflicto con los metadatos institucionales de About.
- [x] 2.3 Ejecutar la app (`npm run tauri dev`) y validar manualmente que About muestra versión, copyright y URL.

## 3. Cierre técnico

- [x] 3.1 Confirmar que el proyecto compila sin errores tras la modificación del backend Tauri.
- [x] 3.2 Documentar en la descripción del cambio cualquier limitación de plataforma detectada en About (si aplica).

## Why

La ventana `About` de la aplicacion no muestra informacion legal ni una referencia directa al repositorio oficial. Esto reduce trazabilidad del producto y complica identificar autoria/version fuente desde la UI.

## What Changes

- Añadir texto de copyright visible en `About`: `Copyright Destroyer 2026`.
- Mostrar la URL del repositorio oficial en `About`: `https://github.com/destroyer-dcf/openspec-desktop`.
- Mantener el nombre de producto actual (`OpenSpec Desktop`) y el estilo nativo del dialogo.
- Asegurar que la informacion se aplica en el flujo donde se construye `AboutMetadata` en Tauri.

## Capabilities

### New Capabilities
- `about-metadata-legal`: Mostrar metadatos legales y de repositorio en la ventana About.

### Modified Capabilities
- `panel-versiones-configuracion`: Alinear la fuente de informacion visible de version/identidad con los metadatos mostrados en About.

## Impact

- Backend Tauri (Rust): configuracion de `AboutMetadata` y menu nativo.
- UX de escritorio: mejora de informacion institucional visible al usuario.
- Sin cambios en APIs externas ni dependencias nuevas.

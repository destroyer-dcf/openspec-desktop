## Context

La aplicacion usa Tauri con menu nativo y configura `AboutMetadata` desde el backend Rust. Actualmente `About` no incluye datos de copyright ni URL del repositorio, aunque el producto ya esta identificado como `OpenSpec Desktop`.

## Goals / Non-Goals

**Goals:**
- Incluir copyright fijo visible en About: `Copyright Destroyer 2026`.
- Incluir URL del repositorio visible en About: `https://github.com/destroyer-dcf/openspec-desktop`.
- Mantener comportamiento nativo multiplataforma sin introducir dependencias nuevas.
- Reutilizar el punto central actual donde se construye `AboutMetadata`.

**Non-Goals:**
- Rediseñar visualmente la ventana About.
- Añadir enlaces clicables personalizados fuera de capacidades del About nativo.
- Cambiar el flujo de versionado de la aplicacion.

## Decisions

- Definir los nuevos campos legales en la construccion de `AboutMetadata` en Rust.
Rationale: es la fuente de verdad del dialogo nativo y evita duplicaciones en frontend.

- Mantener la URL en formato texto estable (string hardcoded de repositorio oficial).
Rationale: evita dependencia de runtime externo y garantiza consistencia entre builds.

- No añadir capa de configuracion para estos campos.
Rationale: son metadatos institucionales, no preferencias de usuario.

Alternativas consideradas:
- Cargar URL/copyright desde `package.json` o config externa: descartado por complejidad y bajo valor para este caso.
- Mostrar estos datos solo en modal de configuracion: descartado porque el requerimiento es especifico de About.

## Risks / Trade-offs

- [Diferencias por plataforma en render de About] → Validar manualmente en macOS (actual) y mantener fallback de texto simple.
- [Campos no soportados en alguna plataforma] → Mantener al menos el contenido en los campos soportados por Tauri y documentar limitaciones.

## Validation Notes

- No se detectaron limitaciones adicionales de plataforma en compilación (`cargo check`) ni en arranque (`npm run tauri dev`) durante esta implementación.
- La verificación visual final de contenido en el diálogo About queda como validación manual del usuario.

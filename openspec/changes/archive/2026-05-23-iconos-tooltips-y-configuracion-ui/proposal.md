## Why

La UI todavía tiene botones sin icono ni ayuda contextual, y el tamaño visual general resulta grande para flujos intensivos. Necesitamos mejorar densidad y claridad con iconografía consistente, tooltips y opciones de configuración de tipografía/botones.

## What Changes

- Añadir icono a todos los botones interactivos de la aplicación.
- Añadir tooltip descriptivo en todos los botones con acciones visibles.
- Reducir tamaño de texto global para mayor densidad visual de herramienta de desarrollo.
- Añadir en modal de configuración control para tamaño de texto (al menos `compact` y `normal`).
- Añadir en modal de configuración selector de color de botones con opciones iniciales `blue` y `green`.
- Aplicar cambios de tipografía y color de botones en caliente y persistir preferencias.

## Capabilities

### New Capabilities
- `ui-controles-botones`: iconografía y tooltips homogéneos en botones de toda la app.
- `configuracion-densidad-y-color`: configuración de tamaño de texto y color de botones desde modal.

### Modified Capabilities
- `tema-aplicacion`: ampliar configuración visual para incluir densidad tipográfica y variante cromática de botones.

## Impact

- Frontend Svelte: componentes de botones, sidebar, dashboard, modal configuración, wizard/editor.
- Sistema de estilos: nuevos tokens para escala tipográfica y variantes de botón (`blue`/`green`).
- Persistencia local: guardar `fontScale` y `buttonColor` junto a tema.
- Accesibilidad: tooltips consistentes y `aria-label` alineados con acción real.

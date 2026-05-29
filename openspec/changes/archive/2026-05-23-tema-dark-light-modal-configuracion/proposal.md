## Why

La aplicación todavía no permite elegir tema visual desde la UI, lo que limita legibilidad y preferencia de uso en distintos entornos. Necesitamos soporte explícito dark/light alineado con estilo GitHub para mantener consistencia visual y mejorar experiencia diaria.

## What Changes

- Añadir en el modal de configuración un selector de tema con opciones `light` y `dark`.
- Aplicar el tema seleccionado de forma global en la app usando tokens de color (sin hardcodes) basados en `STYLEGUIDELINES.md`.
- Persistir la preferencia de tema para restaurarla al reiniciar la aplicación.
- Actualizar componentes principales (sidebar, dashboard, paneles, inputs, botones) para respetar tokens de tema GitHub-like en ambos modos.

## Capabilities

### New Capabilities
- `tema-aplicacion`: gestión de selección, aplicación y persistencia del tema `light/dark` desde modal de configuración.

### Modified Capabilities
- `dashboard-estado`: adaptación de la vista principal para renderizar colores, bordes y contraste según tema activo.

## Impact

- Frontend Svelte: modal de configuración, layout global y componentes visuales base.
- Estado de app: almacenamiento de preferencia de tema y restauración al arrancar.
- Estilos CSS: introducción de variables/tokens semánticos GitHub-inspired para dark/light.
- Sin cambios en lógica de dominio OpenSpec ni parsing de proyectos.

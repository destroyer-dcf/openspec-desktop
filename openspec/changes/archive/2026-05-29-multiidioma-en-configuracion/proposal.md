## Why

La interfaz está fija en español y limita adopción para equipos internacionales. Añadir soporte multiidioma configurable desde el modal permite usar la app en distintos contextos sin cambiar código ni reiniciar flujos.

## What Changes

- Añadir soporte de idioma de interfaz para: Inglés, Francés, Alemán y Portugués.
- Incluir selector de idioma en el modal de configuración.
- Aplicar traducciones en caliente en la UI y persistir preferencia entre sesiones.
- Mantener fallback seguro a idioma por defecto cuando falte una clave.

## Capabilities

### New Capabilities
- `internacionalizacion-ui`: Sistema de traducciones para textos de interfaz con diccionarios por idioma.

### Modified Capabilities
- `configuracion-densidad-y-color`: Extiende configuración con selector de idioma.
- `tema-aplicacion`: Mantiene coherencia de preferencias globales incorporando idioma como preferencia persistida.

## Impact

- Frontend Svelte: capa i18n, uso de claves en componentes visibles y settings modal.
- Persistencia local de preferencias UI.
- Tipos/utilidades para idioma activo y fallback.

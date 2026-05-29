## Why

En el dashboard, las tarjetas de cambios activos y archivados muestran el nombre pero no el contexto funcional inmediato del cambio. Mostrar el texto `Why` de `proposal.md` mejora la comprensión rápida del objetivo de cada cambio sin abrir documentos.

## What Changes

- Extraer y mostrar el bloque `Why` de `proposal.md` debajo del título en tarjetas de cambios activos.
- Aplicar el mismo comportamiento en tarjetas de cambios archivados.
- Añadir fallback amigable cuando `proposal.md` no existe o no tiene sección `Why`.
- Truncar visualmente a 2 líneas para mantener densidad y legibilidad.

## Capabilities

### New Capabilities
- `resumen-why-en-tarjetas-cambio`: Presentación de resumen `Why` en tarjetas de cambios activos y archivados.

### Modified Capabilities
- `dashboard-estado`: Se amplía el contenido de tarjeta para incluir contexto textual del cambio.
- `consulta-cambios-archivados`: Las tarjetas archivadas muestran el mismo resumen `Why` que activos.

## Impact

- Backend de carga de cambios para extraer `Why` desde `proposal.md`.
- Tipado de `Change` y render en `Dashboard.svelte`.
- Estilos UI para preview textual de 2 líneas en ambos paneles.

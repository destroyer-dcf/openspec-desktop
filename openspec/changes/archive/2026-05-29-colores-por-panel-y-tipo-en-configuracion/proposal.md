## Why

Las tarjetas de cambios activos, cambios archivados y propuestas no transmiten de forma consistente el estado visual por tipo/resultado, y hoy no existe un control centralizado para ajustar esos colores. Esto reduce la lectura rápida del pipeline del proyecto y dificulta adaptar la UI a preferencias del usuario.

## What Changes

- Añadir configuración de color por panel/tipo en el modal de configuración.
- Permitir asignar color para tarjetas completadas y pendientes en cambios activos.
- Permitir asignar color para tarjetas por tipo en propuestas (feature/bug).
- Permitir asignar color para tarjetas en cambios archivados.
- Incluir opción explícita de “sin color” para cada selector.
- Aplicar el mismo estilo visual del estado seleccionado del sidebar (fondo azul + borde/contraste equivalente) como formato base para los estados coloreados.

## Capabilities

### New Capabilities
- `configuracion-colores-por-panel`: Gestión centralizada de colores de tarjetas por panel y tipo, incluyendo opción sin color.

### Modified Capabilities
- `dashboard-estado`: Cambia la presentación visual de tarjetas de cambios activos/archivados según configuración.
- `gestion-propuestas`: Cambia la presentación visual de tarjetas de propuestas según tipo y configuración.
- `configuracion-densidad-y-color`: Amplía el modal de configuración para controlar colores por panel/tipo.
- `sidebar-proyectos`: Reutiliza el patrón visual del seleccionado como referencia de estilo para los estados coloreados.

## Impact

- Frontend Svelte de paneles de contenido y modal de configuración.
- Modelo de preferencias persistidas (estructura de configuración de color).
- Estilos CSS variables/tokens para variantes de tarjetas por estado/tipo.
- Validaciones de fallback para modo “sin color”.

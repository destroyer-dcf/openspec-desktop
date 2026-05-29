## Why

El botón de configuración del sidebar no sigue la jerarquía visual deseada para una app de escritorio: parece un botón primario completo y su alineación no prioriza lectura rápida. Se necesita un estilo más liviano y consistente con acciones de navegación lateral.

## What Changes

- Alinear el texto del control de configuración a la izquierda dentro del footer del sidebar.
- Eliminar apariencia de botón con borde permanente en configuración.
- Mantener interacción visual únicamente en `hover`/`focus`.
- Sustituir el icono actual por icono de rueda dentada.

## Capabilities

### New Capabilities
- `configuracion-sidebar-estilo-ligero`: Define el patrón visual ligero del acceso a configuración en el footer del sidebar.

### Modified Capabilities
- `sidebar-proyectos`: Ajusta el comportamiento visual y alineación del acceso de configuración en el footer.
- `sistema-iconos`: Cambia el icono del acceso a configuración por rueda dentada.

## Impact

- Componente del sidebar (estructura del footer y clases CSS del control de configuración).
- Estilos del botón/enlace de configuración (border/background/hover/focus/alineación).
- Iconografía del control de configuración.

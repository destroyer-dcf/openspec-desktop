## Por qué

El dashboard actual es funcional pero visualmente plano. No hay iconos que diferencien artifacts, acciones o secciones, y el progreso de las tareas se muestra como texto sin ninguna representación visual. Sin un sistema de iconos consistente y sin indicadores visuales de progreso, la app parece inacabada y es difícil leer el estado de un proyecto de un vistazo.

## Qué Cambia

- Se añade `lucide-svelte` como librería de iconos: iconos en la barra lateral, dashboard, acciones y estado de artifacts.
- Los artifacts muestran un icono de estado: completado (✓), pendiente (○) o bloqueado (⊘).
- Se añade un indicador de progreso circular global que muestra el porcentaje de tareas completadas del proyecto activo.
- Se añade una barra de progreso lineal por cambio que muestra `X/Y tareas`.
- El dashboard pasa a un layout de dos columnas: cambios activos a la izquierda, resumen global y progreso circular a la derecha.
- Los indicadores se actualizan en tiempo real cuando los ficheros cambian en disco.

## Capacidades

### Nuevas Capacidades
- `sistema-iconos`: Sistema de iconos visual consistente usando `lucide-svelte`. Cubre navegación, acciones, secciones y estado de artifacts (✓/○/⊘).
- `progreso-proyecto`: Indicadores visuales de progreso: circular global (% del total de tareas activas) y barra lineal por cambio (X/Y tareas). Se actualiza en tiempo real.

### Capacidades Modificadas
- `dashboard-estado`: El dashboard adopta un layout de dos columnas con iconos y barras de progreso integrados. Muestra "Sin tareas" cuando un cambio no tiene `tasks.md`.

## Impacto

- Solo afecta al frontend (Svelte); no hay cambios en Rust ni en la API de Tauri commands.
- Nueva dependencia frontend: `lucide-svelte` (tree-shakeable, MIT, ~2KB por icono utilizado).
- El progreso circular es un componente SVG custom (sin librerías extra), usando `stroke-dasharray`/`stroke-dashoffset`.
- El progreso por cambio usa el elemento nativo `<progress>` con estilos CSS.

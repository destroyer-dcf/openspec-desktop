## Why

El estado global del proyecto hoy se entiende con un rosco centrado en porcentaje de tareas, pero falta contexto ejecutivo (totales, distribución y volumen). Un gráfico de barras con métricas agregadas mejora lectura operativa y comparación rápida entre indicadores del proyecto.

## What Changes

- Sustituir el resumen circular global por un panel de estado general con visualización principal en barras.
- Mostrar métricas agregadas del proyecto (por ejemplo: total de cambios, cambios activos/archivados, total de tareas y tareas completadas).
- Mantener coherencia visual con el panel de descripción del proyecto y layout actual del dashboard.
- Definir comportamiento para estado sin datos (proyecto vacío) sin errores.

## Capabilities

### New Capabilities
- `estado-general-proyecto-barras`: Visualización agregada del estado del proyecto mediante gráfico de barras y contadores.

### Modified Capabilities
- `progreso-proyecto`: Cambia la representación global de progreso (de rosco a barras/métricas agregadas).
- `panel-descripcion-proyecto`: Integra el nuevo bloque de estado general en la zona derecha del panel descriptivo.
- `dashboard-estado`: Ajusta la lectura de estado global en el pipeline para reflejar métricas de barras.

## Impact

- Componente de descripción/resumen del proyecto en `content`.
- Lógica de agregación para métricas globales (cambios y tareas).
- Estilos del bloque de estado global y comportamiento responsive de escritorio.

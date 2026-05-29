## Context

Actualmente las tarjetas de cambios activos muestran dos elementos de progreso: barra lineal `X/Y tareas` y un rosco de porcentaje. En escenarios con muchas tarjetas, el rosco introduce ruido visual y resta espacio útil para acciones y lectura rápida.

La base funcional de cálculo de progreso ya existe y está validada en el flujo de carga de cambios (`completedTasks`, `totalTasks`, porcentaje derivado). Por tanto, el cambio es puramente de presentación en el panel de cambios activos.

## Goals / Non-Goals

**Goals:**
- Sustituir el rosco de porcentaje por un indicador textual grande y en negrita dentro de la tarjeta.
- Reubicar el porcentaje a la derecha de la zona de acciones, conservando jerarquía visual.
- Mantener intacta la lógica de cálculo de progreso y el texto `X/Y tareas`.
- Evitar regresiones en cambios archivados, propuestas y resumen global.

**Non-Goals:**
- No cambiar el cálculo de progreso global del proyecto.
- No rediseñar el contenido funcional de la tarjeta fuera del indicador.
- No modificar el formato de `tasks.md` ni reglas de parseo.

## Decisions

1. Reemplazo visual, sin tocar el modelo de datos.
- Se reutiliza el porcentaje ya calculado por tarjeta.
- Se elimina el uso del componente circular solo en tarjetas activas.

2. Ubicación del nuevo indicador.
- El porcentaje textual se renderiza a la derecha del bloque de botones de acción.
- Se usa tipografía mayor y peso alto para lectura inmediata.

3. Iconografía opcional y ligera.
- Se permite un icono discreto de progreso junto al texto solo si no aumenta ruido visual.
- Si compite con los iconos de artifacts, se prioriza texto puro.

4. Compatibilidad con estado sin tareas.
- Si no hay tareas (`totalTasks = 0`), se mantiene el comportamiento existente (`Sin tareas`) y el porcentaje textual se representa de forma coherente (por ejemplo `0%`).

## Risks / Trade-offs

- [Riesgo] Desalineación en tarjetas con títulos largos o múltiples acciones.
  Mitigación: ajustar layout con contenedor de acciones/progreso y reglas de wrap/align.

- [Riesgo] Inconsistencia visual con otros paneles que aún usan indicadores distintos.
  Mitigación: limitar explícitamente el cambio a tarjetas activas y documentarlo en specs.

- [Riesgo] Regresión en estilos por dependencias CSS compartidas.
  Mitigación: aplicar clases específicas del panel de activos y validar estados hover/focus.

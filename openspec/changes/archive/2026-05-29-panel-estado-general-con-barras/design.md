## Context

El dashboard ya ofrece contexto por panel (propuestas, cambios activos y archivados) y un resumen global en formato circular. Ese formato no permite mostrar bien varias métricas de proyecto al mismo tiempo (volumen de cambios, distribución y carga de tareas).

## Goals / Non-Goals

**Goals:**
- Reemplazar el rosco global por un bloque de estado general basado en barras.
- Incluir métricas agregadas clave del proyecto en una sola vista compacta.
- Mantener compatibilidad con la estructura actual del panel de descripción del proyecto.
- Preservar legibilidad en anchos reducidos de escritorio.

**Non-Goals:**
- No cambiar la lógica de progreso por tarjeta en cambios activos.
- No introducir dependencias externas de charting pesado.
- No alterar el flujo de edición/consulta de artifacts.

## Decisions

1. Visual principal en barras simples HTML/CSS.
- Se prioriza `div/progress` estilizado en vez de librería externa.
- Menor complejidad y consistencia con tema actual.

2. Métricas mínimas obligatorias del estado general.
- Total cambios.
- Cambios activos y archivados.
- Total tareas y tareas completadas.
- Porcentaje global derivado.

3. Ubicación de la visualización.
- El bloque de estado general sustituye el rosco en la zona derecha del panel de descripción.
- La zona izquierda mantiene datos de contexto del proyecto línea a línea.

4. Estados vacíos y fallback.
- Si no hay cambios o tareas, barras y contadores muestran 0 sin error.

## Risks / Trade-offs

- [Riesgo] Saturar visualmente el panel de descripción.
  Mitigación: limitar número de barras y agrupar métricas secundarias en texto compacto.

- [Riesgo] Pérdida de lectura instantánea frente al rosco.
  Mitigación: mostrar también porcentaje global destacado junto a las barras.

- [Riesgo] Desalineación en ventanas estrechas.
  Mitigación: permitir wrap vertical del bloque de barras en breakpoint de escritorio reducido.

## Context

Tras el rediseño de tarjetas activas se detectan tres desajustes: posición de botones, visual de tasks y color de headers markdown. Se requiere ajuste de micro-layout y tipografía sin tocar lógica de negocio.

## Goals / Non-Goals

**Goals:**
- Colocar botones de acciones debajo de la fila de iconos de artifacts.
- Mostrar tasks como icono simple (sin texto X/X junto al icono).
- Estilo de headers markdown en negro y negrita.

**Non-Goals:**
- Cambiar flujo modal preview/editar.
- Cambiar cálculo de porcentaje circular.
- Cambiar backend o estructura de archivos.

## Decisions

1. Reordenar bloques en tarjeta activa.
- Decisión: artifacts primero, luego fila de botones.
- Alternativa descartada: mantener botones en cabecera; no cumple requerimiento.

2. Tasks icon-only.
- Decisión: conservar información de progreso en círculo y quitar texto redundante junto al icono tasks.
- Alternativa descartada: mostrar X/X pequeño; sigue siendo ruido visual.

3. Headers markdown.
- Decisión: aplicar color negro (var texto principal) + font-weight bold en h1..h4.
- Alternativa descartada: azul/accent color; no cumple criterio solicitado.

## Risks / Trade-offs

- [Riesgo] Menor descubribilidad de acciones al moverlas abajo -> Mitigación: mantener tooltip/aria-label y separación visual clara.
- [Riesgo] Pérdida de contexto de tasks sin texto -> Mitigación: dejar círculo de porcentaje como fuente principal de progreso.
- [Riesgo] Contraste en dark theme con negro puro -> Mitigación: usar `var(--text-primary)` en vez de hex fijo.

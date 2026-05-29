## Context

El panel de descripción del proyecto existe encima de cambios activos, y el rosco de resumen global está separado en otro bloque del dashboard. Esto fragmenta información principal del proyecto. Además, los metadatos del proyecto aparecen en una sola línea con menor legibilidad.

## Goals / Non-Goals

**Goals:**
- Integrar el rosco de resumen global dentro del panel de descripción del proyecto, alineado a la derecha.
- Mostrar datos de proyecto línea a línea en columna legible.
- Mantener layout estable en app de escritorio sin solapes.

**Non-Goals:**
- Cambiar cálculo del porcentaje del rosco.
- Cambiar fuente de datos (`config.yaml.contexto`).
- Alterar lógica de carga de cambios activos/archivados.

## Decisions

1. Composición única de panel.
- Decisión: panel `Proyecto` tendrá dos zonas: izquierda (metadatos multilinea) y derecha (rosco).
- Alternativa descartada: mantener rosco fuera y duplicar indicadores; genera ruido visual.

2. Metadatos en bloque vertical.
- Decisión: cada campo del contexto en línea propia con etiqueta + valor.
- Alternativa descartada: fila única con separadores; pierde escaneabilidad.

3. Reutilizar componente del rosco.
- Decisión: mover/reubicar componente existente del resumen global al panel descripción.
- Alternativa descartada: recrear nuevo componente; riesgo de inconsistencia.

## Risks / Trade-offs

- [Riesgo] Panel más alto reduce espacio para cambios activos -> Mitigación: espaciado compacto y tipografía existente.
- [Riesgo] Rosco desalineado en anchos menores -> Mitigación: límites de ancho del panel y alineación vertical central.
- [Riesgo] Reordenación rompe tests visuales/manuales previos -> Mitigación: validación manual en temas claro/oscuro y tamaño mínimo de ventana.

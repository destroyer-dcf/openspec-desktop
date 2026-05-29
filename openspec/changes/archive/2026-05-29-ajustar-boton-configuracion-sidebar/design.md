## Context

En el sidebar actual existe una acción de configuración en el footer, pero su estilo es demasiado "botón" (borde y caja persistentes) y compite visualmente con acciones primarias. Además, la alineación del texto no está optimizada para lectura en layout de navegación.

## Goals / Non-Goals

**Goals:**
- Alinear el texto de configuración a la izquierda.
- Aplicar estilo sin borde persistente ni caja de botón fija.
- Mantener estado visual claro en hover/focus.
- Usar icono de rueda dentada como metáfora estándar de configuración.

**Non-Goals:**
- No modificar funcionalidad del modal de configuración.
- No alterar otras acciones del sidebar (añadir proyecto, desvincular, selección).
- No rediseñar globalmente todos los botones de la aplicación.

## Decisions

- Implementar el control de configuración con apariencia de acción secundaria del sidebar (flat), no botón primario.
- Dejar fondo/borde transparentes por defecto y aplicar realce solo en hover/focus-visible.
- Mantener área clicable suficiente para escritorio y accesibilidad por teclado.
- Reemplazar icono actual por `Settings`/rueda dentada en la librería de iconos existente.

## Risks / Trade-offs

- [Riesgo] Pérdida de descubribilidad al quitar borde permanente.
  Mitigación: mantener icono+texto y estados hover/focus notorios.

- [Riesgo] Inconsistencia con otros controles del footer.
  Mitigación: restringir cambio solo al botón de configuración, documentando patrón en spec.

- [Riesgo] Contraste insuficiente en temas.
  Mitigación: usar tokens de color del tema para hover/focus y comprobar en light/dark.

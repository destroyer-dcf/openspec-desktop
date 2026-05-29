## Context

El panel de propuestas actualmente mezcla filtros con acciones globales de lote (archivar/borrar) y selección por checkbox. El diseño deseado desplaza esas acciones a cada tarjeta para simplificar flujo. En paralelo, la configuración de color de botones añade complejidad y se quiere volver al estilo base de tema sin selector ni variante verde.

## Goals / Non-Goals

**Goals:**
- Mover archivar/borrar al contexto de cada tarjeta de propuesta.
- Eliminar selección múltiple y acciones globales asociadas.
- Mostrar extracto corto del contenido markdown bajo el título de propuesta.
- Eliminar selector de color de botones en modal de configuración.
- Normalizar color y hover de botones al tema por defecto.

**Non-Goals:**
- Cambiar modelo de persistencia de propuestas.
- Alterar el editor/modal de propuesta más allá de lo necesario para el preview.
- Rediseñar el resto de paneles fuera de propuestas y configuración de botones.

## Decisions

1. **Acciones por tarjeta con comandos existentes**
   - Reutilizar endpoints actuales de archivado/borrado pero invocados con una sola ruta por tarjeta.
   - Evita crear nuevos comandos backend y reduce riesgo.

2. **Preview markdown simplificado (2 líneas)**
   - Extraer texto plano del markdown y renderizarlo truncado con `line-clamp`/equivalente CSS.
   - Mantiene tarjetas compactas y mejora escaneabilidad.

3. **Retirada completa de `buttonColor` en UI prefs**
   - Quitar campo, selector y atributo `data-button-color`.
   - Unificar tokens de botón en tema claro/oscuro.

4. **Fallback visual estable**
   - Mantener jerarquía visual con iconos + tooltips en acciones por tarjeta.
   - Estados hover basados en variables de tema para consistencia global.

## Risks / Trade-offs

- **[Riesgo]** Menos eficiencia para acciones masivas → **Mitigación**: mantener UX rápida con botones icon-only y tooltips claros.
- **[Riesgo]** Preview de markdown puede incluir ruido (frontmatter/sintaxis) → **Mitigación**: limpiar cabecera metadata antes de truncar.
- **[Trade-off]** Quitar color de botones elimina personalización → **Mitigación**: consistencia de tema como prioridad de producto.

## Context

La aplicación ya permite configurar parte del estilo (tema, densidad, color base de botones) y renderiza varios paneles con tarjetas: cambios activos, cambios archivados y propuestas. Hoy el color semántico por estado/tipo no está centralizado ni configurable por panel, por lo que la lectura visual del pipeline es irregular.

## Goals / Non-Goals

**Goals:**
- Añadir configuración de color por panel/tipo desde el modal de configuración.
- Permitir estados diferenciados en tarjetas de cambios activos (completado vs pendiente).
- Permitir estados diferenciados en propuestas (feature vs bug) y en archivados.
- Mantener una opción explícita de “sin color” para conservar el comportamiento actual.
- Reutilizar el patrón visual de la tarjeta seleccionada del sidebar como estilo base de estado coloreado.

**Non-Goals:**
- Cambiar la estructura funcional de propuestas, cambios activos o archivados.
- Introducir un editor avanzado de paletas fuera de los presets definidos.
- Rediseñar tipografías, layout general o navegación.

## Decisions

1. **Nuevo bloque de preferencias de colores por panel/tipo**
   - Se añade una estructura de configuración persistida con claves por panel y tipo (activos, propuestas, archivados).
   - Rationale: evita lógica dispersa y habilita extensibilidad futura.

2. **Aplicación visual mediante clases semánticas + tokens CSS**
   - El render de cada tarjeta calcula una variante (`none`, `blue`, `green`, `red`, etc.) y aplica clases reutilizables.
   - Rationale: minimiza duplicación y garantiza consistencia entre paneles.

3. **Formato visual alineado con sidebar seleccionado**
   - Las variantes coloreadas replican el mismo patrón (fondo, borde, contraste) usado en proyecto seleccionado del sidebar.
   - Rationale: coherencia visual global y menor curva cognitiva.

4. **Fallback explícito a “sin color”**
   - Si no existe valor o el usuario elige none, se mantiene el estilo neutral actual.
   - Rationale: retrocompatibilidad y control fino por usuario.

Alternativas consideradas:
- Gestionar color sólo con inline styles: descartado por mantenimiento y consistencia.
- Un único color global por app: descartado por falta de granularidad por panel/tipo.

## Risks / Trade-offs

- **[Riesgo]** Mayor complejidad en el modal de configuración → **Mitigación**: agrupar por panel con labels claras y valores por defecto.
- **[Riesgo]** Colores con contraste insuficiente en tema claro/oscuro → **Mitigación**: limitar presets válidos y validar contraste visual.
- **[Trade-off]** Más estados configurables implican más combinaciones de prueba → **Mitigación**: matriz mínima de smoke tests por panel.

## Context

El modal de configuración ya gestiona tema, densidad y colores, por lo que es el punto natural para añadir la preferencia de idioma. Actualmente los textos están hardcodeados en español, sin capa central de traducción.

## Goals / Non-Goals

**Goals:**
- Soportar idiomas UI: `en`, `fr`, `de`, `pt`.
- Exponer selector de idioma en configuración.
- Aplicar traducción en caliente y persistir preferencia.
- Mantener fallback robusto cuando falten traducciones.

**Non-Goals:**
- No internacionalizar contenido libre de archivos markdown/propuestas.
- No integrar librerías externas pesadas si no son necesarias.
- No traducir mensajes de sistema/errores de backend en esta iteración.

## Decisions

- Añadir diccionarios locales por clave (`key-value`) para textos UI principales.
- Mantener español como fallback por defecto si falta clave/idioma.
- Extender preferencias UI en `localStorage` con `language`.
- Reemplazar textos hardcodeados gradualmente en componentes principales más visibles.

## Risks / Trade-offs

- [Riesgo] Cobertura parcial de traducciones en primera iteración.
  Mitigación: fallback a español por clave y lista inicial de pantallas críticas.

- [Riesgo] Incremento de mantenimiento por nuevas claves.
  Mitigación: centralizar diccionarios y tipar claves compartidas.

- [Riesgo] Inconsistencias si algún texto queda hardcodeado.
  Mitigación: revisión manual de UI y checklist de componentes afectados.

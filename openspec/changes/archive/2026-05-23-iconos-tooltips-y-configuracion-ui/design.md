## Context

La app ya dispone de sistema de tema (`light/dark`) y tokens CSS base. Falta extender configuración visual para densidad tipográfica y paleta de botones, y completar consistencia de iconos/tooltips en todos los botones. Se busca estilo compacto orientado a developer tooling.

## Goals / Non-Goals

**Goals:**
- Todo botón visible con icono coherente.
- Tooltip en todos los botones de acción.
- Escala tipográfica global más compacta configurable.
- Selector de color de botón (`blue`/`green`) en modal de configuración.
- Persistencia local de nuevas preferencias.

**Non-Goals:**
- No introducir librería pesada de tooltip/theme.
- No añadir más variantes cromáticas en esta iteración.
- No rediseñar navegación ni arquitectura backend.

## Decisions

### Tokens nuevos de tipografía y botones
Añadir tokens: `--font-size-base`, `--font-size-small`, `--button-accent`, `--button-accent-hover`, `--button-accent-text` y variantes por perfil.
- Alternativa descartada: tamaños hardcode por componente.

### Configuración visual centralizada
Extender modal de configuración existente con campos `fontScale` y `buttonColor` junto a `theme`.
- Alternativa descartada: modal separado para “apariencia”.

### Persistencia uniforme en localStorage
Guardar un objeto único de preferencias visuales (`theme`, `fontScale`, `buttonColor`) y restaurar en arranque.
- Alternativa descartada: claves separadas dispersas; más riesgo de drift.

### Tooltips nativos para primera iteración
Usar `title` y `aria-label` en botones como baseline accesible, con patrón consistente de texto.
- Alternativa descartada: sistema custom floating para tooltips (más complejidad).

### Auditoría de botones por componente
Recorrer sidebar, dashboard, change detail, editor, wizard y settings para asegurar icono+tooltip en todas las acciones.
- Alternativa descartada: tocar solo botones “principales”; dejaría deuda de consistencia.

## Risks / Trade-offs

- [Botones saturados visualmente] → Mitigación: iconos de 14-16px y espaciado compacto.
- [Tooltips duplicados/ambiguos] → Mitigación: catálogo de textos cortos por acción.
- [Contraste insuficiente en variante verde] → Mitigación: tokens verde con contraste AA mínimo sobre texto.
- [Regresiones de layout al reducir fuente] → Mitigación: validar en breakpoints y en vistas con alta densidad (dashboard/editor).

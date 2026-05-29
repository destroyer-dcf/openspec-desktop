## Context

La aplicación ya tiene sidebar, dashboard y modal de inicialización, pero no existe control centralizado de tema ni tokens semánticos de color. Los estilos están repartidos por componente y no hay persistencia explícita de modo visual. Debemos introducir selector dark/light en configuración y aplicar guía GitHub-like definida en `STYLEGUIDELINES.md`.

## Goals / Non-Goals

**Goals:**
- Añadir selector de tema `light/dark` en modal de configuración.
- Aplicar tema global sin recargar app.
- Persistir preferencia y restaurarla al iniciar.
- Unificar colores con tokens CSS semánticos alineados a guía GitHub.

**Non-Goals:**
- No implementar modo `system` en esta iteración.
- No rehacer toda la arquitectura de componentes.
- No añadir librería externa de theming.

## Decisions

### Estado de tema global en frontend
Guardar `theme` en estado raíz de `+page.svelte` y propagar por clase en `document.documentElement` (`data-theme="light|dark"`).
- Alternativa descartada: store complejo o contexto global dedicado; sobrecarga innecesaria para 2 modos.

### Persistencia local de tema
Persistir en `localStorage` (clave estable) y restaurar en `onMount`.
- Alternativa descartada: persistencia backend/Tauri; no aporta valor para preferencia puramente visual.

### Tokens CSS semánticos
Definir variables globales: `--bg-primary`, `--bg-secondary`, `--text-primary`, `--border-default`, `--accent-color` y derivados, con valores light/dark inspirados en GitHub.
- Alternativa descartada: hardcode por componente; genera deriva visual y mantenimiento caro.

### Modal de configuración dedicado
Crear/expandir modal de configuración con selector explícito dark/light y apply inmediato.
- Alternativa descartada: toggle suelto en sidebar; menos descubrible y peor escalabilidad de opciones.

### Compatibilidad incremental de componentes
Actualizar primero contenedores base (layout, sidebar, dashboard, inputs, botones, paneles) para usar tokens; luego componentes secundarios.
- Alternativa descartada: big-bang de todos los componentes en un solo commit.

## Risks / Trade-offs

- [Inconsistencia visual parcial] → Mitigación: checklist de componentes que deben migrar a tokens antes de cerrar la tarea.
- [Flicker inicial al cargar] → Mitigación: aplicar tema restaurado lo antes posible en `onMount` y fallback predecible.
- [Baja accesibilidad por contraste] → Mitigación: validar contraste textual y estados focus con paleta GitHub-like.
- [Colisión de estilos heredados] → Mitigación: reemplazar colores fijos por tokens de forma iterativa y verificar en dashboard/editor/modal.

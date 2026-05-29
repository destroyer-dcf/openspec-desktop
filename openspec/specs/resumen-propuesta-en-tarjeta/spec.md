# resumen-propuesta-en-tarjeta Specification

## Purpose
Display content preview and metadata summaries directly within proposal cards for quick scanning and discovery without opening full editor.

## Requirements

### Requirement: Preview de contenido en tarjeta de propuesta
El sistema SHALL mostrar debajo del título de cada propuesta un extracto breve del contenido markdown, en tipografía secundaria y limitado visualmente a dos líneas.

#### Scenario: Render de preview breve
- **WHEN** se lista una propuesta en el panel
- **THEN** la tarjeta muestra un extracto textual del contenido
- **AND** el extracto aparece truncado a dos líneas máximo

### Requirement: Limpieza básica del texto de preview
El sistema SHALL excluir metadatos de cabecera y markdown decorativo para priorizar texto legible en el extracto.

#### Scenario: Propuesta con frontmatter
- **WHEN** el contenido incluye frontmatter o sintaxis markdown
- **THEN** el preview muestra texto útil para lectura rápida

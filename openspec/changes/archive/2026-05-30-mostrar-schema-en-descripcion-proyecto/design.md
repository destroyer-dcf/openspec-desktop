## Context

La app ya muestra datos de `contexto` del `config.yaml` en el panel de descripción del proyecto, pero no expone explícitamente el campo `schema`. Ese dato es importante para saber si el proyecto está usando `spec-driven` u otro flujo.

## Goals / Non-Goals

**Goals:**
- Leer `schema` de `openspec/config.yaml` del proyecto activo.
- Exponer `schema` en el panel de descripción del proyecto.
- Definir fallback robusto cuando no exista valor.

**Non-Goals:**
- Editar el valor de `schema` desde la UI.
- Cambiar estructura completa del panel de descripción.
- Añadir nuevas dependencias de parseo fuera del backend actual.

## Decisions

- Reutilizar el parseo de configuración ya existente en backend Rust para extraer `schema`.
Rationale: evita duplicación y asegura una única fuente de verdad.

- Incluir `schema` en el modelo que alimenta el panel de descripción.
Rationale: desacopla render frontend del archivo YAML y simplifica la vista.

- Aplicar fallback legible (`No disponible`) si falta el campo o hay error de lectura.
Rationale: evita romper la UI y mantiene consistencia de UX.

Alternativas consideradas:
- Leer `config.yaml` directamente desde frontend: descartado por acoplar la vista al filesystem.
- Mostrar `schema` solo en modal de configuración: descartado porque el requerimiento es panel de descripción.

## Risks / Trade-offs

- [Config sin `schema` o YAML inválido] → Fallback seguro y no bloquear carga de proyecto.
- [Inconsistencia de traducción del label] → Usar sistema i18n existente para etiquetar `Schema` en todos los idiomas soportados.

## Migration Plan

1. Extender backend para incluir `schema` en datos de proyecto.
2. Propagar campo al contrato TypeScript.
3. Renderizar línea `Schema: <valor>` en panel de descripción.
4. Validar compilación y verificación visual en tema claro/oscuro.

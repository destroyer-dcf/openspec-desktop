## Context

El wizard de inicialización crea `openspec/config.yaml` con `schema` fijo en `spec-driven`. OpenSpec CLI ya expone información de plantillas/schema mediante `openspec templates`, pero actualmente no se integra en la UI.

## Goals / Non-Goals

**Goals:**
- Mostrar un combo de schemas en el modal de inicialización.
- Obtener opciones disponibles invocando `openspec templates` desde backend.
- Persistir el schema seleccionado en `config.yaml` al crear proyecto.
- Definir fallback robusto a `spec-driven` cuando falle la detección.

**Non-Goals:**
- Soportar edición de schema post-inicialización desde este cambio.
- Parsear paths de templates completos para más metadata.
- Introducir dependencias nuevas externas para parseo CLI.

## Decisions

- Crear un comando backend dedicado para listar schemas disponibles.
Rationale: centraliza interacción con CLI y evita lógica de parseo en frontend.

- Parsear las líneas `Schema: <name>` del output de `openspec templates`.
Rationale: formato más estable y directo para poblar el combo.

- Mantener valor por defecto `spec-driven` si el comando falla o no retorna datos.
Rationale: continuidad del flujo actual sin bloquear inicialización.

- En `init_project`, usar `input.schema` (opcional) normalizado con fallback a `spec-driven`.
Rationale: compatibilidad con llamadas previas y robustez de entrada.

## Risks / Trade-offs

- [Formato de `openspec templates` cambia] → Aislar parseo en backend y aplicar fallback seguro.
- [CLI no disponible] → Mostrar mensaje existente y mantener flujo de error controlado del wizard.

## Migration Plan

1. Extender contrato de entrada del wizard con campo `schema`.
2. Añadir comando backend para listar schemas.
3. Conectar combo frontend con carga de schemas y default.
4. Escribir `schema` seleccionado en `config.yaml` durante init.
5. Validar compilación y flujo manual de inicialización.

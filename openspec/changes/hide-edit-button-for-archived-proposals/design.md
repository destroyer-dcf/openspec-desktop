## Context

El panel de propuestas presenta acciones contextuales por tarjeta. Actualmente la disponibilidad de acciones no diferencia de forma explícita entre propuestas activas y archivadas para el caso de edición, lo que permite mostrar una acción no válida funcionalmente en elementos archivados.

## Goals / Non-Goals

**Goals:**
- Definir una regla clara de render: el botón editar solo aparece en propuestas no archivadas.
- Mantener las demás acciones existentes sin cambios de comportamiento no solicitados.
- Reducir ambigüedad visual y evitar intentos de edición en propuestas archivadas.

**Non-Goals:**
- No cambiar la lógica de archivado/borrado.
- No modificar el flujo del modal de edición para propuestas activas.
- No introducir nuevos permisos o roles.

## Decisions

1. Condicionar render del botón editar por estado de propuesta.
- Si `status` (o equivalente de archivado) indica archivada, no renderizar botón editar.
- Para propuestas activas, conservar render actual.
- Rationale: evita affordance inválida y minimiza cambios.

2. Aplicar el cambio en el componente de listado/tarjeta de propuestas.
- Mantener decisión en capa de presentación para aislar impacto.
- Rationale: el requerimiento es de UX/visibilidad, no de modelo de datos.

3. Verificación dirigida en tests/component checks.
- Validar presencia/ausencia condicional del botón según estado.
- Rationale: previene regresiones cuando cambie markup de tarjetas.

## Risks / Trade-offs

- [Campo de estado inconsistente entre fuentes] → Mitigación: reutilizar el mismo flag ya usado por acciones de archivado/listado.
- [Regresión visual en layout de acciones] → Mitigación: revisar spacing de contenedor al ocultar botón.
- [Cobertura insuficiente] → Mitigación: añadir prueba explícita para caso archivado y no archivado.

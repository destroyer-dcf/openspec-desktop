# resumen-why-en-tarjetas-cambio Specification

## Purpose
Mostrar resumen del bloque `Why` de `proposal.md` debajo del título de tarjetas de cambio, en activos y archivados, con fallback cuando no esté disponible.

## Requirements

### Requirement: Resumen Why en tarjetas de cambio
La app SHALL mostrar un resumen del bloque `Why` de `proposal.md` debajo del título de cada tarjeta de cambio, tanto en activos como archivados.

#### Scenario: Tarjeta activa con proposal válido
- **WHEN** el cambio tiene `proposal.md` con sección `## Why`
- **THEN** la tarjeta activa muestra el texto `Why` bajo el título

#### Scenario: Tarjeta archivada con proposal válido
- **WHEN** el cambio archivado tiene `proposal.md` con sección `## Why`
- **THEN** la tarjeta archivada muestra el texto `Why` bajo el título

### Requirement: Fallback sin errores
La app SHALL mostrar un texto fallback cuando no sea posible obtener `Why`, sin presentar error técnico.

#### Scenario: proposal ausente o sin Why
- **WHEN** falta `proposal.md` o no contiene bloque `Why`
- **THEN** la tarjeta muestra "Sin resumen" (o equivalente) y continúa renderizando sin error

# selector-schema-init-proyecto Specification

## Purpose
Provide dynamic schema selection during project initialization, fetching available templates from OpenSpec with graceful fallback to default spec-driven schema when CLI is unavailable.

## Requirements

### Requirement: Listado dinámico de schemas para inicialización
La aplicación SHALL obtener y mostrar los schemas disponibles de OpenSpec para inicialización ejecutando `openspec templates` y extrayendo los valores de `Schema:` del resultado.

#### Scenario: CLI retorna schemas válidos
- **WHEN** el usuario abre el modal de inicialización y la CLI responde correctamente
- **THEN** el combo de schema muestra las opciones detectadas y selecciona una por defecto

#### Scenario: Error al consultar templates
- **WHEN** falla la ejecución de `openspec templates` o no se detectan schemas
- **THEN** el combo usa fallback `spec-driven` y el flujo de inicialización sigue disponible

### Requirement: Aplicación del schema seleccionado en config
La aplicación MUST escribir en `openspec/config.yaml` el `schema` seleccionado por el usuario al confirmar la inicialización.

#### Scenario: Confirmación con schema seleccionado
- **WHEN** el usuario confirma el wizard con un schema elegido
- **THEN** el `config.yaml` generado contiene ese valor en la clave `schema`

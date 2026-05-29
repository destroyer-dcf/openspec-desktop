## ADDED Requirements

### Requirement: Contexto funcional visible en cambios archivados
La sección de cambios archivados SHALL mostrar el mismo resumen `Why` debajo del título de cada tarjeta.

#### Scenario: Archivado con resumen disponible
- **WHEN** el cambio archivado tiene resumen `Why`
- **THEN** la tarjeta archivada lo muestra con estilo secundario

#### Scenario: Archivado sin resumen disponible
- **WHEN** no existe resumen `Why` para el cambio archivado
- **THEN** la tarjeta muestra fallback y mantiene fecha + acción de consulta

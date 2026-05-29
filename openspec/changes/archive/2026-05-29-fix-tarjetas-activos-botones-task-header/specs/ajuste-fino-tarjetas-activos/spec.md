## ADDED Requirements

### Requirement: Orden interno de elementos en tarjeta activa
La app SHALL renderizar en tarjeta de cambio activo primero los iconos de artifacts y debajo la fila de botones de acción.

#### Scenario: Tarjeta activa renderizada
- **WHEN** se muestra una tarjeta de cambio activo
- **THEN** los botones de acción aparecen debajo de la fila de iconos de artifacts

### Requirement: Tasks en modo icon-only
La app SHALL mostrar tasks en la fila de artifacts como icono sin texto adicional X/X.

#### Scenario: Fila de artifacts
- **WHEN** se renderiza la fila de artifacts de un cambio activo
- **THEN** tasks aparece únicamente como icono, manteniendo consistencia visual con el resto

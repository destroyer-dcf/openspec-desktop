# modal-consulta-archivados-scroll Specification

## Purpose
Fixed modal size with internal scroll for archived change consultation modal to maintain stable UI layout.

## Requirements

### Requirement: Tamaño estándar del modal de consulta archivada
La aplicación SHALL mostrar el modal de consulta de cambios archivados con dimensiones estándar y estables, sin crecer en función del tamaño del markdown seleccionado.

#### Scenario: Apertura con documento grande
- **WHEN** la persona usuaria abre un documento markdown extenso en la consulta archivada
- **THEN** el modal conserva su tamaño estándar y no expande su contenedor principal

### Requirement: Scroll interno para contenido markdown archivado
La aplicación SHALL habilitar scroll vertical dentro del panel de contenido del documento cuando el markdown supere el área disponible del modal.

#### Scenario: Contenido supera altura visible
- **WHEN** el contenido renderizado del markdown excede la altura del panel de documento
- **THEN** la app permite desplazamiento vertical dentro del panel, manteniendo visibles la estructura del modal y sus acciones

## MODIFIED Requirements

### Requirement: Footer y responsive del sidebar
La app SHALL ubicar la acción de configuración en el footer del sidebar, con texto alineado a la izquierda y estilo visual ligero (sin borde persistente), manteniendo layout de escritorio usable en anchos reducidos.

#### Scenario: Sidebar en viewport reducido
- **WHEN** el ancho disponible es pequeño
- **THEN** las tarjetas, texto y acciones del sidebar mantienen legibilidad y no se solapan

#### Scenario: Alineación del control de configuración
- **WHEN** la persona usuaria visualiza el footer del sidebar
- **THEN** el texto del control de configuración aparece alineado a la izquierda dentro del área clicable

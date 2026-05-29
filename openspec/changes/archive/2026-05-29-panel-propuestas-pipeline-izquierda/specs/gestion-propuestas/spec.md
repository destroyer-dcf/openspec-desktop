## MODIFIED Requirements

### Requirement: Panel de propuestas en grid
La aplicación SHALL mostrar un panel “Propuestas” como etapa inicial del pipeline en `content`, ubicado a la izquierda de “Cambios activos”, manteniendo visualización en grid de propuestas activas cargadas desde `openspec/propose/actives`.

#### Scenario: Render de propuestas activas
- **WHEN** existen archivos de propuesta en `openspec/propose/actives`
- **THEN** la app muestra una tarjeta por propuesta con fecha de creación, nombre y botón “Modificar” dentro del panel izquierdo del pipeline

#### Scenario: Sin propuestas activas
- **WHEN** no existen propuestas en `openspec/propose/actives`
- **THEN** la app muestra estado vacío del panel de propuestas y mantiene disponible el botón para añadir propuesta

## Why

El panel de propuestas no está integrado visualmente como etapa del flujo principal del proyecto junto a cambios activos y archivados. Se busca una lectura tipo pipeline, con propuestas como fase previa visible en la misma jerarquía del estado.

## What Changes

- Crear un panel de Propuestas con estructura visual equivalente a “Cambios activos” y “Cambios archivados”.
- Ubicar ese panel a la izquierda de “Cambios activos” para reflejar orden de pipeline (propuestas -> cambios activos -> cambios archivados).
- Mantener información y acciones actuales de propuestas dentro del nuevo panel.
- Ajustar layout para que la composición de tres paneles sea estable y legible en escritorio.

## Capabilities

### New Capabilities
- `pipeline-visual-estado-proyecto`: representación en pipeline visual con panel de propuestas como etapa inicial a la izquierda.

### Modified Capabilities
- `dashboard-estado`: reorganización estructural de paneles para incorporar propuestas en el flujo principal del estado.
- `gestion-propuestas`: adaptación de presentación del panel propuestas al nuevo contenedor de pipeline sin perder acciones actuales.

## Impact

- Componentes frontend del dashboard y del panel de propuestas.
- Estilos de layout/grid/flex para orden visual de etapas.
- Sin cambios en backend ni estructura de persistencia de propuestas.

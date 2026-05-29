## Why

Actualmente falta una descripción visible del proyecto en el área principal, lo que dificulta el contexto rápido al abrir un workspace con varios cambios activos. Mostrar el contexto del `config.yaml` mejora orientación y reduce errores de interpretación.

## What Changes

- Añadir un panel horizontal de descripción del proyecto en `content`, situado encima de la sección de cambios activos.
- Poblar ese panel con los campos del bloque `contexto` de `config.yaml` del proyecto activo.
- Definir comportamiento de fallback cuando no exista `config.yaml`, no tenga `contexto`, o falten campos.
- Mantener el layout sin solapes con sidebar y con comportamiento responsive coherente.

## Capabilities

### New Capabilities
- `panel-descripcion-proyecto`: Panel horizontal informativo del proyecto basado en `config.yaml`.

### Modified Capabilities
- `dashboard-estado`: Se actualiza la composición del panel principal para incluir primero la descripción del proyecto y después cambios activos.

## Impact

- Frontend/dashboard: componentes y layout del área de contenido.
- Capa de lectura/parsing de configuración del proyecto activo (`config.yaml`).
- Estilos responsive para evitar solape o reflujo incorrecto.

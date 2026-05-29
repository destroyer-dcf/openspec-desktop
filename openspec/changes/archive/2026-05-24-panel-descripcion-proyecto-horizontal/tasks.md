## 1. Modelo de datos del contexto

- [x] 1.1 Identificar el flujo actual de carga de `config.yaml` del proyecto activo y extender el modelo de estado para incluir `contexto`.
- [x] 1.2 Implementar parsing defensivo de `contexto` con fallback por campo a "No definido" cuando falten claves o no exista archivo.

## 2. UI del panel horizontal

- [x] 2.1 Crear componente/panel de descripción de proyecto que renderice pares etiqueta/valor del `contexto`.
- [x] 2.2 Insertar el panel encima de la sección "Cambios activos" dentro de `content`, respetando el orden requerido por spec.
- [x] 2.3 Añadir estilos responsive (fila con wrap) para evitar solape en anchos reducidos.

## 3. Integración y validación

- [x] 3.1 Verificar render correcto con proyecto con `contexto` completo.
- [x] 3.2 Verificar comportamiento fallback con `config.yaml` ausente o `contexto` incompleto.
- [x] 3.3 Verificar que no hay regresión de layout con sidebar ni en lista de cambios activos.

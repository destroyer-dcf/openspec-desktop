## Why

Hay dos fallos de UX/estabilidad: al intentar abrir rutas que son directorio se lanza error de lectura de fichero, y el panel principal se solapa con la sidebar en ciertos estados. Ambos rompen flujo básico de navegación.

## What Changes

- Corregir lectura de artifacts para detectar directorios y mostrar mensaje funcional en vez de error OS (`os error 21`).
- Evitar que carpetas vacías bajo `specs/` disparen intento de lectura como documento.
- Corregir layout para impedir solape entre sidebar y panel de contenido en dashboard y detalle de archivos.
- Añadir validaciones visuales y de navegación para escenarios con carpeta vacía y listas largas.

## Capabilities

### New Capabilities
- `manejo-rutas-directorio`: manejo seguro de selección de artifacts cuando la ruta apunta a directorio o recurso no-leíble como documento.

### Modified Capabilities
- `editor-markdown`: ajustar comportamiento al abrir rutas inválidas/no-fichero.
- `dashboard-estado`: ajustar grid/layout para evitar solape con sidebar.

## Impact

- Frontend Svelte: lógica de selección de artifact, mensajes de error, estilos de layout principal/dashboard/detail.
- Validación UX: navegación en specs vacías, listado de artifacts y cambios activos sin colisión visual.
- Sin cambios de dominio OpenSpec ni formato de datos.

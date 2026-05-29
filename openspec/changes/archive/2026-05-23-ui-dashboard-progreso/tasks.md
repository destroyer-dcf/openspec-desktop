## 1. Instalación de lucide-svelte

- [x] 1.1 Añadir `lucide-svelte` como dependencia en `package.json` (`npm install lucide-svelte`)
- [x] 1.2 Validar: importar un icono de `lucide-svelte` en un componente y verificar que compila sin errores

## 2. Iconos en la barra lateral

- [x] 2.1 Añadir icono de proyecto (ej. `FolderOpen`) junto al nombre de cada proyecto en `Sidebar.svelte`
- [x] 2.2 Añadir icono en el botón "+" de añadir proyecto (`Plus` de lucide-svelte)
- [x] 2.3 Validar: los iconos aparecen en la barra lateral sin afectar el layout existente

## 3. Iconos de artifacts y acciones

- [x] 3.1 Añadir icono de estado de artifact en `Dashboard.svelte`: `CheckCircle` (completado), `Circle` (pendiente), `XCircle` (bloqueado)
- [x] 3.2 Añadir icono de acción en botones de abrir artifact (`FileText`) y editar (`Pencil`)
- [x] 3.3 Validar: cada artifact muestra su icono de estado correspondiente según el estado real

## 4. Iconos de secciones

- [x] 4.1 Añadir icono de sección en la cabecera "Cambios activos" (`GitBranch`) y "Cambios archivados" (`Archive`)
- [x] 4.2 Añadir icono en la cabecera del panel de resumen global (`BarChart2`)
- [x] 4.3 Validar: los iconos de sección aparecen correctamente en ambas columnas del dashboard

## 5. Componente ProgressBar

- [x] 5.1 Crear componente `ProgressBar.svelte` con props: `completed: number`, `total: number`
- [x] 5.2 Usar elemento nativo `<progress value={completed} max={total}>` con estilos CSS (`appearance: none`, color de marca)
- [x] 5.3 Mostrar texto "X/Y tareas" junto a la barra
- [x] 5.4 Mostrar "Sin tareas" si `total === 0`
- [x] 5.5 Validar: la barra refleja correctamente la proporción completado/total

## 6. Componente CircularProgress

- [x] 6.1 Crear componente `CircularProgress.svelte` con prop `percent: number` (0-100)
- [x] 6.2 Implementar círculo SVG con `stroke-dasharray` / `stroke-dashoffset` para representar el porcentaje
- [x] 6.3 Mostrar el porcentaje en texto centrado dentro del círculo
- [x] 6.4 Validar: el componente renderiza correctamente con 0%, 50% y 100%

## 7. Layout de dos columnas en el dashboard

- [x] 7.1 Reorganizar `Dashboard.svelte` con CSS Grid de dos columnas: lista de cambios (izquierda) y panel resumen + `CircularProgress` (derecha)
- [x] 7.2 Añadir `ProgressBar.svelte` debajo de cada cambio activo en la columna izquierda
- [x] 7.3 Añadir `CircularProgress.svelte` en la columna derecha con el progreso global (% de todas las tareas activas)
- [x] 7.4 Hacer el layout responsive: una sola columna cuando el ancho sea inferior a 640px
- [x] 7.5 Validar: el layout se ve correctamente con 1, 3 y 10 cambios activos

## 8. Actualización en tiempo real del progreso

- [x] 8.1 Asegurar que el evento `project-updated` de Tauri recalcula el progreso en el frontend sin recargar la app
- [x] 8.2 Validar: marcar una tarea como completada en `tasks.md` desde terminal actualiza la barra de progreso en menos de 2 segundos

## 9. Validación general de UI

- [x] 9.1 Revisar que todos los iconos se importan con tree-shaking (solo los usados)
- [x] 9.2 Verificar que el bundle final no supera 15KB adicionales por los iconos
- [x] 9.3 Validar accesibilidad: los elementos `<progress>` tienen atributos `aria-label` descriptivos

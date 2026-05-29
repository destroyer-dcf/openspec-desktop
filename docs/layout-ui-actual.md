# Distribución Actual de la UI

Este documento resume la distribución actual del frontend de la aplicación y dónde vive cada pieza en el código.

## Layout global

Archivo:
- `/Users/destroyer/MIS_PROYECTOS/openspec-manager/src/routes/+page.svelte`

Estructura:
- Grid de **2 columnas**: sidebar izquierda + contenido derecha.
- Desktop: `minmax(220px, 280px) 1fr`
- Móvil (`<640px`): se mantiene en 2 columnas (`220px 1fr`) para no romper el flujo.

## Mapa ASCII

```text
┌──────────────────────────────────────────────────────────────────────┐
│                           APP ( +page.svelte )                      │
├───────────────────────────────┬──────────────────────────────────────┤
│ SIDEBAR                       │ CONTENT                              │
│ (Sidebar.svelte)              │                                      │
│                               │  ┌────────────────────────────────┐  │
│  • Lista de proyectos         │  │ Vista activa según estado      │  │
│  • Proyecto activo resaltado  │  │                                │  │
│  • Botón + añadir proyecto    │  │ 1) Dashboard.svelte            │  │
│  • Iconos FolderOpen / Plus   │  │ 2) ChangeDetail.svelte         │  │
│                               │  │ 3) EditorMarkdown.svelte       │  │
│                               │  └────────────────────────────────┘  │
├───────────────────────────────┴──────────────────────────────────────┤
│ Modal opcional superpuesto: WizardInit.svelte                        │
│ (se abre cuando carpeta seleccionada no contiene openspec/)          │
└──────────────────────────────────────────────────────────────────────┘
```

## Componentes principales

### 1) Sidebar

Archivo:
- `/Users/destroyer/MIS_PROYECTOS/openspec-manager/src/lib/components/Sidebar.svelte`

Función:
- Lista de proyectos.
- Botón `+` para seleccionar carpeta.
- Iconos de proyecto y acción (`FolderOpen`, `Plus`).

### 2) Panel principal

Renderiza una de estas vistas:

- Dashboard:
  - `/Users/destroyer/MIS_PROYECTOS/openspec-manager/src/lib/components/Dashboard.svelte`
- Detalle de cambio:
  - `/Users/destroyer/MIS_PROYECTOS/openspec-manager/src/lib/components/ChangeDetail.svelte`
- Editor Markdown:
  - `/Users/destroyer/MIS_PROYECTOS/openspec-manager/src/lib/components/EditorMarkdown.svelte`

### 3) Dashboard interno

Archivo:
- `/Users/destroyer/MIS_PROYECTOS/openspec-manager/src/lib/components/Dashboard.svelte`

Distribución interna:
- **2 columnas**
  - Izquierda: cambios activos + artifacts + `ProgressBar`
  - Derecha: resumen global + `CircularProgress` + cambios archivados
- Responsive:
  - Bajo `640px`, el dashboard interno pasa a una columna.

Componentes usados:
- `/Users/destroyer/MIS_PROYECTOS/openspec-manager/src/lib/components/ProgressBar.svelte`
- `/Users/destroyer/MIS_PROYECTOS/openspec-manager/src/lib/components/CircularProgress.svelte`

### 4) Wizard de inicialización

Archivo:
- `/Users/destroyer/MIS_PROYECTOS/openspec-manager/src/lib/components/WizardInit.svelte`

Función:
- Modal para inicializar proyecto cuando carpeta no tiene `openspec/`.
- Recoge datos del proyecto y proveedor IA.

## Resumen corto

La app está organizada como:
- **Shell principal fijo** (sidebar + contenido).
- **Contenido dinámico** (dashboard/detalle/editor).
- **Modal de wizard** cuando aplica.

Esto permite cambiar de proyecto rápido sin recargar app y mantener navegación consistente.

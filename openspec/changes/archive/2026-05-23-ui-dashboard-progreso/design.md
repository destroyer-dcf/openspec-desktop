## Context

El frontend existe y funciona. Este cambio es puramente de presentación: añadir iconos, indicadores de progreso visual y reorganizar el layout del dashboard. No se modifica el backend Rust ni los Tauri commands. El modelo de datos (`ProjectState`) ya contiene la información de progreso necesaria (calculada en el cambio anterior, `visor-proyecto`).

## Goals / Non-Goals

**Goals:**
- Sistema de iconos consistente con `lucide-svelte` en toda la interfaz.
- Indicador circular de progreso global del proyecto activo.
- Barras de progreso lineales por cambio.
- Layout de dos columnas en el dashboard.
- Actualización en tiempo real de los indicadores cuando los ficheros cambian.

**Non-Goals:**
- Temas de color / modo oscuro (fuera de alcance de este cambio).
- Animaciones complejas de carga.
- Cambios en el backend Rust o en los Tauri commands.

## Decisions

### lucide-svelte para iconos
`lucide-svelte` es el binding oficial de Lucide para Svelte. Es tree-shakeable (solo se empaquetan los iconos importados), tiene licencia MIT y un diseño consistente que se adapta bien a herramientas de desarrollo.
- **Alternativa descartada**: SVG inline manual — difícil de mantener y sin consistencia visual.
- **Alternativa descartada**: heroicons — no tiene binding oficial para Svelte 5.

### Progreso circular: SVG custom
El indicador circular se implementa como un componente `CircularProgress.svelte` usando SVG con `stroke-dasharray` y `stroke-dashoffset`. No requiere librerías adicionales y el resultado es ligero (~50 líneas de SVG + CSS).
- **Alternativa descartada**: `svelte-circular-progressbar` u otras librerías — overhead innecesario para un componente tan simple.

### Progreso lineal por cambio: `<progress>` nativo + CSS
El elemento HTML nativo `<progress>` con estilos CSS personalizados (`appearance: none`, colores de marca) ofrece accesibilidad nativa (atributo `value`/`max`) sin necesidad de librerías.

### Cálculo de progreso en el frontend
El porcentaje se calcula en el frontend a partir del `ProjectState` recibido desde Rust (que ya incluye `completed_tasks` y `total_tasks` por cambio). No hay nueva lógica en el backend.

### Layout de dos columnas
CSS Grid con dos columnas: izquierda para la lista de cambios activos, derecha para el panel de resumen global con el indicador circular. En pantallas pequeñas, las columnas colapsan a una sola.

## Risks / Trade-offs

- **Bundle size**: `lucide-svelte` con tree-shaking agresivo no debería superar 10-15KB para los iconos usados. Mitigación: importar solo los iconos necesarios, no el módulo completo.
- **SVG circular en HiDPI**: Los valores de `stroke-dasharray` se calculan en función del radio del círculo; se debe usar proporciones en unidades relativas para evitar blur en pantallas Retina.

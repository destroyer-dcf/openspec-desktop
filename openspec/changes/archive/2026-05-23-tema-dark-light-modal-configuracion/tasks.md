## 1. Base de tema global

- [x] 1.1 Definir tokens CSS globales (`--bg-primary`, `--bg-secondary`, `--text-primary`, `--border-default`, `--accent-color`) para light/dark según `STYLEGUIDELINES.md`
- [x] 1.2 Añadir selector de tema global vía atributo (`data-theme`) en la raíz de la app
- [x] 1.3 Implementar estado frontend `theme` con valores permitidos `light|dark`
- [x] 1.4 Restaurar tema desde persistencia al arrancar la app
- [x] 1.5 Persistir tema al cambiar selección

## 2. Modal de configuración

- [x] 2.1 Crear o extender modal de configuración para incluir sección "Tema"
- [x] 2.2 Añadir control de selección con opciones `light` y `dark`
- [x] 2.3 Aplicar cambio de tema en caliente sin recargar app
- [x] 2.4 Validar estado inicial del selector con tema persistido
- [x] 2.5 Validar que cerrar/reabrir modal mantiene sincronía con tema activo

## 3. Migración visual a tokens GitHub-like

- [x] 3.1 Actualizar layout principal (sidebar/content) para usar tokens en fondo, borde y texto
- [x] 3.2 Actualizar sidebar (ítems, activo, hover, botón añadir) para usar tokens
- [x] 3.3 Actualizar dashboard (paneles, cabeceras, listas, estados) para usar tokens
- [x] 3.4 Actualizar formularios y controles (inputs, selects, botones, focus ring) para usar tokens
- [x] 3.5 Actualizar modal wizard para usar tokens sin hardcodes de color

## 4. Validación funcional de tema

- [x] 4.1 Validar tema light en flujo completo (sidebar, dashboard, modales, editor)
- [x] 4.2 Validar tema dark en flujo completo (sidebar, dashboard, modales, editor)
- [x] 4.3 Validar persistencia: seleccionar dark, reiniciar app, confirmar dark restaurado
- [x] 4.4 Validar persistencia: seleccionar light, reiniciar app, confirmar light restaurado

## 5. Validación de calidad visual

- [x] 5.1 Verificar contraste y legibilidad de texto primario/secundario en ambos temas
- [x] 5.2 Verificar bordes/separadores sutiles estilo GitHub en ambos temas
- [x] 5.3 Verificar estados focus visibles y consistentes en teclado (inputs/botones/select)
- [x] 5.4 Revisar que no queden colores hardcodeados en componentes migrados

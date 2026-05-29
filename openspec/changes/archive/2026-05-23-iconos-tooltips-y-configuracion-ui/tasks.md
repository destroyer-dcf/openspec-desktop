## 1. Extensión de preferencias visuales

- [x] 1.1 Extender estado visual global para incluir `fontScale` (`compact|normal`) y `buttonColor` (`blue|green`)
- [x] 1.2 Persistir/restaurar `theme`, `fontScale` y `buttonColor` en una estructura unificada de localStorage
- [x] 1.3 Definir tokens CSS para escala tipográfica compact/normal
- [x] 1.4 Definir tokens CSS para variantes cromáticas de botón blue/green

## 2. Modal de configuración

- [x] 2.1 Añadir selector de tamaño de texto (`compact`, `normal`) en modal de configuración
- [x] 2.2 Añadir selector de color de botones (`blue`, `green`) en modal de configuración
- [x] 2.3 Aplicar cambios en caliente al confirmar selección sin recargar app
- [x] 2.4 Validar sincronía del modal con valores persistidos al abrir/cerrar

## 3. Iconos y tooltips en botones

- [x] 3.1 Auditar botones de Sidebar y añadir icono+tooltip donde falte
- [x] 3.2 Auditar botones de Dashboard y añadir icono+tooltip donde falte
- [x] 3.3 Auditar botones de ChangeDetail y añadir icono+tooltip donde falte
- [x] 3.4 Auditar botones de EditorMarkdown y añadir icono+tooltip donde falte
- [x] 3.5 Auditar botones de WizardInit/Settings y añadir icono+tooltip donde falte

## 4. Ajustes de densidad tipográfica

- [x] 4.1 Aplicar tokens de tamaño de texto en layout base y tipografía de componentes principales
- [x] 4.2 Ajustar paddings/espaciado de controles para mantener consistencia compacta
- [x] 4.3 Verificar que textos siguen legibles en `compact` y `normal`

## 5. Validación funcional y visual

- [x] 5.1 Validar que todos los botones visibles tienen icono y tooltip funcional
- [x] 5.2 Validar cambio `blue/green` en botones principales en todas las vistas
- [x] 5.3 Validar persistencia tras reinicio para `theme`, `fontScale`, `buttonColor`
- [x] 5.4 Validar accesibilidad básica (`aria-label`, foco visible, contraste)

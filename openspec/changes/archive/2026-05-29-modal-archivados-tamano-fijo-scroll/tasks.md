## 1. Estructura del modal y tamaño estándar

- [x] 1.1 Definir dimensiones estándar del modal de consulta archivada con límites máximos relativos al viewport.
- [x] 1.2 Evitar que el contenedor principal del modal cambie de tamaño al seleccionar documentos markdown extensos.
- [x] 1.3 Mantener cabecera y acciones del modal siempre visibles fuera del área scrollable de contenido.

## 2. Scroll interno del documento

- [x] 2.1 Encapsular el panel de render markdown en un contenedor con altura controlada.
- [x] 2.2 Habilitar overflow vertical en el panel de documento para contenido largo.
- [x] 2.3 Verificar que la navegación del listado de documentos y la lectura del markdown no generen doble scroll global.

## 3. Validación visual y regresión

- [x] 3.1 Validar en tema claro y oscuro que el modal mantiene tamaño estable.
- [x] 3.2 Validar que documentos cortos y largos se visualizan correctamente con/ sin scroll interno.
- [x] 3.3 Validar que no se afectan acciones existentes del modal (abrir, cambiar documento, cerrar).

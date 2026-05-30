## 1. Persistencia de tamaño en estado de aplicación

- [x] 1.1 Extender el modelo de estado persistido con campos opcionales `window_width` y `window_height`.
- [x] 1.2 Implementar guardado del tamaño de ventana principal en el cierre normal de la app.
- [x] 1.3 Asegurar que la escritura del estado no rompe compatibilidad con estados previos sin estos campos.

## 2. Restauración de tamaño al arranque

- [x] 2.1 Leer dimensiones persistidas durante bootstrap y validar contra mínimos de ventana.
- [x] 2.2 Aplicar restauración del tamaño en la ventana principal cuando los datos sean válidos.
- [x] 2.3 Definir fallback a dimensiones por defecto cuando no haya datos o sean inválidos.

## 3. Validación funcional y técnica

- [x] 3.1 Verificar compilación sin errores (`cargo check` y `npm run check`).
- [x] 3.2 Validar manualmente: redimensionar, cerrar, abrir, comprobar que se conserva tamaño.
- [x] 3.3 Documentar cualquier limitación observada por plataforma durante la validación.

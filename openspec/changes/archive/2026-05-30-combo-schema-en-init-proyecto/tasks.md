## 1. Backend: schemas disponibles y uso en init

- [x] 1.1 Añadir comando backend para listar schemas disponibles parseando `openspec templates` (líneas `Schema:`).
- [x] 1.2 Definir fallback a `spec-driven` cuando el comando falle o no devuelva opciones.
- [x] 1.3 Extender `InitProjectInput` para aceptar `schema` opcional y usarlo al escribir `config.yaml`.

## 2. Frontend: combo de schema en wizard

- [x] 2.1 Añadir estado/campo `schema` en `WizardInit` con valor por defecto `spec-driven`.
- [x] 2.2 Cargar opciones desde el nuevo comando backend y poblar un combo en el modal de inicialización.
- [x] 2.3 Enviar el schema seleccionado al comando `init_project`.

## 3. Validación

- [x] 3.1 Verificar compilación y checks sin errores (`cargo check` y `npm run check`).
- [x] 3.2 Validar manualmente que el combo muestra schemas y permite inicializar con el valor seleccionado.
- [x] 3.3 Validar fallback: sin datos de templates o con error, se usa `spec-driven` sin bloquear el wizard.

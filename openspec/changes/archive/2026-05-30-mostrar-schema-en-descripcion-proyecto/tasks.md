## 1. Backend: exponer schema desde config

- [x] 1.1 Localizar el parseo de `openspec/config.yaml` y extraer el campo `schema` en el modelo de estado del proyecto.
- [x] 1.2 Añadir fallback `No disponible` cuando falte `schema` o el archivo no sea legible.
- [x] 1.3 Asegurar que el nuevo campo no rompe compatibilidad con proyectos ya cargados.

## 2. Frontend: mostrar schema en panel descripción

- [x] 2.1 Extender tipos TypeScript para incluir el campo `schema` en los datos de descripción del proyecto.
- [x] 2.2 Renderizar línea `Schema: <valor>` en el panel de descripción con el mismo estilo de metadatos existente.
- [x] 2.3 Añadir etiqueta traducible para `Schema` en los idiomas soportados si aplica al componente.

## 3. Validación

- [x] 3.1 Verificar compilación y checks sin errores (`cargo check` y `npm run check`).
- [x] 3.2 Validar manualmente en UI que el panel muestra `Schema` correcto para un proyecto con `config.yaml` válido.
- [x] 3.3 Validar comportamiento fallback (`No disponible`) cuando `schema` no exista.

## 1. Configuración de compatibilidad

- [x] 1.1 Crear el archivo interno de compatibilidad de versiones CLI mantenido por desarrollo (formato y schema inicial).
- [x] 1.2 Implementar validación del archivo al cargar y fallback a estado `unknown` cuando sea inválido o incompleto.

## 2. Detección y evaluación de versión CLI

- [x] 2.1 Implementar/ajustar la lectura de versión instalada de `opencode` CLI y su normalización semántica.
- [x] 2.2 Implementar módulo de evaluación de compatibilidad por rangos semver con soporte de `<`, `<=`, `>`, `>=`.
- [x] 2.3 Exponer estado triestado (`compatible`, `incompatible`, `unknown`) para consumo de UI.

## 3. Panel de compatibilidad en UI

- [x] 3.1 Crear el panel persistente superior derecho, pegado al borde derecho, con texto `Opencode Desktop`.
- [x] 3.2 Mostrar versión CLI detectada y estado de validación dentro del panel.
- [x] 3.3 Aplicar semántica de color: verde (`compatible`), rojo (`incompatible`), amarillo (`unknown`).

## 4. Pruebas y verificación

- [x] 4.1 Añadir pruebas unitarias para evaluación de rangos, incluyendo casos con `<` y `>`.
- [x] 4.2 Añadir pruebas para estados `compatible`, `incompatible` y `unknown` ante errores de detección/configuración.
- [x] 4.3 Verificar render/estilos del panel para asegurar posición y colores requeridos.

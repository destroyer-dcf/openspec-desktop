## Context

El artifact `specs` de un cambio apunta a un directorio (`.../specs`) y no a un markdown concreto. El visor actual intenta abrirlo como archivo y lanza error de sistema (`os error 21`). Se necesita convertir ese caso en estado vacío controlado y ofrecer creación guiada de archivos de especificación.

## Goals / Non-Goals

**Goals:**
- Evitar error técnico al abrir artifact `specs` cuando la ruta es directorio.
- Mostrar mensaje amigable cuando no hay `spec.md` disponibles.
- Permitir crear un `spec.md` nuevo desde ese estado.
- Mantener navegación normal cuando sí existen specs en subcarpetas.

**Non-Goals:**
- Rediseñar editor markdown general.
- Implementar generación automática completa de spec sin intervención del usuario.
- Cambiar formato OpenSpec de artifacts.

## Decisions

1. **Detección explícita de tipo de ruta**
   - Antes de leer, comprobar si la ruta de artifact es archivo o directorio.
   - Directorio `specs` se trata como contenedor, no como documento.

2. **Listado de documentos de specs por subcarpeta**
   - Resolver `specs/*/spec.md` y mostrar lista en el modal/visor.
   - Si lista vacía, renderizar estado "No existen ficheros de especificaciones".

3. **CTA de creación de spec**
   - Añadir acción "Crear especificación" que solicite nombre capability y cree `specs/<capability>/spec.md` con plantilla mínima.
   - Tras crear, refrescar lista y abrir nuevo documento.

4. **Errores técnicos solo para fallos reales de IO**
   - `Is a directory` deja de mostrarse al usuario final.
   - Solo se muestran errores si falla lectura/escritura real del fichero objetivo.

## Risks / Trade-offs

- **[Riesgo]** Crear spec con nombre inválido de capability → **Mitigación**: normalizar a kebab-case y validar no vacío.
- **[Riesgo]** Diferencias de UX entre artifacts archivo y directorio → **Mitigación**: unificar en modal con lista y acciones.
- **[Trade-off]** Mayor lógica en frontend para caso `specs` → **Mitigación**: encapsular en helpers específicos.

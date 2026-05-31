## Context

La aplicación necesita evidenciar si la versión instalada de `opencode` CLI es compatible con la versión de Opencode Desktop en uso. Hoy no existe una fuente de verdad interna ni una señal visual persistente para esta validación. La solución debe permitir mantenimiento por parte de desarrollo, soporte de rangos semánticos con operadores relacionales (incluyendo `<` y `>`), y degradación controlada cuando no pueda determinarse un estado concluyente.

## Goals / Non-Goals

**Goals:**
- Introducir un archivo interno de compatibilidad mantenido por desarrollo con reglas de versiones CLI permitidas.
- Evaluar la versión CLI detectada contra dichas reglas en runtime.
- Exponer un panel fijo arriba a la derecha, pegado al borde, con nombre del producto, versión CLI y estado.
- Representar estado con semántica de color: verde (compatible), rojo (no compatible), amarillo (indeterminado).

**Non-Goals:**
- No implementar auto-actualización del CLI.
- No añadir telemetría o reporting remoto sobre compatibilidad.
- No soportar estrategias de versionado no semántico fuera de lo declarable por el parser de rangos.

## Decisions

1. Archivo interno versionado en repositorio para política de compatibilidad.
- Se usará un archivo de configuración interno de la app (p. ej. JSON) con lista de rangos permitidos.
- Rationale: permite control explícito por release y revisión por PR.
- Alternativa descartada: hardcodear rangos en código; incrementa riesgo de drift y dificulta mantenimiento.

2. Evaluación de compatibilidad basada en semver ranges.
- Las reglas aceptarán expresiones de rango con operadores relacionales (`<`, `<=`, `>`, `>=`) y combinaciones estándar.
- Rationale: cubre el requisito de admitir `<`/`>` y evita lógica ad hoc.
- Alternativa descartada: comparación manual por segmentos; mayor complejidad y superficie de errores.

3. Modelo de estado triestado para UI (`compatible`, `incompatible`, `unknown`).
- `unknown` cubrirá casos como versión CLI no detectable, configuración inválida o ausencia de regla aplicable.
- Rationale: evita falsos negativos/positivos y comunica incertidumbre operativa.
- Alternativa descartada: binario compatible/no compatible; insuficiente ante errores de lectura o parseo.

4. Panel de compatibilidad persistente en cabecera superior derecha.
- El panel incluirá texto `Opencode Desktop`, versión CLI detectada y etiqueta/indicador de estado.
- Posicionado en la zona superior derecha pegada al borde derecho.
- Rationale: visibilidad inmediata sin navegación adicional.
- Alternativa descartada: mostrar solo en configuración; reduce descubribilidad.

## Risks / Trade-offs

- [Reglas de versión mal mantenidas] → Mitigación: validar schema del archivo al cargar y fallback a `unknown` con señal amarilla.
- [Salida de versión CLI con prefijos o formatos no esperados] → Mitigación: normalizar input antes de evaluar semver y tratar fallos como `unknown`.
- [Ruido visual por panel persistente] → Mitigación: diseño compacto y consistente con estilos de cabecera.
- [Dependencia en librería semver] → Mitigación: encapsular evaluación en módulo pequeño para facilitar reemplazo y pruebas unitarias.

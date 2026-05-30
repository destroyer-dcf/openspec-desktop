## Context

OpenSpec Desktop corre en Tauri y actualmente define tamaño/mínimos de ventana de forma estática en configuración. La aplicación ya persiste parte de su estado de UI, pero no guarda ni restaura el tamaño de la ventana principal al reiniciar.

## Goals / Non-Goals

**Goals:**
- Persistir ancho/alto de la ventana principal al cerrar.
- Restaurar el último tamaño válido al abrir la app.
- Respetar límites mínimos para evitar estados inválidos.
- No añadir dependencias nuevas.

**Non-Goals:**
- Persistir posición exacta de la ventana (x/y) en esta iteración.
- Añadir controles de configuración manual del tamaño en modal.
- Soportar layouts por monitor o perfiles múltiples de ventana.

## Decisions

- Persistir dimensiones en almacenamiento local de estado ya usado por la app (mismo circuito de bootstrap del backend).
Rationale: minimiza complejidad y mantiene una sola fuente de estado de sesión.

- Aplicar restauración solo si las dimensiones guardadas son válidas y mayores o iguales al mínimo.
Rationale: evita ventanas invisibles o demasiado pequeñas por datos corruptos.

- Capturar tamaño final en evento de cierre de ventana principal.
Rationale: garantiza que se guarda el estado real elegido por el usuario.

Alternativas consideradas:
- Guardar en archivo externo dedicado: descartado por duplicar mecanismos de persistencia.
- Guardar solo al evento de resize: descartado por mayor frecuencia de escritura y menor necesidad.

## Risks / Trade-offs

- [Dimensiones guardadas no compatibles tras cambio de monitor/resolución] → Validar contra mínimos y usar fallback a tamaño por defecto cuando no aplique.
- [Persistencia incompleta por cierre abrupto del proceso] → Guardar en evento de cierre normal; aceptar fallback por defecto en cierres no controlados.

## Migration Plan

1. Extender modelo de estado persistido con `window_width` y `window_height` opcionales.
2. Restaurar tamaño durante bootstrap antes de interacción principal.
3. Conectar evento de cierre para guardar tamaño actual.
4. Validar compilación y prueba manual: redimensionar, cerrar, reabrir.

## Validation Notes

- Compilación verificada sin errores con `cargo check` y `npm run check`.
- Arranque de app verificado con `npm run tauri dev`.
- No se detectaron limitaciones técnicas nuevas de plataforma en esta fase; queda validación visual manual de persistencia entre reinicios.

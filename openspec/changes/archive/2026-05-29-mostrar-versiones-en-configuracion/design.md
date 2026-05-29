## Context

El modal de configuración ya concentra preferencias de tema y densidad, pero no muestra información técnica de versiones. Hoy, para saber versión de OpenSpec o de la app, hay que salir a terminal o revisar archivos de proyecto.

## Goals / Non-Goals

**Goals:**
- Exponer en Configuración la versión de OpenSpec CLI y de la app.
- Ubicar el bloque en la parte superior del modal para consulta inmediata.
- Mantener estilo consistente con temas light/dark existentes.
- Definir fallbacks cuando OpenSpec CLI no esté instalado o no responda.

**Non-Goals:**
- No gestionar actualización automática de OpenSpec o de la app.
- No añadir telemetría ni envío remoto de versiones.
- No cambiar lógica actual de preferencias visuales.

## Decisions

- Añadir comando backend `get_versions` que devuelva `{ appVersion, openspecVersion }`.
- Resolver `appVersion` desde metadatos de build (crate/package version).
- Resolver `openspecVersion` invocando `openspec --version` y normalizando salida.
- Si falla invocación, devolver valor fallback descriptivo (`No disponible`).
- Renderizar panel superior read-only en `SettingsModal` con dos filas de versión.

## Risks / Trade-offs

- [Riesgo] Latencia al abrir Configuración por llamada a CLI.
  Mitigación: cargar versiones una vez al abrir modal y cachear durante la sesión.

- [Riesgo] Formatos distintos de salida en versiones de OpenSpec.
  Mitigación: mostrar salida saneada sin parseos frágiles.

- [Riesgo] Error de comando en entornos sin OpenSpec.
  Mitigación: fallback legible en UI sin romper apertura del modal.

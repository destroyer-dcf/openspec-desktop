# about-metadata-legal Specification

## Purpose
Display copyright and repository information in the native About dialog to establish institutional identity and provide access to project source.

## Requirements

### Requirement: About incluye copyright y repositorio
La aplicación SHALL mostrar en la ventana nativa About el texto `Copyright Destroyer 2026` y la URL oficial del repositorio `https://github.com/destroyer-dcf/openspec-desktop`.

#### Scenario: Apertura de About con metadata completa
- **WHEN** el usuario abre la ventana About desde el menú de la aplicación
- **THEN** la ventana muestra nombre de producto, versión, copyright y URL de repositorio

#### Scenario: Consistencia de metadatos entre ejecuciones
- **WHEN** la aplicación se inicia en cualquier ejecución normal
- **THEN** el contenido de copyright y repositorio en About permanece fijo y no depende de preferencias de usuario

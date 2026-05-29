## 1. Base de internacionalización

- [x] 1.1 Crear estructura de diccionarios para `en`, `fr`, `de`, `pt` con fallback a español
- [x] 1.2 Añadir estado global de idioma activo y helper de traducción por clave
- [x] 1.3 Sustituir textos hardcodeados de pantallas principales por claves traducibles

## 2. Integración en configuración

- [x] 2.1 Añadir selector de idioma en `SettingsModal`
- [x] 2.2 Persistir preferencia de idioma junto con el resto de preferencias UI
- [x] 2.3 Aplicar cambio de idioma en caliente sin reiniciar app

## 3. Validación

- [x] 3.1 Verificar cambio correcto a inglés, francés, alemán y portugués
- [x] 3.2 Verificar persistencia del idioma tras reiniciar la app
- [x] 3.3 Verificar fallback cuando falte una clave sin romper UI

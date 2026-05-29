## 1. Reubicación del resumen global

- [x] 1.1 Identificar el componente/contenedor actual del rosco de resumen global en dashboard.
- [x] 1.2 Mover el render del rosco al panel de descripción del proyecto en posición derecha.
- [x] 1.3 Eliminar el bloque redundante del rosco en su ubicación anterior.

## 2. Reestructuración de datos del proyecto

- [x] 2.1 Cambiar layout de datos de proyecto a formato línea por línea (etiqueta + valor).
- [x] 2.2 Ajustar estilos para legibilidad vertical y separación consistente entre líneas.
- [x] 2.3 Mantener fallback de valores no definidos sin romper el nuevo layout.

## 3. Integración visual del panel combinado

- [x] 3.1 Definir layout de dos zonas en panel descripción: datos a la izquierda, rosco a la derecha.
- [x] 3.2 Ajustar alineación y tamaños para evitar solapes en anchos reducidos de escritorio.
- [x] 3.3 Verificar que el panel de cambios activos queda debajo sin regresiones de espaciado.

## 4. Validación funcional

- [x] 4.1 Validar que el valor del rosco no cambia tras moverlo de ubicación.
- [x] 4.2 Validar visualmente en tema claro/oscuro la legibilidad de datos línea a línea.
- [x] 4.3 Validar apertura con proyecto sin contexto completo y fallback "No definido".

---
name: chequeo de version cli de opencode
type: feature
createdAt: 1780222308
---

Es necesario que nuestra aplicación cheque si es compatible con la versión cli de opencode instalada. Para ello necesito un fichero interno de la aplicación, en el cual estara las versiones para las que es valido nuestra aplicación. Para mostrarselo al usuario, crearemos un panel, arriba alineado a la derecha (pegado al borde derecho), de las letras Opencode Desktop. Lo haremos eun panel que ponga la versión del cli y si esta validado nuestra aplicación para ello, el panel cambiara de color dependiendo de esa validación, si esta todo ok con respecto al archivo y a la versión cli estara en verde, rojo si no esta validada nuestra aplicación para ello, o amarillo. Ese fichero sera mantenido por el desarrollador. El fichero debe admitir <> en las versiones para esa validación.

# Solana-Gaming-PDA
- (CRUD + PDA)

Este proyecto es un contrato inteligente desarrollado en Solana utilizando Rust y el framework Anchor. Fue creado como proyecto de certificación y simula un sistema backend para perfiles de jugadores en un videojuego (Web3 Gaming).

- Descripción del Proyecto

El programa permite gestionar la progresión de un jugador utilizando un **PDA (Program Derived Address)**. El uso del PDA garantiza que cada wallet solo pueda tener un único perfil asociado, evitando cuentas duplicadas y asegurando que solo el dueño de la wallet pueda modificar su progreso.

### Funcionalidades (CRUD)
* **Create:** La instrucción `crear_jugador` inicializa una cuenta PDA usando como semilla la palabra "perfil" y la Public Key del usuario. Se le asigna un nombre de usuario (String), nivel 1 y 0 de experiencia.
* **Read:** Al estar desplegado en la red, cualquier cliente puede consultar la cuenta PDA del jugador para leer su `username`, `nivel` y `xp` de forma pública y transparente.
* **Update:** La instrucción `ganar_experiencia` permite sumar XP al perfil. El contrato incluye lógica interna: por cada 100 XP obtenidos, el jugador sube de nivel automáticamente y la experiencia sobrante se conserva. Solo el creador (Signer) puede invocar esto.
* **Delete:** La instrucción `eliminar_jugador` permite al usuario borrar su perfil del juego. Al cerrar la cuenta, los lamports retenidos por la renta de la blockchain son devueltos automáticamente a la wallet del dueño.

-test del codigo

1. Abre [Solana Playground](https://beta.solpg.io/).
2. Crea un nuevo proyecto y pega el código de `lib.rs`.
3. Haz clic en **Build** para compilar el programa.
4. Asegúrate de tener fondos en Devnet (`solana airdrop 2` en la terminal).
5. Haz clic en **Deploy** para subir el contrato a la red de prueba.
6. Utiliza el panel de cliente generado automáticamente a la izquierda para ejecutar las instrucciones (`crear_jugador`, `ganar_experiencia`, `eliminar_jugador`).

-🎮Solana Gaming Profile (CRUD + Toggle + PDA) 

Este repositorio contiene la versión mejorada y completa del contrato inteligente para la gestión de perfiles de jugadores en la red de Solana. El proyecto ha sido desarrollado utilizando Rust y el framework Anchor, cumpliendo con los requisitos de un sistema CRUD completo, manejo de PDA (Program Derived Addresses) e instrucciones de estado alternable

-Descripción del Proyecto-

El programa permite administrar la persistencia de datos de jugadores de forma descentralizada. Se utiliza un PDA basado en la wallet del usuario para garantizar la propiedad única del perfil: un usuario, un perfil.

-Funcionalidades Implementadas (CRUD + Toggle)

1.  **Create (Crear):** La instrucción `crear_jugador` inicializa la cuenta en la blockchain, asignando un nombre de usuario, nivel inicial (1), experiencia (0) y estado activo (`true`).
2.  **Read (Leer):** Se implementó la función `ver_jugador`. A diferencia de otras instrucciones, esta es de solo lectura (no mutable), permitiendo consultar las estadísticas del jugador (Nombre, Nivel, XP, Estado) directamente desde el programa.
3.  **Update (Actualizar - XP):** La función `ganar_experiencia` permite modificar la XP del jugador. Incluye lógica de negocio para subir de nivel automáticamente al alcanzar cada 100 puntos de experiencia.
4.  **Update (Actualizar - Toggle/Alternar):** Se añadió la función `alternar_estado`. Esta permite cambiar el estado del jugador entre "Activo" e "Inactivo" (True/False), cumpliendo con el requisito de alternancia de estado.
5.  **Delete (Eliminar):** La instrucción `eliminar_jugador` cierra la cuenta del PDA y devuelve los lamports (SOL) depositados por concepto de renta a la wallet del propietario original.

 -Requisitos Técnicos

* **Lenguaje:** Rust
* **Framework:** Anchor
* **Entorno:** Solana Playground o Solana CLI local.
* **Red:** Devnet (requiere SOL de prueba).

-Cómo utilizar este proyecto(si se usa de ejemplo para otro proyecto)

-Preparación en Solana Playground
1. Dirígete a [Solana Playground](https://beta.solpg.io/).
2. Crea un nuevo proyecto e importa el código contenido en `lib.rs`.
3. Ejecuta `Build` para compilar el programa.

-Despliegue (Deploy)
1. Asegúrate de tener saldo en tu wallet de Playground: `solana airdrop 2`.
2. Haz clic en `Deploy` para subir el contrato a la Devnet.

-Interacción
Una vez desplegado, utiliza el panel de "Client" para ejecutar las siguientes funciones en orden:
* **`crear_jugador`**: Envía un nombre (string).
* **`ver_jugador`**: Consulta tus datos actuales.
* **`ganar_experiencia`**: Suma puntos para subir de nivel.
* **`alternar_estado`**: Cambia tu estado de conexión.
* **`eliminar_jugador`**: Borra el perfil y recupera tu SOL.

 -Seguridad
El contrato utiliza validaciones de seguridad mediante `has_one = owner` y semillas de PDA, asegurando que solo el dueño de la wallet pueda modificar o eliminar su propio perfil de juego.

---
Desarrollado para la Certificación de Desarrollador Solana

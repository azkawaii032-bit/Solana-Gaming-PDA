use anchor_lang::prelude::*;

// ID del Solana Program (Playground lo actualiza automáticamente al hacer build)
declare_id!("6BQoT32CiV1ggGRPKKvCGmC7qaPAWvBF4xucRd2YYkvW"); 

#[program]
pub mod solana_gaming_profile {
    use super::*;

    /// =======================================================================
    /// C - CREATE: Crea un nuevo perfil de jugador (PDA) vinculado a su wallet
    /// =======================================================================
    pub fn crear_jugador(ctx: Context<CrearJugador>, username: String) -> Result<()> {
        let jugador = &mut ctx.accounts.jugador;
        
        // Inicializamos los datos del jugador
        jugador.owner = ctx.accounts.owner.key();
        jugador.username = username;
        jugador.nivel = 1; // Todos empiezan en nivel 1
        jugador.xp = 0;
        jugador.bump = ctx.bumps.jugador; // Guardamos el bump generado por el PDA
        
        msg!("¡Perfil creado! Jugador: {} | Nivel: {}", jugador.username, jugador.nivel);
        Ok(())
    }

    /// =======================================================================
    /// U - UPDATE: Añade experiencia al jugador y lo sube de nivel si es necesario
    /// =======================================================================
    pub fn ganar_experiencia(ctx: Context<ModificarJugador>, xp_ganada: u32) -> Result<()> {
        let jugador = &mut ctx.accounts.jugador;
        jugador.xp += xp_ganada;
        
        // Lógica del juego: Cada 100 XP se sube de nivel
        if jugador.xp >= 100 {
            jugador.nivel += 1;
            jugador.xp -= 100; // Restamos la XP usada para subir de nivel
            msg!("¡Level Up! {} ha alcanzado el nivel {}", jugador.username, jugador.nivel);
        } else {
            msg!("{} ganó {} XP. Total XP: {}", jugador.username, xp_ganada, jugador.xp);
        }
        Ok(())
    }

    /// =======================================================================
    /// D - DELETE: Elimina el perfil del jugador y devuelve los SOL de renta
    /// =======================================================================
    pub fn eliminar_jugador(_ctx: Context<EliminarJugador>) -> Result<()> {
        // La macro 'close' en el contexto se encarga de transferir los fondos 
        // de vuelta al owner y borrar la cuenta de la blockchain.
        msg!("Perfil de jugador eliminado. Los SOL de renta han sido devueltos.");
        Ok(())
    }
}

// ----------------------------------------------------------------------------
// ESTRUCTURAS DE CONTEXTO (Donde definimos el PDA y permisos)
// ----------------------------------------------------------------------------

#[derive(Accounts)]
pub struct CrearJugador<'info> {
    #[account(
        init, 
        payer = owner, 
        // Espacio: Discriminador(8) + Pubkey(32) + String Prefix(4) + String(20 max) + u16(2) + u32(4) + u8(1)
        space = 8 + 32 + 4 + 20 + 2 + 4 + 1, 
        // ¡REQUISITO PDA!: Usamos "perfil" y la llave pública del creador como semillas
        seeds = [b"perfil", owner.key().as_ref()], 
        bump
    )]
    pub jugador: Account<'info, Jugador>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModificarJugador<'info> {
    #[account(
        mut,
        seeds = [b"perfil", owner.key().as_ref()],
        bump = jugador.bump,
        has_one = owner // ¡Seguridad!: Solo el dueño original puede modificar su XP
    )]
    pub jugador: Account<'info, Jugador>,
    pub owner: Signer<'info>, // Quien firma la transacción
}

#[derive(Accounts)]
pub struct EliminarJugador<'info> {
    #[account(
        mut,
        close = owner, // REQUISITO DELETE: Transfiere los lamports de regreso al owner
        seeds = [b"perfil", owner.key().as_ref()],
        bump = jugador.bump,
        has_one = owner
    )]
    pub jugador: Account<'info, Jugador>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

// ----------------------------------------------------------------------------
// ESTRUCTURA DE LA CUENTA (El estado guardado en la blockchain)
// ----------------------------------------------------------------------------

#[account]
pub struct Jugador {
    pub owner: Pubkey,   // Dueño de la cuenta
    pub username: String, // Nombre en el juego
    pub nivel: u16,      // Nivel actual
    pub xp: u32,         // Experiencia actual
    pub bump: u8,        // Bump del PDA
}

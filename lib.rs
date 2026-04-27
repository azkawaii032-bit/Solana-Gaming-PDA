use anchor_lang::prelude::*;

// ID del Solana Program (Playground lo actualiza automáticamente al hacer build)
declare_id!("6BQoT32CiV1ggGRPKKvCGmC7qaPAWvBF4xucRd2YYkvW"); 

#[program]
pub mod solana_gaming_profile {
    use super::*;

    /// =======================================================================
    /// C - CREATE: Crea un nuevo perfil de jugador (PDA)
    /// =======================================================================
    pub fn crear_jugador(ctx: Context<CrearJugador>, username: String) -> Result<()> {
        let jugador = &mut ctx.accounts.jugador;
        
        jugador.owner = ctx.accounts.owner.key();
        jugador.username = username;
        jugador.nivel = 1; 
        jugador.xp = 0;
        jugador.is_active = true; // Inicializamos el estado alternable en verdadero
        jugador.bump = ctx.bumps.jugador; 
        
        msg!("¡Perfil creado! Jugador: {} | Activo: {}", jugador.username, jugador.is_active);
        Ok(())
    }

    /// =======================================================================
    /// R - READ: Ver los datos actuales del jugador (Solo Lectura)
    /// =======================================================================
    pub fn ver_jugador(ctx: Context<VerJugador>) -> Result<()> {
        // Observa que aquí no usamos "&mut" porque no vamos a modificar nada, solo leer.
        let jugador = &ctx.accounts.jugador;
        
        msg!("--- ESTADÍSTICAS DEL JUGADOR ---");
        msg!("Nombre: {}", jugador.username);
        msg!("Nivel: {} | XP: {}", jugador.nivel, jugador.xp);
        msg!("Estado Activo: {}", jugador.is_active);
        
        Ok(())
    }

    /// =======================================================================
    /// U - UPDATE 1: Añade experiencia al jugador
    /// =======================================================================
    pub fn ganar_experiencia(ctx: Context<ModificarJugador>, xp_ganada: u32) -> Result<()> {
        let jugador = &mut ctx.accounts.jugador;
        jugador.xp += xp_ganada;
        
        if jugador.xp >= 100 {
            jugador.nivel += 1;
            jugador.xp -= 100;
            msg!("¡Level Up! {} ha alcanzado el nivel {}", jugador.username, jugador.nivel);
        } else {
            msg!("{} ganó {} XP. Total XP: {}", jugador.username, xp_ganada, jugador.xp);
        }
        Ok(())
    }

    /// =======================================================================
    /// U - UPDATE 2: Alternar el estado activo/inactivo del jugador
    /// =======================================================================
    pub fn alternar_estado(ctx: Context<ModificarJugador>) -> Result<()> {
        let jugador = &mut ctx.accounts.jugador;
        
        // Invertimos el valor booleano actual (si es true pasa a false, y viceversa)
        jugador.is_active = !jugador.is_active;
        
        msg!("El estado de conexión de {} ha cambiado. Activo: {}", jugador.username, jugador.is_active);
        Ok(())
    }

    /// =======================================================================
    /// D - DELETE: Elimina el perfil del jugador
    /// =======================================================================
    pub fn eliminar_jugador(_ctx: Context<EliminarJugador>) -> Result<()> {
        msg!("Perfil de jugador eliminado. Los SOL de renta han sido devueltos.");
        Ok(())
    }
}

// ----------------------------------------------------------------------------
// ESTRUCTURAS DE CONTEXTO
// ----------------------------------------------------------------------------

#[derive(Accounts)]
pub struct CrearJugador<'info> {
    #[account(
        init, 
        payer = owner, 
        // Agregamos 1 byte extra al espacio de memoria para guardar el nuevo campo booleano
        space = 8 + 32 + 4 + 20 + 2 + 4 + 1 + 1, 
        seeds = [b"perfil", owner.key().as_ref()], 
        bump
    )]
    pub jugador: Account<'info, Jugador>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Nueva estructura exclusiva para la función de lectura (sin el atributo mut)
#[derive(Accounts)]
pub struct VerJugador<'info> {
    #[account(
        seeds = [b"perfil", owner.key().as_ref()],
        bump = jugador.bump,
        has_one = owner
    )]
    pub jugador: Account<'info, Jugador>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ModificarJugador<'info> {
    #[account(
        mut,
        seeds = [b"perfil", owner.key().as_ref()],
        bump = jugador.bump,
        has_one = owner 
    )]
    pub jugador: Account<'info, Jugador>,
    pub owner: Signer<'info>, 
}

#[derive(Accounts)]
pub struct EliminarJugador<'info> {
    #[account(
        mut,
        close = owner, 
        seeds = [b"perfil", owner.key().as_ref()],
        bump = jugador.bump,
        has_one = owner
    )]
    pub jugador: Account<'info, Jugador>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

// ----------------------------------------------------------------------------
// ESTRUCTURA DE LA CUENTA
// ----------------------------------------------------------------------------

#[account]
pub struct Jugador {
    pub owner: Pubkey,   
    pub username: String, 
    pub nivel: u16,      
    pub xp: u32,         
    pub is_active: bool, // función Alternar
    pub bump: u8,        
}

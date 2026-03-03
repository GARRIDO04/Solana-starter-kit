use anchor_lang::prelude::*;
declare_id!("YgmRSAUmBegew9qkC8h93omVr9NLy8mGwUZ36sxNmug");

#[program]
pub mod playlist {
    use super::*;
    pub fn crear_playlist(context: Context<CrearPlaylist>, nombre: String) -> Result<()> {
        let playlist = &mut context.accounts.playlist;
        playlist.nombre = nombre;
        playlist.owner = context.accounts.owner.key();
        playlist.canciones = Vec::new();
        Ok(())
    }

    pub fn agregar_cancion(context: Context<AgregarCancion>, cancion: Cancion) -> Result<()> {
        let playlist = &mut context.accounts.playlist;

        // Validamos que el firmante sea el owner
        //require!(playlist.owner == context.accounts.owner.key(),
        // ProgramError::IllegalOwner);

        playlist.canciones.push(cancion);

        Ok(())
        }

    pub fn ver_playlist(context: Context<VerPlaylist>) -> Result<()>{
        let playlist = &context.accounts.playlist;

        msg!("--- PLAYLIST ---");
        
        msg!("Nombre: {}", playlist.nombre);

        msg!("Total canciones: {}", playlist.canciones.len());
        for (i, c) in playlist.canciones.iter().enumerate() {
            msg!(
                "#{} | {} - {} | disponible: {}",
                i + 1,
                c.nombre,
                c.artista,
                c.disponible
            );
        }
        Ok(())
        }

}

#[derive(InitSpace)]
#[account]
pub struct Playlist {
    #[max_len(60)]
    nombre: String,

    owner: Pubkey,

    #[max_len(10)]
    canciones: Vec<Cancion>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Cancion {
    #[max_len(40)]
    nombre: String,

    #[max_len(40)]
    artista: String,

    disponible: bool,
}

#[derive(Accounts)]
pub struct CrearPlaylist<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Playlist::INIT_SPACE + 8,
        seeds = [b"playlist", owner.key().as_ref()],
        bump
    )]
    pub playlist: Account<'info, Playlist>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AgregarCancion<'info> {
    #[account(mut)]
    pub playlist: Account<'info, Playlist>,

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerPlaylist<'info>{
    pub playlist: Account<'info, Playlist>,
}

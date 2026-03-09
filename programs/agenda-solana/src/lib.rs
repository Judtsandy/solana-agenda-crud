use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod agenda_solana {

    use super::*;

    pub fn crear_tarea(
        ctx: Context<CrearTarea>,
        titulo: String,
        descripcion: String,
        fecha: String,
    ) -> Result<()> {

        let tarea = &mut ctx.accounts.tarea;

        tarea.titulo = titulo;
        tarea.descripcion = descripcion;
        tarea.fecha = fecha;
        tarea.estado = false;
        tarea.autor = *ctx.accounts.usuario.key;

        Ok(())
    }

    pub fn actualizar_estado(
        ctx: Context<ActualizarTarea>,
        estado: bool,
    ) -> Result<()> {

        let tarea = &mut ctx.accounts.tarea;

        tarea.estado = estado;

        Ok(())
    }

    pub fn editar_tarea(
        ctx: Context<EditarTarea>,
        titulo: String,
        descripcion: String,
        fecha: String,
    ) -> Result<()> {

        let tarea = &mut ctx.accounts.tarea;

        tarea.titulo = titulo;
        tarea.descripcion = descripcion;
        tarea.fecha = fecha;

        Ok(())
    }

    pub fn eliminar_tarea(
        _ctx: Context<EliminarTarea>
    ) -> Result<()> {

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(titulo: String)]
pub struct CrearTarea<'info> {

    #[account(
        init,
        payer = usuario,
        space = 8 + 200,
        seeds = [b"tarea", usuario.key().as_ref(), titulo.as_bytes()],
        bump
    )]
    pub tarea: Account<'info, Task>,

    #[account(mut)]
    pub usuario: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActualizarTarea<'info> {

    #[account(mut)]
    pub tarea: Account<'info, Task>,

    pub usuario: Signer<'info>,
}

#[derive(Accounts)]
pub struct EditarTarea<'info> {

    #[account(mut)]
    pub tarea: Account<'info, Task>,

    pub usuario: Signer<'info>,
}

#[derive(Accounts)]
pub struct EliminarTarea<'info> {

    #[account(
        mut,
        close = usuario
    )]
    pub tarea: Account<'info, Task>,

    #[account(mut)]
    pub usuario: Signer<'info>,
}

#[account]
pub struct Task {

    pub titulo: String,
    pub descripcion: String,
    pub fecha: String,
    pub estado: bool,
    pub autor: Pubkey,
}
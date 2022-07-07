use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod hello_tests {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let foo = &mut ctx.accounts.foo;
        foo.authority = ctx.accounts.authority.key();
        foo.bump = ctx.bumps["foo"];

        let clock = Clock::get()?;
        let now_ts = clock.unix_timestamp as u64;

        #[cfg(feature="test")]
        msg!("initialize:::> clock: {0}", now_ts);
        foo.start_utc_in_seconds = now_ts;

        foo.last_pause_timestamp = 0;
        foo.last_resume_timestamp = 0;
        foo.status = Status::Running as u8;

        #[cfg(feature="test")]
        msg!("initialize:::> clock: {0}", now_ts);

        #[cfg(feature="test")]
        msg!("initialize:::>\n {0}", foo.data_string());
        Ok(())
    }

    pub fn pause(ctx: Context<Pause>) -> Result<()> {
        let foo = &mut ctx.accounts.foo;

        let clock = Clock::get()?;
        let now_ts = clock.unix_timestamp as u64;

        #[cfg(feature="test")]
        msg!("pause:::>clock: {0}", now_ts);

        foo.last_pause_timestamp = now_ts;
        foo.status = Status::Pause as u8;

        #[cfg(feature="test")]
        msg!("pause:::>\n {0}", foo.data_string());
        Ok(())
    }

    pub fn resume(ctx: Context<Resume>) -> Result<()> {
        let foo = &mut ctx.accounts.foo;

        let clock = Clock::get()?;
        let now_ts = clock.unix_timestamp as u64;

        #[cfg(feature="test")]
        msg!("pause:::>clock: {0}", now_ts);

        foo.last_resume_timestamp = now_ts;
        foo.status = Status::Running as u8;

        #[cfg(feature="test")]
        msg!("resume:::>\n {0}", foo.data_string());

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [authority.key().as_ref()],
        bump,
        payer = authority,
        space = Foo::LEN + 8
    )]
    pub foo: Account<'info, Foo>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Pause<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [authority.key().as_ref()],
        bump = foo.bump,
        constraint = foo.status == Status::Pause as u8,
        has_one = authority,
    )]
    pub foo: Account<'info, Foo>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Resume<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [authority.key().as_ref()],
        constraint = foo.status == Status::Pause as u8,
        bump = foo.bump,
        has_one = authority,
    )]
    pub foo: Account<'info, Foo>,

    pub system_program: Program<'info, System>,
}

#[derive(Default)]
#[account]
pub struct Foo {
    pub bump: u8,                   // 1
    pub authority: Pubkey,          // 32
    pub start_utc_in_seconds: u64,  // 8
    pub status: u8,                 // 1
    pub last_pause_timestamp: u64,  // 8
    pub last_resume_timestamp: u64, // 8
}

impl Foo {
    pub const LEN: usize = 1 + 32 + 8 + 1 + 8 + 8;
    pub fn data_string<'info>(&self) -> String {
        return format!("{{\n\tauthority: {0},\n\tstart_utc_in_seconds: {1},\n\tstatus: {2},\n\tlast_pause_timestamp: {3},\n\tlast_resume_timestamp: {4}\n}}", 
            self.authority.to_string(),
            self.start_utc_in_seconds,
            self.status,
            self.last_pause_timestamp,
            self.last_resume_timestamp
        );
    }
}

#[repr(u8)]
#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize)]
pub enum Status {
    Running = 0,
    Pause = 1,
}

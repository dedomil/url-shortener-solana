use anchor_lang::prelude::*;

declare_id!("5evXQyffaUhYwCKGg7SoTZTfdDXCwKaQogTYQ873vHmg");

#[program]
mod url_shortener {
    use super::*;

    pub fn create(ctx: Context<Create>, url: String) -> Result<()> {
        let url_account: &mut Account<UrlAccount> = &mut ctx.accounts.url_account;

        if url.chars().count() < 1 {
            return err!(Errors::EmptyURL);
        }

        url_account.authority = ctx.accounts.signer.key();
        url_account.url = url;
        url_account.shortcode = Clock::get().unwrap().unix_timestamp.to_string();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(
        init,
        payer = signer,
        space = 32 + 5 * 4 + 200 + 8
    )]
    pub url_account: Account<'info, UrlAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UrlAccount {
    authority: Pubkey,
    shortcode: String,
    url: String,
}

#[error_code]
pub enum Errors {
    #[msg("URL cannot be empty")]
    EmptyURL,
}

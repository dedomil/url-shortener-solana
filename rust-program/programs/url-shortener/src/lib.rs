use anchor_lang::prelude::*;

declare_id!("5evXQyffaUhYwCKGg7SoTZTfdDXCwKaQogTYQ873vHmg");

#[program]
mod url_shortener {
    use super::*;

    pub fn create(ctx: Context<Create>, args: CreateUrlAccountArgs) -> Result<()> {
        let url_account: &mut Account<UrlAccount> = &mut ctx.accounts.url_account;

        if args.url.chars().count() < 1 {
            return err!(Errors::EmptyURL);
        }

        url_account.authority = ctx.accounts.signer.key();
        url_account.url = args.url;
        url_account.shortcode = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(args: CreateUrlAccountArgs)]
pub struct Create<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 // discriminator 8
            + 32 // authority 40
            + 8 // shortcode 48
            + 4 + args.url.len() // 4 + bytes | 52+
    )]
    pub url_account: Account<'info, UrlAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UrlAccount {
    // discriminator   // 8 = 8
    authority: Pubkey, // 32 = 40
    shortcode: i64,    // 8 = 48
    // fields below cannot be accessed by `getProgramAccounts` since they are of variable size
    url: String, // 4+
}

#[error_code]
pub enum Errors {
    #[msg("URL cannot be empty")]
    EmptyURL,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateUrlAccountArgs {
    url: String,
}

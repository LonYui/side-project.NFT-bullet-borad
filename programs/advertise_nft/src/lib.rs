extern crate anchor_lang;

use anchor_lang::prelude::*;

declare_id!("CRafZpWZ7GkbeU72ED6bcUuGsdJHR9trxvSm1rrjgXX9");

#[program]
pub mod advertise_nft {
    use super::*;
    // use anchor_lang::ToAccountInfo;

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        // Get a reference to the account.
        let base_account = &mut ctx.accounts.base_account;
        // Initialize total_gifs.
        base_account.total_gifs = 0;
        Ok(())
    }

    // Another function woo!
    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result<()> {
        // Get a reference to the account and increment total_gifs.
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct{gif_link: gif_link,user_address:*user.to_account_info().key,vote_count:0,};

        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    pub fn up_vote(ctx: Context<Upvote>, gif_link:String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        for mut item in base_account.gif_list.iter_mut() {
            if item.gif_link==gif_link {
                let result = item.vote_count+1;
                std::mem::replace(&mut item.vote_count, result);
                // send tip to base_account
                // transfer_service_fee_lamports(ctx.accounts.user.to_account_info(),base_account.to_account_info(),10^9)
                anchor_lang::solana_program::system_instruction::transfer(ctx.accounts.user.key(),item.user_address,10^9)
                // item.user_address
            }
        }
        Ok(())
    }

    // /// Transfers lamports from one account (must be program owned)
    // /// to another account. The recipient can by any account
    // fn transfer_service_fee_lamports(
    //     from_account: &AccountInfo,
    //     to_account: &AccountInfo,
    //     amount_of_lamports: u64,
    // ) -> ProgramResult {
    //     // Does the from account have enough lamports to transfer?
    //     if **from_account.try_borrow_lamports()? < amount_of_lamports {
    //         return Err(CustomError::InsufficientFundsForTransaction.into());
    //     }
    //     // Debit from_account and credit to_account
    //     **from_account.try_borrow_mut_lamports()? -= amount_of_lamports;
    //     **to_account.try_borrow_mut_lamports()? += amount_of_lamports;
    //     // Ok(())
    // }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Specify what data you want in the AddGif Context.
// Getting a handle on the flow of things :)?
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Upvote<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}


#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link:String,
    pub user_address:Pubkey,
    pub vote_count:u8,
}
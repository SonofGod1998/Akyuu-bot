use poise::serenity_prelude as serenity;

use crate::{Context, Error};

/// register slash commands to a guild or globally
#[poise::command(prefix_command, hide_in_help, owners_only)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
	poise::builtins::register_application_commands_buttons(ctx).await?;
	Ok(())
}

/// clear the move database
#[poise::command(prefix_command, hide_in_help, owners_only)]
pub async fn resetmoves(ctx: Context<'_>) -> Result<(), Error> {
	let yes_id = format!("{}yes", ctx.id());
	let reply = ctx
		.send(|r| {
			r.content("are you sure?").components(|c| {
				c.create_action_row(|ar| {
					ar.create_button(|b| {
						b.label("yes").custom_id(yes_id).style(serenity::ButtonStyle::Danger)
					})
				})
			})
		})
		.await?;

	let interaction =
		reply.message().await?.await_component_interaction(ctx).author_id(ctx.author().id).await;

	if let Some(press) = &interaction {
		// FIX: the character sql files are not working when used though sqlx but are working in
		// all other scenarios
		ctx.reply("currently not working").await?;

		// sqlx::query("DELETE from `moves`;").execute(&ctx.data().pool).await?;

		// the sql files must be explicitly written out
		// sqlx::query_file!("character-sql/byakuren.sql").execute(&ctx.data().pool).await?;
		// sqlx::query_file!("character-sql/doremy.sql").execute(&ctx.data().pool).await?;
		// sqlx::query_file!("character-sql/futo.sql").execute(&ctx.data().pool).await?;
		// sqlx::query_file!("character-sql/ichirin.sql").execute(&ctx.data().pool).await?;
		// sqlx::query_file!("character-sql/joon.sql").execute(&ctx.data().pool).await?;
		// sqlx::query_file!("character-sql/kasen.sql").execute(&ctx.data().pool).await?;
		// sqlx::query_file!("character-sql/koishi.sql").execute(&ctx.data().pool).await?;
		// sqlx::query_file!("character-sql/kokoro.sql").execute(&ctx.data().pool).await?;
		// sqlx::query_file!("character-sql/mami.sql").execute(&ctx.data().pool).await?;
		// sqlx::query_file!("character-sql/marisa.sql").execute(&ctx.data().pool).await?;
		// sqlx::query_file!("character-sql/miko.sql").execute(&ctx.data().pool).await?;
		// sqlx::query_file!("character-sql/mokou.sql").execute(&ctx.data().pool).await?;
		// sqlx::query_file!("character-sql/nitori.sql").execute(&ctx.data().pool).await?;
		// sqlx::query_file!("character-sql/reimu.sql").execute(&ctx.data().pool).await?;
		// sqlx::query_file!("character-sql/reisen.sql").execute(&ctx.data().pool).await?;
		// sqlx::query_file!("character-sql/sukuna.sql").execute(&ctx.data().pool).await?;
		// sqlx::query_file!("character-sql/sumi.sql").execute(&ctx.data().pool).await?;
		// sqlx::query_file!("character-sql/tenshi.sql").execute(&ctx.data().pool).await?;
		// sqlx::query_file!("character-sql/yukari.sql").execute(&ctx.data().pool).await?;

		press
			.create_interaction_response(ctx, |b| b.kind(serenity::InteractionResponseType::Pong))
			.await?;
	}

	Ok(())
}
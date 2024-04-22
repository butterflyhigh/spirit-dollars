use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommandOption, ResolvedValue};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::database::Id;
use crate::GlobalDatabase;

pub async fn run(ctx: &Context, options: &[ResolvedOption<'_>], interaction: &CommandInteraction) -> String {
    let username = options.iter().find(|opt| opt.name == "user");

    if let Some(ResolvedOption {
        value: ResolvedValue::User(user, _), ..
    }) = username {
        let id = Id(user.id.to_string());
        let sender_id = Id(interaction.user.id.to_string());

        let data = ctx.data.read().await;
        let db = data.get::<GlobalDatabase>().unwrap();

        match db.get_amount(&sender_id, &id) {
            Ok(amt) => {
                format!("{} owes {} {} spirit dollars", interaction.user.name, user.name, amt)
            },
            Err(_) => {
                format!("{} does not owe {} anything", interaction.user.name, user.name)
            }
        }
    } else {
        String::from("Something bad happened")
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("getuserdebt")
        .description("See how much spirit money you owe someone")
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "The user you owe")
                .required(true),
        )
}

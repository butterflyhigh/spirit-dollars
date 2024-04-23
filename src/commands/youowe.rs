use serenity::all::{CommandInteraction, CommandOptionType, CreateCommandOption, Mentionable, ResolvedValue};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub async fn run(options: &[ResolvedOption<'_>], interaction: &CommandInteraction) -> String {
    let username = options.iter().find(|opt| opt.name == "user");
    let amount = options.iter().find(|opt| opt.name == "amount");

    if let Some(ResolvedOption {
        value: ResolvedValue::User(user, _), ..
    }) = username && let Some(ResolvedOption {
        value: ResolvedValue::Number(amt), ..
    }) = amount {
        format!("Hey {}, you owe {} {} spirit dollars. Pay up!!!!", user.mention(), interaction.user.name, amt)
    } else {
        String::from("Something bad happened")
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("youowe")
        .description("\"politely\" remind someone they owe you spirit dollars (doesn't change the database)")
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "The user who's scamming you out of your spirit dollars")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::Number, "amount", "How much they owe you")
                .required(true)
        )
}

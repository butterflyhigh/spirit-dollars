use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommandOption, ResolvedValue};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::database::{self, Id};
use crate::GlobalDatabase;

pub async fn run(ctx: &Context, options: &[ResolvedOption<'_>], interaction: &CommandInteraction) -> String {
    let username = options.iter().find(|opt| opt.name == "user");
    let amount = options.iter().find(|opt| opt.name == "amount");

    if let Some(ResolvedOption {
        value: ResolvedValue::User(user, _), ..
    }) = username && let Some(ResolvedOption {
        value: ResolvedValue::Number(amt),
        ..
    }) = amount {
        let id = Id(user.id.to_string());
        let dollas = amt;
        let sender_id = Id(interaction.user.id.to_string());

        let debt = database::Debt::new(id, *dollas);
        let data = ctx.data.read().await;
        let db = data.get::<GlobalDatabase>().unwrap();

        let add_debt = db.add_debt(&debt, &sender_id);

        match add_debt {
            Ok(_) => {
                format!("{} now owes {} {} spirit dollars", interaction.user.name, user.name, dollas)
            },
            Err(e) => format!("Error: {}", e),
        }
    } else {
        String::from("Something bad happened")
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("iowe")
        .description("Confirm you owe someone that sweet spirit money")
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "The user you owe")
                .required(true),
        ).add_option(
            CreateCommandOption::new(CommandOptionType::Number, "amount", "The amount you owe them")
                .required(true)
        )
}

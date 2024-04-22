use serenity::all::CreateCommand;

pub mod iowe;
pub mod getuserdebt;
pub mod youowe;

pub fn get_commands() -> Vec<CreateCommand> {
    vec![
        iowe::register(),
        getuserdebt::register(),
        youowe::register()
    ]
}

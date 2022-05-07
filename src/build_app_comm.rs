use serenity::{
    builder::{CreateApplicationCommand, CreateApplicationCommandOption},
    model::interactions::application_command::ApplicationCommandOptionType,
};

pub fn build_mfunc_app(
) -> Box<dyn FnOnce(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand + Sync + Send> {
    Box::new(move |command: &mut CreateApplicationCommand| {
        let mut my_options: Vec<CreateApplicationCommandOption> = Vec::new();
        let my_func = |term: &mut CreateApplicationCommandOption, i: i32| {
            if i == 0 {
                return term.name("constant")
                .description("Constant term")
                .kind(ApplicationCommandOptionType::Number)
                .required(true).clone();
            }
            term.name(format!("{}", i))
                .description(format!("Coefficient of x^{}", i))
                .kind(ApplicationCommandOptionType::Number)
                .required(true).clone()
        };
        for i in 0..5 {
            let mut foo = CreateApplicationCommandOption::default();
            my_func(&mut foo, i);
            my_options.push(foo.clone());
        }
        command
            .name("mfunc")
            .description("Math Stuff")
            .set_options(my_options)
    })
}

pub fn build_froot_app(
) -> Box<dyn FnOnce(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand + Sync + Send> {
    Box::new(move |command: &mut CreateApplicationCommand| {
        let mut my_options: Vec<CreateApplicationCommandOption> = Vec::new();
        let my_func = |term: &mut CreateApplicationCommandOption, i: i32| {
            if i == 0 {
                return term.name("constant")
                .description("Constant term")
                .kind(ApplicationCommandOptionType::Number)
                .required(true).clone();
            }
            term.name(format!("{}", i))
                .description(format!("Coefficient of x^{}", i))
                .kind(ApplicationCommandOptionType::Number)
                .required(true).clone()
        };
        for i in 0..5 {
            let mut foo = CreateApplicationCommandOption::default();
            my_func(&mut foo, i);
            my_options.push(foo.clone());
        }
        command
            .name("froot")
            .description("Math Stuff")
            .set_options(my_options)
    })
}

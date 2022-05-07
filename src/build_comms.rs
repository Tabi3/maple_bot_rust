use std::fmt::format;

use serenity::{model::interactions::application_command::{ApplicationCommandInteractionDataOptionValue, ApplicationCommandInteraction}, builder::CreateInteractionResponseData};
use crate::test_module::*;


pub fn build_mfunc_func(
    command: &ApplicationCommandInteraction,
) -> Box<
    dyn for<'b, 'c> Fn(
            &'b mut CreateInteractionResponseData<'c>,
        ) -> &'b mut CreateInteractionResponseData<'c>
        + Sync
        + Send
        + '_,
> {
    let options_vec = &*command
        .data
        .options;
        Box::new(
            move |message: &mut CreateInteractionResponseData| -> &mut CreateInteractionResponseData {
            let mut my_vec: Vec<String>  = Vec::new();
            for i in 0..options_vec.len() {
                if let ApplicationCommandInteractionDataOptionValue::Number(number) =
                    options_vec.get(i).expect("asdf").resolved.as_ref().expect("asdf")
                    {
                        my_vec.push(format!("({}x^{})", number, i))
                    } else {
                        return message.content("error or smthing idk");
                    }
            }
            return message.content(format!("{}", my_vec.join("+")));
        })
}

pub fn build_froot_func(
    command: &ApplicationCommandInteraction,
) -> Box<
    dyn for<'b, 'c> Fn(
            &'b mut CreateInteractionResponseData<'c>,
        ) -> &'b mut CreateInteractionResponseData<'c>
        + Sync
        + Send
        + '_,
> {
    let options_vec = &*command
        .data
        .options;
        Box::new(
            move |message: &mut CreateInteractionResponseData| -> &mut CreateInteractionResponseData {
            let mut my_vec: Vec<f64>  = Vec::new();
            for i in 0..options_vec.len() {
                if let ApplicationCommandInteractionDataOptionValue::Number(number) =
                    options_vec.get(i).expect("asdf").resolved.as_ref().expect("asdf")
                    {
                        my_vec.push(*number)
                    } else {
                        return message.content("error or smthing idk");
                    }
            }
            return message.embed(|e| e.field("Roots", format!("`{:?}`", froot(&f_from_vec(my_vec), 0.1)), false).color(0xAAF0D1));
        })
}

pub fn build_ping_func() -> Box<
    dyn for<'b, 'c> Fn(
            &'b mut CreateInteractionResponseData<'c>,
        ) -> &'b mut CreateInteractionResponseData<'c>
        + Sync
        + Send,
> {
    return Box::new(
        move |message: &mut CreateInteractionResponseData| -> &mut CreateInteractionResponseData {
            message
                .content("Ping Pong")
                .embed(|e| e.field("Ping", "Pong", true).field("Pong", "Ping", true))
        },
    );
}
pub fn build_id_func(
    command: &ApplicationCommandInteraction,
) -> Box<
    dyn for<'b, 'c> Fn(
            &'b mut CreateInteractionResponseData<'c>,
        ) -> &'b mut CreateInteractionResponseData<'c>
        + Sync
        + Send
        + '_,
> {
    let optionsvar = &*command
        .data
        .options
        .get(0)
        .expect("jfsk")
        .resolved
        .as_ref()
        .expect("yes");
    Box::new(
        move |message: &mut CreateInteractionResponseData| -> &mut CreateInteractionResponseData {
            if let ApplicationCommandInteractionDataOptionValue::User(user, _member) =
                &optionsvar.clone()
            {
                message.content(format!("{}'s id is {}", user.tag(), user.id))
            } else {
                message.content("Please provide a valid user".to_string())
            }
        },
    )
}

pub fn build_not_impl_func() -> Box<
    dyn for<'b, 'c> Fn(
            &'b mut CreateInteractionResponseData<'c>,
        ) -> &'b mut CreateInteractionResponseData<'c>
        + Sync
        + Send,
> {
    return Box::new(
        move |message: &mut CreateInteractionResponseData| -> &mut CreateInteractionResponseData {
            message.content("not implemented :(")
        },
    );
}
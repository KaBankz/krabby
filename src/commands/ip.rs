use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub async fn run(_options: &[CommandDataOption]) -> String {
    match get_ip().await {
        Ok(ip) => format!("The server's IP address is {}", ip),
        Err(_) => "Failed to get IP address".to_string(),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ip")
        .description("Returns the server's IP address")
}

async fn get_ip() -> Result<String, Box<dyn std::error::Error>> {
    Ok(reqwest::get("https://ip.me").await?.text().await?)
}

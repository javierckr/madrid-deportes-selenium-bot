use chrono;
use std::env;
use teloxide::requests::{Request, Requester};
use thirtyfour::prelude::*;

fn get_two_days_later_date() -> String {
    let today = chrono::Local::now().date_naive();
    let two_days_later = today + chrono::Duration::days(2);
    two_days_later.format("%d/%m/%Y").to_string()
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let user: String = env::var("USER").expect("USER environment variable not set");
    let password: String = env::var("PASSWORD").expect("PASSWORD environment variable not set");
    let hour: String = env::var("HOUR").expect("HOUR environment variable not set");
    let webdriver_url: String =
        env::var("WEBDRIVER_URL").expect("WEBDRIVER_URL environment variable not set");
    let chat: i64 = env::var("CHAT")
        .expect("CHAT environment variable not set")
        .trim()
        .parse()
        .expect("CHAT environment variable must be a valid i64");
    let token = env::var("TOKEN").expect("TOKEN environment variable not set");
    let bot = teloxide::Bot::new(token);
    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless()?;
    caps.add_arg("--ignore-certificate-errors")?;
    caps.set_browser_option(
        "prefs",
        serde_json::json!({
            "profile.password_manager_leak_detection": false,
        }),
    )?;
    let driver = WebDriver::new(webdriver_url, caps).await?;

    let login_url = "https://deportesweb.madrid.es/DeportesWeb/login";
    let ticketingurl = "https://deportesweb.madrid.es/DeportesWeb/Modulos/VentaServicios/Eventos/AltaEventos?token=5F47B0942AD5C1AEDF2C9763DED7327B6A76610EF308A974322DE773241863D1";

    driver.goto(login_url).await?;
    // Accept cookies
    match driver.find(By::XPath("/html/body/div[1]/div/a")).await {
        Ok(elem) => {
            elem.click().await?;
        }
        Err(_) => {}
    }
    //
    match driver.find(By::XPath("/html/body/form/div[3]/div[2]/div[2]/div/div[2]/section[1]/div[2]/div/div/div[2]/article[1]")).await {
        Ok(elem) => {
            elem.click().await?;
        }
        Err(_) => {
            bot.send_message(
                teloxide::types::ChatId(chat),
                format!("No se encuentra el boton login con pass"),
            )
            .send()
            .await
            .unwrap();
            let screenshot = driver.screenshot_as_png().await?;
            bot.send_photo(
                teloxide::types::ChatId(chat),
                teloxide::types::InputFile::memory(screenshot),
            )
            .send()
            .await
            .unwrap();
        }
    }
    std::thread::sleep(std::time::Duration::from_secs(1));
    let elem_form = driver
        .find(By::Id("ContentFixedSection_uLogin_txtIdentificador"))
        .await?;
    elem_form.click().await?;
    elem_form.send_keys(user).await?;

    let elem_form = driver
        .find(By::Id("ContentFixedSection_uLogin_txtContrasena"))
        .await?;
    elem_form.click().await?;
    elem_form.send_keys(password).await?;

    let submit_button = driver
        .find(By::Id("ContentFixedSection_uLogin_btnLogin"))
        .await?;
    submit_button.click().await?;

    std::thread::sleep(std::time::Duration::from_secs(1));
    driver.goto(ticketingurl).await?;
    std::thread::sleep(std::time::Duration::from_secs(1));

    let desired_date = get_two_days_later_date();
    // use custom attribute data-day to select the date
    let datepicker = driver
        .find(By::XPath(format!("//td[@data-day='{desired_date}']")))
        .await?;
    datepicker.click().await?;

    std::thread::sleep(std::time::Duration::from_secs(1));
    match driver
        .find(By::XPath(format!("//h4[contains(text(),'{hour}')]")))
        .await
    {
        Ok(day) => {
            day.click().await?;
        }
        Err(_) => {
            bot.send_message(
                teloxide::types::ChatId(chat),
                format!(
                    "No se puede seleccionar el dia {} a las {}",
                    desired_date, hour
                ),
            )
            .send()
            .await
            .unwrap();
            let screenshot = driver.screenshot_as_png().await?;
            bot.send_photo(
                teloxide::types::ChatId(chat),
                teloxide::types::InputFile::memory(screenshot),
            )
            .send()
            .await
            .unwrap();
            driver.quit().await?;
            return Ok(());
        }
    }

    std::thread::sleep(std::time::Duration::from_secs(1));

    match driver
        .find(By::Id(
            "ContentFixedSection_uCarritoConfirmar_btnConfirmCart",
        ))
        .await
    {
        Ok(confirm) => {
            confirm.click().await?;
        }
        Err(_) => {
            bot.send_message(
                teloxide::types::ChatId(chat),
                format!(
                    "No se puede seleccionar el dia {} a las {}",
                    desired_date, hour
                ),
            )
            .send()
            .await
            .unwrap();
            let screenshot = driver.screenshot_as_png().await?;
            bot.send_photo(
                teloxide::types::ChatId(chat),
                teloxide::types::InputFile::memory(screenshot),
            )
            .send()
            .await
            .unwrap();
            driver.quit().await?;
            return Ok(());
        }
    }

    std::thread::sleep(std::time::Duration::from_secs(3));

    let screenshot = driver.screenshot_as_png().await?;

    bot.send_photo(
        teloxide::types::ChatId(chat),
        teloxide::types::InputFile::memory(screenshot),
    )
    .send()
    .await
    .unwrap();

    // Close the browser
    driver.quit().await?;

    Ok(())
}

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
    //caps.set_headless()?;
    caps.add_arg("--ignore-certificate-errors")?;
    caps.set_browser_option(
        "prefs",
        serde_json::json!({
            "profile.password_manager_leak_detection": false,
        }),
    )?;
    let driver = WebDriver::new(webdriver_url, caps).await?;

    let login_url = "https://deportesweb.madrid.es/DeportesWeb/login";
    let ticketingurl: String =
        env::var("TICKETING_URL").expect("TICKETING_URL environment variable not set");

    driver.goto(login_url).await?;
    // Accept cookies
    match driver.find(By::XPath("/html/body/div[1]/div/a")).await {
        Ok(elem) => {
            elem.click().await?;
        }
        Err(_) => {}
    }
    //
    match driver.query(By::XPath("/html/body/form/div[3]/div[2]/div[2]/div/div[2]/section[1]/div[2]/div/div/div[2]/article[1]"))
        .wait(
            std::time::Duration::from_secs(10),
            std::time::Duration::from_millis(250),
        )
        .single()
        .await
        {
        Ok(elem) => {
            elem.click_when_ready().await?;
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
    match driver
        .query(By::Id("ContentFixedSection_uLogin_txtIdentificador"))
        .wait(
            std::time::Duration::from_secs(10),
            std::time::Duration::from_millis(250),
        )
        .single()
        .await
    {
        Ok(elem) => {
            elem.click_when_ready().await?;
            elem.send_keys(user).await?;
        }
        Err(_) => {
            bot.send_message(
                teloxide::types::ChatId(chat),
                format!("No se encuentra el campo de usuario"),
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

    match driver
        .query(By::Id("ContentFixedSection_uLogin_txtContrasena"))
        .wait(
            std::time::Duration::from_secs(10),
            std::time::Duration::from_millis(250),
        )
        .single()
        .await
    {
        Ok(elem) => {
            elem.click_when_ready().await?;
            elem.send_keys(password).await?;
        }
        Err(_) => {
            bot.send_message(
                teloxide::types::ChatId(chat),
                format!("No se encuentra el campo de password"),
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

    match driver
        .query(By::Id("ContentFixedSection_uLogin_btnLogin"))
        .wait(
            std::time::Duration::from_secs(10),
            std::time::Duration::from_millis(250),
        )
        .single()
        .await
    {
        Ok(elem) => {
            elem.click_when_ready().await?;
        }
        Err(_) => {
            bot.send_message(
                teloxide::types::ChatId(chat),
                format!("No se encuentra el boton de login"),
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
    // Wait till login
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    driver.goto(ticketingurl).await?;

    let desired_date = get_two_days_later_date();
    // use custom attribute data-day to select the date
    match driver
        .query(By::XPath(format!("//td[@data-day='{desired_date}']")))
        .wait(
            std::time::Duration::from_secs(10),
            std::time::Duration::from_millis(250),
        )
        .single()
        .await
    {
        Ok(date_element) => {
            date_element.click().await?;
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
        Err(_) => {
            bot.send_message(
                teloxide::types::ChatId(chat),
                format!("No se puede seleccionar el dia {}", desired_date),
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

    for calles in 1..=2 {
        match driver
            .query(By::XPath(format!(
                "(//h4[contains(., '{hour}')])[{calles}]"
            )))
            .wait(
                std::time::Duration::from_secs(10),
                std::time::Duration::from_millis(250),
            )
            .single()
            .await
        {
            Ok(hour_element) => {
                hour_element.click().await?;
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                match driver.find(By::Id("uAlert_spnAlertDanger")).await {
                    Ok(alert) => {
                        let alert_text = alert.text().await?;
                        if alert_text.contains("disponible") {
                            println!("Alerta: {} en la calle {}", alert_text, calles);
                        } else {
                            println!("Alerta: {} en la calle {}", alert_text, calles);
                        }
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
            Err(e) => {
                println!("Error al seleccionar la hora: {:?}", e);
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
    }

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
                    "No se puede confirmar el dia {} a las {}",
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

// Используем необходимые внешние зависимости
extern crate reqwest;
use reqwest::blocking::get;

fn fetch_website_content(url: &str) -> Result<String, reqwest::Error> {
    // Отправляем GET-запрос на указанный URL
    let response = get(url)?;

    // Проверяем успешность ответа
    if response.status().is_success() {
        // Читаем ответ в виде строки
        let content = response.text()?;

        // Возвращаем содержимое страницы
        Ok(content)
    } else {
        // Обработка ошибки (например, страница не найдена)
        // Err(reqwest::Error::new(reqwest::StatusCode::NOT_FOUND, "Page not found"))
        let content2 = response.text()?;
        Ok(content2)
    }
}

fn main() {
    // Здесь вызываем функцию fetch_website_content
    match fetch_website_content("https://example.net/a/b.html") {
        Ok(content) => println!("Website content:\n{}", content),
        Err(error) => eprintln!("Error: {}", error),
    }
}

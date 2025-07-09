use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Main function - Основная функция
    top_5_ip();        // Get top 5 IPs - Получить топ 5 IP-адресов
    most_requests_path(); // Get most requested paths - Получить самые запрашиваемые пути
    most_status_codes();  // Get most frequent status codes - Получить самые частые коды статусов
    user_request();       // Get top user agents - Получить топ пользовательских агентов
}

fn read_file() -> std::io::Result<impl Iterator<Item = std::io::Result<String>>> {
    // Read log file - Чтение файла логов
    let log_path = "nginx-access.log.txt"; // Path to log file - Путь к файлу логов

    let file = File::open(log_path)?; // Open file - Открываем файл
    let reader = BufReader::new(file); // Create buffered reader - Создаем буферизированный читатель

    Ok(reader.lines()) // Return line iterator - Возвращаем итератор строк
}

fn top_5_ip() -> std::io::Result<()> {
    // Count IP addresses - Подсчет IP-адресов
    let lines = read_file()?; // Get lines iterator - Получаем итератор строк
    
    let mut ip_counts = HashMap::new(); // Create counting hashmap - Создаем хеш-таблицу для подсчета
    
    for line in lines { // Process each line - Обрабатываем каждую строку
        let line = line?; // Unwrap line - Извлекаем строку из Result
        if let Some(ip) = line.split_whitespace().next() { // Extract IP - Извлекаем IP-адрес
            let counts = ip_counts.entry(ip.to_string()).or_insert(0); // Count IP - Подсчитываем IP
            *counts += 1; // Increment count - Увеличиваем счетчик
        }
    }

    // Sort and get top 5 - Сортировка и получение топ 5
    let mut ip_counts: Vec<_> = ip_counts.into_iter().collect(); // Convert to vector - Преобразуем в вектор
    ip_counts.sort_by(|a, b| b.1.cmp(&a.1)); // Sort descending - Сортируем по убыванию
    let top_5: Vec<_> = ip_counts.into_iter().take(5).collect(); // Take top 5 - Берем первые 5

    println!("TOP 5 IP: ");
    for (ip, count) in top_5 {
        println!("{} - {} requests", ip, count); // Print results - Выводим результаты
    }
    Ok(())
}


fn most_requests_path() -> std::io::Result<()> {
    // Count request paths - Подсчет запрашиваемых путей
    let lines = read_file()?;
    let mut path_counts = HashMap::new();

    for line in lines {
        let line = line?;
        
        match line.char_indices().filter(|&(_, c)| c == '/').nth(2).map(|(idx, _)| idx) {
            // Find path in log line - Находим путь в строке лога
            Some(pos) => {
                let cross_after_slash = &line[pos..];
                if let Some(end) = cross_after_slash.find(" ") {
                    let path = &cross_after_slash[..end];
                    let counts = path_counts.entry(path.to_string()).or_insert(0);
                    *counts += 1;
                };
            }
            None => continue, // Skip if no path found - Пропускаем если путь не найден
        }
    }

    let mut vec_path_counts: Vec<_> = path_counts.into_iter().collect();
    vec_path_counts.sort_by(|a, b| b.1.cmp(&a.1));
    let top_5: Vec<_> = vec_path_counts.into_iter().take(5).collect();

    println!("\n\nTOP 5 PATH: ");
    for (path, count) in top_5 {
        println!("{} - {} requests", path, count);
    }
    Ok(())
}

fn most_status_codes() -> std::io::Result<()> {
    // Count status codes - Подсчет кодов статусов
    let lines = read_file()?;
    let mut codes_counts = HashMap::new();

    for line in lines {
        let line = line?;
        
        match line.char_indices().filter(|&(_, c)| c == '"').nth(1).map(|(idx, _)| idx) {
            // Find status code position - Находим позицию кода статуса
            Some(pos) => {
                let cross_after_mark = &line[pos+2..];
                if let Some(end) = cross_after_mark.find(" ") {
                    let codes = &cross_after_mark[..end];
                    let counts = codes_counts.entry(codes.to_string()).or_insert(0);
                    *counts += 1;
                };
            }
            None => continue,
        }
    }

    let mut vec_codes_counts: Vec<_> = codes_counts.into_iter().collect();
    vec_codes_counts.sort_by(|a, b| b.1.cmp(&a.1));
    let top_5: Vec<_> = vec_codes_counts.into_iter().take(5).collect();

    println!("\n\nTOP 5 CODES: ");
    for (path, count) in top_5 {
        println!("{} - {} requests", path, count);
    }
    Ok(())
}

fn user_request() -> std::io::Result<()> {
    // Count user agents - Подсчет пользовательских агентов
    let lines = read_file()?;
    let mut user_request_counts = HashMap::new();

    for line in lines {
        let line = line?;
        
        match line.char_indices().filter(|&(_, c)| c == '"').nth(4).map(|(idx, _)| idx) {
            // Find user agent position - Находим позицию пользовательского агента
            Some(pos) => {
                let cross_after_mark = &line[pos+1..];
                if let Some(end) = cross_after_mark.find(')') {
                    let user = &cross_after_mark[..end+1];
                    let counts = user_request_counts.entry(user.to_string()).or_insert(0);
                    *counts += 1;
                };
            }
            None => continue,
        }
    }

    let mut vec_user_counts: Vec<_> = user_request_counts.into_iter().collect();
    vec_user_counts.sort_by(|a, b| b.1.cmp(&a.1));
    let top_5: Vec<_> = vec_user_counts.into_iter().take(5).collect();

    println!("\n\nTOP 5 USERS REQUEST: ");
    for (user, count) in top_5 {
        println!("{} - {} requests", user, count);
    }
    Ok(())
}

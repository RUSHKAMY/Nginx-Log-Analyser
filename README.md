https://roadmap.sh/projects/nginx-log-analyser

ENGLISH AND RUSSIA README

####ENGLISH####


Nginx Log Analyzer
This project is a Rust program for analyzing Nginx web server logs. The program provides statistics on the most active IP addresses, requested paths, response codes, and user agents.

Features
The program analyzes the nginx-access.log.txt file and outputs 4 types of statistics:

Top 5 IP addresses - list of IP addresses that made the most requests

Top 5 paths - most frequently requested URL paths

Top 5 response codes - most common HTTP status codes

Top 5 user agents - most active clients (browsers, bots, etc.)

How to Use
Ensure you have Rust installed (https://www.rust-lang.org/tools/install)

Place your Nginx log file in combined format in the same folder named nginx-access.log.txt

Build and run the program:

cargo build --release
cargo run --release
Output Format
The program outputs results in a readable format:

TOP 5 IP:
159.89.185.30 - 120 requests
142.93.136.176 - 115 requests
...

TOP 5 PATH:
/v1-health - 235 requests
/ - 50 requests
...

TOP 5 CODES:
200 - 300 requests
404 - 45 requests
...

TOP 5 USERS REQUEST:
DigitalOcean Uptime Probe 0.22.0 - 150 requests
Mozilla/5.0 - 80 requests
...
Log Requirements
The program expects logs in standard Nginx combined format:

IP - - [date] "METHOD URI HTTP/VERSION" STATUS_CODE RESPONSE_SIZE "REFERER" "USER_AGENT"


####RUSSIA####

Анализатор Nginx логов
Этот проект представляет собой программу на Rust для анализа логов веб-сервера Nginx. Программа предоставляет статистику по самым активным IP-адресам, запрашиваемым путям, кодам ответов и пользовательским агентам.

Функциональность
Программа анализирует файл nginx-access.log.txt и выводит 4 вида статистики:

Топ-5 IP-адресов - список IP-адресов, с которых было сделано больше всего запросов

Топ-5 путей - самые часто запрашиваемые URL-пути

Топ-5 кодов ответов - наиболее часто встречающиеся HTTP-статусы

Топ-5 пользовательских агентов - самые активные клиенты (браузеры, боты и т.д.)

Как использовать
Убедитесь, что у вас установлен Rust (https://www.rust-lang.org/tools/install)

Поместите ваш файл с логами Nginx в формате combined в ту же папку под именем nginx-access.log.txt

Соберите и запустите программу:

cargo build --release
cargo run --release
Формат вывода
Программа выводит результаты в читаемом виде:


TOP 5 IP:
159.89.185.30 - 120 запросов
142.93.136.176 - 115 запросов
...

TOP 5 PATH:
/v1-health - 235 запросов
/ - 50 запросов
...

TOP 5 CODES:
200 - 300 запросов
404 - 45 запросов
...

TOP 5 USERS REQUEST:
DigitalOcean Uptime Probe 0.22.0 - 150 запросов
Mozilla/5.0 - 80 запросов
...
Требования к логам
Программа ожидает логи в стандартном формате Nginx combined:

IP - - [дата] "МЕТОД URI HTTP/ВЕРСИЯ" КОД_СТАТУСА РАЗМЕР_ОТВЕТА "РЕФЕРЕР" "USER_AGENT"

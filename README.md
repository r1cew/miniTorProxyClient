<!DOCTYPE html>
<html lang="ru">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>miniTorProxyClient</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            line-height: 1.6;
            margin: 20px;
        }
        h2 {
            color: #2c3e50;
        }
        ul {
            list-style-type: none;
            padding: 0;
        }
        li {
            margin: 5px 0;
        }
        .warning {
            color: red;
            font-weight: bold;
        }
    </style>
</head>
<body>

<h2>Описание:</h2>
<p>miniTorProxyClient — это приложение, работающее в панели задач Windows, проксирующее трафик через сеть Tor. Приложение обеспечивает безопасное и анонимное соединение.</p>

<h2>Используемые порты:</h2>
<ul>
    <li>9050 — стандартный порт для SOCKS5-прокси Tor.</li>
    <li>8118 — порт для HTTP-прокси (используется для установки в качестве системного прокси).</li>
</ul>

<h2>Пример использования:</h2>
<h3>Установка расширения FoxyProxy в ваш основной браузер:</h3>
<ol>
    <li>Перейдите в магазин расширений вашего браузера.</li>
    <li>Найдите и установите расширение FoxyProxy.</li>
    <li>Создайте новый прокси и укажите:</li>
    <ul>
        <li>IP адрес: 127.0.0.1</li>
        <li>Порт: 9050</li>
        <li>Тип прокси: SOCKS5</li>
    </ul>
</ol>

<h2>Использование как системный прокси:</h2>
<p>Вы можете проксировать весь трафик системы, нажав кнопку <strong>«Использовать как системный прокси»</strong>. В этом случае иконка приложения изменится на желтую. Если вы хотите отменить это действие, нажмите кнопку <strong>«Очистить системный прокси»</strong>.</p>

<h2 class="warning">Если приложение не стартует:</h2>
<p class="warning">Если иконка приложения красная, это означает, что необходимо обновить мосты.</p>
<p>Вы можете получить мосты, написав в Telegram-боту <a href="https://t.me/GetBridgesBot">@GetBridgesBot</a> команду <code>/bridges</code>. Скопируйте сообщение с полученными мостами и в приложении нажмите на кнопку <strong>«Обновить мосты из буфера обмена»</strong>.</p>

</body>
</html>
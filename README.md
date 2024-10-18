# Описание

Домашнее задание №2 по дисциплине "Конфигурационное управление", РТУ МИРЭА, 3 семестр

# Задание

Разработать инструмент командной строки для визуализации графа
зависимостей, включая транзитивные зависимости. Сторонние средства для
получения зависимостей использовать нельзя.
Зависимости определяются по имени пакета `ОС Ubuntu`. Для описания графа зависимостей используется представление `PlantUML`. Визуализатор должен
выводить результат на экран в виде графического изображения графа.
Конфигурационный файл имеет формат `xml` и содержит:

- Путь к программе для визуализации графов;
- Имя анализируемого пакета;
- Максимальная глубина анализа зависимостей.

Все функции визуализатора зависимостей должны быть покрыты тестами.

# Запуск

В директории `src/cfg/` создайте файл `cfg.xml` и скопируйте в него содержимое файла `cfg.example.xml`. Заполните `cfg.xml` в соответствии с вашей конфигурацией (см. ниже).

1. Если вы используете Ubuntu

Из корня проекта в терминале выполните команду:

```bash
cargo run
```

Дождитесь выполнения команды (учтите, чем больше глубина исследования зависимостей, тем существенно дольше выполняется программа).

2. Если вы используете любую другую ОС

Если вы используете отличную от Ubuntu операционную систему, то для успешного запуска вам потребуется установить [Docker](https://docs.docker.com/desktop/),  если он у вас не установлен.

Перед выполнением программы перейдите в `Dockerfile` и в строке с `VOLUME` измените значение в кавычках внутри скобок на указанное в `cfg.xml` значение `<outputpath>`.

Затем перейдите в корень проекта и выполните следующие 2 команды:

```bash
docker build -t depviz .
docker run -v <внешний/путь/к/директории/с/диаграммами/>:<outputpath> depviz
```

Под внешним путем к директории с диаграммами понимается любой желаемый существующий путь в вашей системе. По этому пути будут сохраняться все построенные диаграммы.

# Тестирование

1. Если вы используете Ubuntu

Для запуска тестов в корне проекта выполните:

```bash
cargo test
```

2. Если вы используете любую другую ОС

Выполните следующую команду (если она до этого не была выполнена) в корне проекта:

```bash
docker build -t depviz .
```

Затем запустите контейнер `Docker` в интерактивном режиме

```bash
docker run -it -v <внешний/путь/к/директории/с/диаграммами/>:<outputpath> depviz bash
```

и в открывшемся внутреннем терминале контейнера выполните команду:

```bash
cargo test
```
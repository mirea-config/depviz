FROM ubuntu:latest

# Устанавливаем необходимые зависимости
RUN apt-get update && \
    apt-get install -y openjdk-11-jre plantuml curl build-essential && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . $HOME/.cargo/env && \
    rustup default stable

# Устанавливаем переменные среды
ENV PATH="/root/.cargo/bin:${PATH}"

# Создаём рабочую директорию
WORKDIR /app

# Копируем код проекта в контейнер
COPY . .

# Собираем проект
RUN cargo build --release

# Команда по умолчанию для запуска
CMD ["./target/release/dependency_visualizer"]

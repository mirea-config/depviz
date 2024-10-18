FROM ubuntu:latest

RUN apt-get update && \
    apt-get install -y openjdk-11-jre plantuml curl build-essential && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . $HOME/.cargo/env && \
    rustup default stable

ENV PATH="/root/.cargo/bin:${PATH}"

# монтируем volume, в который будут сохраняться все графики
VOLUME [ "/depviz" ]

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/depviz"]

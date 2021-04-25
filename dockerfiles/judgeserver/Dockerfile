FROM rust:latest

WORKDIR /judgeserver

COPY ./ .

# RUN sudo apt install libseccomp-dev g++ gcc python3 cmake
RUN cd server/judge && cargo build --release --verbose
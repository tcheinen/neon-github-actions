FROM ubuntu:20.04
RUN apt-get update -y; apt-get install curl build-essential -y
RUN curl -fsSL https://deb.nodesource.com/setup_12.x | bash -
RUN apt-get install -y nodejs
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN echo 'source /root/.cargo/env' >> /root/.bashrc

RUN mkdir /wd/
WORKDIR /wd/

ENV PATH="/root/.cargo/bin:${PATH}"

ENTRYPOINT [ "npm", "run", "build" ]
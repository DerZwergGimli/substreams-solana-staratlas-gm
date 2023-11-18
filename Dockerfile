FROM debian

RUN apt-get update
RUN apt-get upgrade -y
RUN apt-get install -y build-essential procps curl file git curl jq
RUN rm -rf /var/lib/apt/lists/*
#
## Install GO
#RUN curl -OL https://golang.org/dl/go1.16.7.linux-amd64.tar.gz
#RUN tar -C /usr/local -xvf go1.16.7.linux-amd64.tar.gz
#ENV PATH=$PATH:/usr/local/go/bin
#RUN go version
#
#
## Install Homebew
#RUN useradd -m -s /bin/zsh linuxbrew && \
#    usermod -aG sudo linuxbrew &&  \
#    mkdir -p /home/linuxbrew/.linuxbrew && \
#    chown -R linuxbrew: /home/linuxbrew/.linuxbrew
#USER linuxbrew
#RUN /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
#USER root
#ENV PATH=$PATH:/home/linuxbrew/.linuxbrew/bin
#
## Install Substreams
#USER linuxbrew
#RUN brew install gcc
#RUN brew install streamingfast/tap/substreams
#USER root
#RUN substreams --version

## Install Substrams Postgress
RUN curl -OL https://github.com/streamingfast/substreams-sink-sql/releases/download/v3.0.5/substreams-sink-sql_linux_x86_64.tar.gz
RUN mkdir /home/substreams-sink-sql
RUN tar -C /home/substreams-sink-sql -xvf substreams-sink-sql_linux_x86_64.tar.gz
ENV PATH=$PATH:/home/substreams-sink-sql
RUN substreams-sink-sql

## Setup files
RUN cd / && curl -OL https://github.com/DerZwergGimli/substreams-solana-staratlas-gm/releases/download/v2.0.0/substreams-staratlas-market-v2.0.0.spkg
COPY /substreams.prod.yaml /substreams.yaml
COPY /schema.sql /
COPY /substreams.wasm /
RUN ls /

COPY /scripts /

RUN chmod +x ./init_substream.sh
RUN chmod +x ./run_substream.sh





FROM debian

RUN apt-get update
RUN apt-get upgrade -y
RUN apt-get install -y build-essential procps curl file git curl
RUN rm -rf /var/lib/apt/lists/*

# Install GO
RUN curl -OL https://golang.org/dl/go1.16.7.linux-amd64.tar.gz
RUN tar -C /usr/local -xvf go1.16.7.linux-amd64.tar.gz
ENV PATH=$PATH:/usr/local/go/bin
RUN go version


# Install Homebew
RUN useradd -m -s /bin/bash linuxbrew && \
    echo 'linuxbrew ALL=(ALL) NOPASSWD:ALL' >>/etc/sudoers
USER linuxbrew
RUN /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
USER root
ENV PATH=$PATH:/home/linuxbrew/.linuxbrew/bin

# Install Substreams
RUN brew install gcc
RUN brew install streamingfast/tap/substreams
RUN substreams --version

# Install Substrams Postgress
RUN curl -OL https://github.com/streamingfast/substreams-sink-sql/releases/download/v3.0.5/substreams-sink-sql_linux_x86_64.tar.gz
RUN mkdir /home/substreams-sink-sql
RUN tar -C /home/substreams-sink-sql -xvf substreams-sink-sql_linux_x86_64.tar.gz
ENV PATH=$PATH:/home/substreams-sink-sql
RUN substreams-sink-sql

# Setup files
RUN mkdir /home/run
RUN cd /home/run && curl -OL https://github.com/DerZwergGimli/substreams-solana-staratlas-gm/releases/download/v2.0.0/substreams-staratlas-market-v2.0.0.spkg
COPY /substreams.prod.yaml /home/run/substreams.yaml
COPY /schema.sql /home/run

RUN ls /home/run





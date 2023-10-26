FROM ubuntu:latest

# Install compatibility system dependencies/packages
RUN apt-get update && apt-get install -yq --no-install-recommends \
    git \
    git-lfs \
    sudo \
    curl \
    build-essential \
    pkg-config \
    ca-certificates \
    libssl-dev \
    nodejs \
    npm

# Create the gitpod user
ENV USER=gitpod
ENV USERID=33333
ENV HOME=/home/${USER}
RUN useradd -l -u ${USERID} -G sudo -md ${HOME} -s /bin/bash -p ${USER} ${USER} \
    && mkdir -p /etc/sudoers.d && echo "%${USER} ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/${USER}

# Select the gitpod user and use a login shell during docker build
USER ${USER}
SHELL [ "bash", "-lic" ]

# Install rust tooling
RUN curl -fsSL https://sh.rustup.rs | sh -s -- -y

# Install tools using cargo and rustup
RUN cargo install sqlx-cli cargo-tree cargo-watch wasm-pack
RUN rustup target add wasm32-unknown-unknown

# Install more packages with apt
RUN sudo apt install -yq shellcheck cowsay

# Setup .bashrc.d loader
RUN mkdir -p ${HOME}/.bashrc.d && echo 'for i in $(ls -A $HOME/.bashrc.d); do source $i; done' >> ${HOME}/.bashrc
COPY --chown=gitpod:gitpod rust/weather.sh $HOME/.bashrc.d/

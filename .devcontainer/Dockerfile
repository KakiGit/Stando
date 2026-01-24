FROM rust:1-bookworm

# Install system dependencies for GTK4 and libadwaita
RUN apt-get update && apt-get install -y \
    pkg-config \
    libgtk-4-dev \
    libadwaita-1-dev \
    libgdk-pixbuf-2.0-dev \
    libglib2.0-dev \
    libpango1.0-dev \
    libcairo2-dev \
    dbus-x11 \
    xauth \
    x11-apps \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

RUN chmod 0777 /tmp

# Create a non-root user
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && apt-get update \
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME \
    && rm -rf /var/lib/apt/lists/*

# Switch to non-root user
USER $USERNAME

# Set working directory
WORKDIR /workspace

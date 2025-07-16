FROM archlinux:latest

# Update packages & install rust
RUN pacman -Syu --noconfirm \
    && pacman -S --noconfirm nodejs yarn base-devel openssl pkgconf \
    && pacman -Scc --noconfirm

# Enable Corepack
RUN corepack enable

# Activate
RUN corepack prepare yarn@4.6.0 --activate

# Setup working folder
WORKDIR /app

# Copy project
COPY . .

# Build release
RUN yarn install

# Share port
EXPOSE 8080

# Run server or test
CMD yarn run start
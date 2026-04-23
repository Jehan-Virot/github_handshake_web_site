# Base image: Node 20 on Debian Bullseye
FROM node:20-slim

# Set working directory
WORKDIR /workspace

# Install system dependencies in a single layer
RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install global Node tools
RUN npm install -g pnpm @vue/cli vite

# Expose dev server port
EXPOSE 5173

# Set default shell
SHELL ["/bin/bash", "-c"]

# Copy package files separately to leverage Docker caching
COPY package.json pnpm-lock.yaml* ./

# Install dependencies with PNPM
RUN pnpm install --force

# Optional: Copy the rest of the source code
COPY /src /workspace/

# Default command (optional, can be overridden in devcontainer.json)
CMD ["pnpm", "run", "dev"]
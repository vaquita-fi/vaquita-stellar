# Docker Development Environment

This project includes a Docker setup that provides an isolated environment for running Make scripts and developing Soroban contracts.

## Quick Start

### Option 1: Using the helper script (Recommended)

```bash
# Start an interactive shell
./docker-run.sh

# Run a specific command
./docker-run.sh "cd contracts/vaquita-pool && make test"
./docker-run.sh "cd contracts/vaquita-pool && make build"
```

### Option 2: Using docker-compose directly

```bash
# Start interactive shell
docker-compose run --rm vaquita-stellar

# Run a specific command
docker-compose run --rm vaquita-stellar bash -c "cd contracts/vaquita-pool && make test"
```

### Option 3: Using docker directly

```bash
# Build the image
docker build -t vaquita-stellar .

# Run interactive shell
docker run -it --rm -v $(pwd):/workspace -w /workspace vaquita-stellar

# Run a specific command
docker run --rm -v $(pwd):/workspace -w /workspace vaquita-stellar bash -c "cd contracts/vaquita-pool && make test"
```

## What's Included

The Docker environment includes:

- **Rust 1.75** with all necessary build tools
- **Stellar CLI** for contract deployment and interaction
- **Make** for running your Make scripts
- **Git** for version control
- **SSL libraries** for secure connections

## Environment Variables

The container will use the `.env` file from your project root. Make sure to set up your environment variables:

```bash
# Copy the example environment file
cp env.example .env

# Edit the .env file with your values
nano .env
```

## Common Commands

Once inside the Docker environment, you can run:

```bash
# Navigate to the contract directory
cd contracts/vaquita-pool

# Run tests
make test

# Build the contract
make build

# Deploy the contract (requires proper .env setup)
make deploy

# Initialize the contract
make initialize

# Format code
make fmt

# Clean build artifacts
make clean
```

## Volume Mounts

The Docker setup includes several volume mounts for optimal performance:

- **Source code**: Your project directory is mounted to `/workspace`
- **Cargo cache**: Cargo registry and git cache are persisted between runs
- **Build artifacts**: Target directory is shared between host and container

## Troubleshooting

### Permission Issues
If you encounter permission issues, make sure the Docker daemon is running and you have proper permissions.

### Build Cache
To clear the build cache:
```bash
docker-compose down -v
docker system prune -f
```

### Environment Variables
Make sure your `.env` file is properly configured with:
- `SOURCE_ACCOUNT`: Your Stellar account
- `USER_ADDRESS`: User address for testing
- `DEPOSIT_ID`: Unique deposit identifier

## Development Workflow

1. **Start the environment**: `./docker-run.sh`
2. **Navigate to contract**: `cd contracts/vaquita-pool`
3. **Make changes**: Edit your Rust code
4. **Test**: `make test`
5. **Build**: `make build`
6. **Deploy**: `make deploy` (if configured)

The environment is designed to be persistent, so you can make changes on your host machine and they'll be reflected immediately in the container.

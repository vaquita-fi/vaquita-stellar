#!/bin/bash

# Script to run commands in the Docker environment
# Usage: ./docker-run.sh [command]
# Example: ./docker-run.sh "cd contracts/vaquita-pool && make test"

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}🐳 Starting Vaquita Stellar Docker Environment${NC}"

# Check if docker-compose is available
if command -v docker-compose &> /dev/null; then
    COMPOSE_CMD="docker-compose"
elif command -v docker &> /dev/null && docker compose version &> /dev/null; then
    COMPOSE_CMD="docker compose"
else
    echo -e "${RED}❌ Neither docker-compose nor 'docker compose' is available${NC}"
    exit 1
fi

# Build the image if it doesn't exist
echo -e "${YELLOW}📦 Building Docker image...${NC}"
$COMPOSE_CMD build

# If no command provided, start interactive shell
if [ $# -eq 0 ]; then
    echo -e "${GREEN}🚀 Starting interactive shell...${NC}"
    echo -e "${YELLOW}💡 You can now run:${NC}"
    echo -e "   cd contracts/vaquita-pool"
    echo -e "   make test"
    echo -e "   make build"
    echo -e "   make deploy"
    echo ""
    $COMPOSE_CMD run --rm vaquita-stellar
else
    echo -e "${GREEN}🚀 Running command: $@${NC}"
    $COMPOSE_CMD run --rm vaquita-stellar bash -c "$@"
fi


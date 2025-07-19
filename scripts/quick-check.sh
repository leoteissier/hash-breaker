#!/bin/bash

# Script de vérification rapide pour hash-breaker
# Usage: ./scripts/quick-check.sh

set -e

echo "⚡ Vérification rapide du projet hash-breaker..."

# Couleurs
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

# Tests essentiels
echo "1. Formatage du code..."
cargo fmt --all -- --check
echo -e "${GREEN}✅ Formatage OK${NC}"

echo "2. Warnings clippy..."
cargo clippy --all-targets --all-features -- -D warnings
echo -e "${GREEN}✅ Clippy OK${NC}"

echo "3. Compilation release..."
cargo build --release
echo -e "${GREEN}✅ Compilation OK${NC}"

echo "4. Tests..."
cargo test
echo -e "${GREEN}✅ Tests OK${NC}"

echo ""
echo -e "${GREEN}🎉 Vérification rapide terminée ! Prêt à commiter.${NC}" 
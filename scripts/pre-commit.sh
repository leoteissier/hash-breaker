#!/bin/bash

# Script de pré-commit pour hash-breaker
# Usage: ./scripts/pre-commit.sh

set -e

echo "🔍 Pré-commit check pour hash-breaker..."
echo "========================================"

# Couleurs
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Fonction pour afficher les résultats
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✅ $2${NC}"
    else
        echo -e "${RED}❌ $2${NC}"
        echo -e "${YELLOW}💡 Corrigez les erreurs avant de commiter${NC}"
        exit 1
    fi
}

# 1. Formatage du code
echo "1. Vérification du formatage..."
cargo fmt --all -- --check
print_status $? "Formatage OK"

# 2. Clippy
echo "2. Vérification avec clippy..."
cargo clippy --all-targets --all-features -- -D warnings
print_status $? "Clippy OK"

# 3. Tests
echo "3. Exécution des tests..."
cargo test
print_status $? "Tests OK"

# 4. Compilation release
echo "4. Compilation release..."
cargo build --release
print_status $? "Compilation OK"

echo ""
echo "========================================"
echo -e "${GREEN}🎉 Pré-commit check réussi !${NC}"
echo ""
echo -e "${YELLOW}💡 Vous pouvez maintenant commiter :${NC}"
echo "  git add ."
echo "  git commit -m \"votre message de commit\""
echo "" 
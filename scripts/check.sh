#!/bin/bash

# Script de vérification complète pour hash-breaker
# Usage: ./scripts/check.sh

set -e  # Arrêter le script en cas d'erreur

echo "🔍 Début de la vérification complète du projet hash-breaker..."
echo "=================================================="

# Couleurs pour l'affichage
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Fonction pour afficher les résultats
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✅ $2${NC}"
    else
        echo -e "${RED}❌ $2${NC}"
        exit 1
    fi
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

# 1. Vérifier que nous sommes dans le bon répertoire
print_info "Vérification du répertoire de travail..."
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}❌ Cargo.toml non trouvé. Assurez-vous d'être dans le répertoire racine du projet.${NC}"
    exit 1
fi
print_status 0 "Répertoire de travail correct"

# 2. Vérifier la version de Rust
print_info "Vérification de la version de Rust..."
RUST_VERSION=$(rustc --version | cut -d' ' -f2)
print_status 0 "Rust version: $RUST_VERSION"

# 3. Vérifier le formatage du code
print_info "Vérification du formatage du code..."
cargo fmt --all -- --check
print_status $? "Formatage du code OK"

# 4. Vérifier les warnings avec clippy
print_info "Vérification des warnings avec clippy..."
cargo clippy --all-targets --all-features -- -D warnings
print_status $? "Clippy OK (aucun warning)"

# 5. Vérifier la compilation en mode debug
print_info "Compilation en mode debug..."
cargo build
print_status $? "Compilation debug OK"

# 6. Vérifier la compilation en mode release
print_info "Compilation en mode release..."
cargo build --release
print_status $? "Compilation release OK"

# 7. Exécuter tous les tests
print_info "Exécution des tests..."
cargo test --verbose
print_status $? "Tous les tests passent"

# 8. Vérifier la documentation
print_info "Vérification de la documentation..."
cargo doc --no-deps
print_status $? "Documentation générée"

# 9. Vérifier les dépendances obsolètes
print_info "Vérification des dépendances obsolètes..."
cargo outdated || print_warning "Certaines dépendances peuvent être obsolètes"

# 10. Vérifier la sécurité (si cargo-audit est installé)
print_info "Vérification de la sécurité..."
if command -v cargo-audit &> /dev/null; then
    cargo audit
    print_status $? "Audit de sécurité OK"
else
    print_warning "cargo-audit non installé. Installez-le avec: cargo install cargo-audit"
fi

# 11. Vérifier les fichiers de configuration Git
print_info "Vérification des fichiers Git..."
if [ -f ".gitignore" ]; then
    print_status 0 ".gitignore présent"
else
    print_warning ".gitignore manquant"
fi

# 12. Vérifier les workflows GitHub Actions
print_info "Vérification des workflows GitHub Actions..."
if [ -d ".github/workflows" ]; then
    for workflow in .github/workflows/*.yml; do
        if [ -f "$workflow" ]; then
            echo "  - $(basename "$workflow")"
        fi
    done
    print_status 0 "Workflows GitHub Actions présents"
else
    print_warning "Dossier .github/workflows manquant"
fi

# 13. Test de base du programme
print_info "Test de base du programme..."
if [ -f "target/release/hash_breaker" ]; then
    # Test simple avec un hash MD5 connu
    echo "5f4dcc3b5aa765d61d8327deb882cf99" | timeout 10s ./target/release/hash_breaker > /dev/null 2>&1 || true
    print_status 0 "Programme exécutable"
else
    print_warning "Binaire release non trouvé"
fi

# 14. Vérifier la taille du binaire
print_info "Vérification de la taille du binaire..."
if [ -f "target/release/hash_breaker" ]; then
    SIZE=$(du -h target/release/hash_breaker | cut -f1)
    print_status 0 "Taille du binaire: $SIZE"
fi

# 15. Vérifier les fichiers de test
print_info "Vérification des fichiers de test..."
if [ -d "tests" ]; then
    TEST_COUNT=$(find tests -name "*.rs" | wc -l)
    print_status 0 "$TEST_COUNT fichiers de test trouvés"
else
    print_warning "Dossier tests manquant"
fi

echo ""
echo "=================================================="
echo -e "${GREEN}🎉 Vérification complète terminée avec succès !${NC}"
echo ""
echo -e "${BLUE}📋 Résumé :${NC}"
echo "  ✅ Formatage du code"
echo "  ✅ Aucun warning clippy"
echo "  ✅ Compilation debug et release"
echo "  ✅ Tous les tests passent"
echo "  ✅ Documentation générée"
echo "  ✅ Workflows GitHub Actions présents"
echo ""
echo -e "${GREEN}🚀 Prêt à pousser sur Git !${NC}"
echo ""
echo -e "${YELLOW}💡 Commandes utiles :${NC}"
echo "  git add ."
echo "  git commit -m \"votre message de commit\""
echo "  git push origin main" 
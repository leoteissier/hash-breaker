# Makefile pour hash-breaker

.PHONY: help check quick-check test build clean format clippy doc install-tools

# Afficher l'aide
help:
	@echo "🔧 Commandes disponibles pour hash-breaker:"
	@echo ""
	@echo "📋 Vérifications:"
	@echo "  make check        - Vérification complète (formatage, clippy, tests, etc.)"
	@echo "  make quick-check  - Vérification rapide (essentiels seulement)"
	@echo "  make test         - Exécuter tous les tests"
	@echo "  make format       - Formater le code"
	@echo "  make clippy       - Vérifier avec clippy"
	@echo ""
	@echo "🔨 Build:"
	@echo "  make build        - Compiler en mode release"
	@echo "  make clean        - Nettoyer les fichiers de build"
	@echo ""
	@echo "📚 Documentation:"
	@echo "  make doc          - Générer la documentation"
	@echo ""
	@echo "🛠️  Outils:"
	@echo "  make install-tools - Installer les outils de développement"

# Vérification complète
check:
	@./scripts/check.sh

# Vérification rapide
quick-check:
	@./scripts/quick-check.sh

# Tests
test:
	cargo test --verbose

# Build release
build:
	cargo build --release

# Nettoyer
clean:
	cargo clean

# Formater le code
format:
	cargo fmt --all

# Vérifier avec clippy
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Documentation
doc:
	cargo doc --no-deps --open

# Installer les outils de développement
install-tools:
	cargo install cargo-audit
	cargo install cargo-outdated
	@echo "✅ Outils installés: cargo-audit, cargo-outdated"

# Pré-commit hook (pour Git)
pre-commit: 
	@./scripts/pre-commit.sh

# Vérification avant push
pre-push: check
	@echo "✅ Pré-push checks passed!" 
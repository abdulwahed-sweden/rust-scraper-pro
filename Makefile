.PHONY: help install build-frontend build-backend build run dev-frontend dev-backend clean test

# Default target
help:
	@echo "Rust Scraper Pro - Build Commands"
	@echo "=================================="
	@echo "make install         - Install all dependencies (Rust + Node)"
	@echo "make build          - Build both frontend and backend"
	@echo "make build-frontend - Build only the React frontend"
	@echo "make build-backend  - Build only the Rust backend"
	@echo "make run            - Run the full-stack application (production)"
	@echo "make dev-frontend   - Run frontend dev server on port 5173"
	@echo "make dev-backend    - Run backend server on port 3000"
	@echo "make clean          - Clean build artifacts"
	@echo "make test           - Run all tests"

# Install dependencies
install:
	@echo "📦 Installing Rust dependencies..."
	cargo fetch
	@echo "📦 Installing Node dependencies..."
	cd frontend && npm install
	@echo "✅ All dependencies installed!"

# Build frontend
build-frontend:
	@echo "🎨 Building React frontend..."
	cd frontend && npm install && npm run build
	@echo "✅ Frontend built successfully at frontend/dist"

# Build backend
build-backend:
	@echo "🦀 Building Rust backend..."
	cargo build --release
	@echo "✅ Backend built successfully at target/release/rust-scraper-pro"

# Build everything
build: build-frontend build-backend
	@echo "✅ Full-stack application built successfully!"
	@echo "Run 'make run' to start the application"

# Run production build
run: build
	@echo "🚀 Starting Rust Scraper Pro..."
	@echo "📡 Server will be available at http://localhost:3000"
	./target/release/rust-scraper-pro

# Development: Run frontend dev server (with hot reload)
dev-frontend:
	@echo "🎨 Starting frontend dev server on http://localhost:5173..."
	@echo "API requests will be proxied to http://localhost:3000"
	cd frontend && npm run dev

# Development: Run backend server
dev-backend:
	@echo "🦀 Starting backend server on http://localhost:3000..."
	@echo "Note: Run 'make build-frontend' first to serve the frontend"
	cargo run

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	rm -rf frontend/dist
	rm -rf frontend/node_modules
	@echo "✅ Clean complete!"

# Run tests
test:
	@echo "🧪 Running Rust tests..."
	cargo test
	@echo "🧪 Running frontend tests..."
	cd frontend && npm run lint
	@echo "✅ All tests passed!"

# Quick development setup
dev-setup: install build-frontend
	@echo "✅ Development environment ready!"
	@echo "Run 'make dev-backend' in one terminal"
	@echo "Run 'make dev-frontend' in another terminal"

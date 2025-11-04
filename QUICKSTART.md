# Quick Start Guide

Get up and running with Rust Scraper Pro in 5 minutes!

## Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs))
- Node.js 18+ ([Install Node.js](https://nodejs.org))

## Installation

```bash
# Clone the repository (if not already cloned)
git clone https://github.com/abdulwahed-sweden/rust-scraper-pro.git
cd rust-scraper-pro

# Install all dependencies
make install
```

## Quick Run (Production Mode)

```bash
# Build everything
make build

# Run the application
make run
```

Open your browser to: **http://localhost:3000**

## Development Mode (With Hot Reload)

Open two terminal windows:

**Terminal 1 - Backend:**
```bash
make dev-backend
```

**Terminal 2 - Frontend:**
```bash
make dev-frontend
```

Then open:
- Frontend dev server: **http://localhost:5173** (with hot reload)
- Backend API: **http://localhost:3000/api/health**

## Available Commands

| Command | Description |
|---------|-------------|
| `make help` | Show all commands |
| `make install` | Install dependencies |
| `make build` | Build frontend + backend |
| `make run` | Run production build |
| `make dev-backend` | Run backend (port 3000) |
| `make dev-frontend` | Run frontend dev server (port 5173) |
| `make clean` | Clean build artifacts |

## Configuration

Create a `.env` file for custom settings:

```bash
cp .env.example .env
```

Edit `.env`:
```env
SERVER_PORT=3000
SERVER_HOST=127.0.0.1
RUST_LOG=info
```

## API Endpoints

Test the API:

```bash
# Health check
curl http://localhost:3000/api/health

# Get scraped data
curl http://localhost:3000/api/data

# Get statistics
curl http://localhost:3000/api/stats
```

## Project Structure

```
rust-scraper-pro/
├── src/              # Rust backend
├── frontend/         # React frontend
├── Makefile          # Build commands
├── .env.example      # Config template
└── INTEGRATION.md    # Full documentation
```

## Troubleshooting

**Frontend not loading?**
```bash
make build-frontend
```

**Port 3000 in use?**
```bash
echo "SERVER_PORT=8080" >> .env
```

**Dependencies issues?**
```bash
make clean
make install
```

## Next Steps

- Read [INTEGRATION.md](./INTEGRATION.md) for detailed documentation
- Explore the API at http://localhost:3000/api/*
- Customize scraping sources in `src/main.rs`
- Configure frontend in `frontend/src/`

## Need Help?

- Check [INTEGRATION.md](./INTEGRATION.md) for detailed docs
- Open an issue on GitHub
- Check logs: `RUST_LOG=debug cargo run`

---

Happy Scraping! 🦀🎨

# Full-Stack Integration Guide

## Overview

This document describes the integration of the React + Vite frontend with the Rust Axum backend, creating a unified full-stack web scraping application.

## Architecture

### Technology Stack

**Backend (Rust)**
- **Framework**: Axum 0.8.6 (high-performance async web framework)
- **Runtime**: Tokio (async runtime)
- **CORS**: tower-http with CORS support
- **Static Files**: tower-http ServeDir for serving React build
- **Environment**: dotenvy for .env configuration
- **Logging**: tracing + tracing-subscriber

**Frontend (React + Vite)**
- **Framework**: React 18.3.1
- **Build Tool**: Vite 5.4.2
- **Styling**: TailwindCSS 3.4.1
- **Icons**: Lucide React
- **Language**: TypeScript 5.5.3

### Server Architecture

The application runs a single Axum server on port 3000 that:
1. Serves API endpoints under `/api/*`
2. Serves static frontend files from `frontend/dist`
3. Provides SPA fallback routing (all non-API routes serve `index.html`)
4. Supports CORS for development (React dev server on port 5173)

## Project Structure

```
rust-scraper-pro/
├── src/
│   ├── main.rs                 # Application entry point with .env support
│   ├── output/
│   │   └── api.rs              # API server with static file serving
│   └── ...
├── frontend/
│   ├── src/                    # React application source
│   ├── dist/                   # Built frontend (gitignored)
│   ├── package.json
│   └── vite.config.ts          # Vite config with API proxy
├── Cargo.toml                  # Rust dependencies
├── Makefile                    # Build automation
├── .env.example                # Environment variables template
└── .gitignore                  # Git ignore rules
```

## Setup & Installation

### Prerequisites

- Rust 1.70+ (install from https://rustup.rs)
- Node.js 18+ and npm
- Make (optional, but recommended)

### Initial Setup

```bash
# Install all dependencies
make install

# Or manually:
cargo fetch
cd frontend && npm install
```

### Environment Configuration

```bash
# Copy the example environment file
cp .env.example .env

# Edit .env with your settings
vim .env
```

Available environment variables:
- `SERVER_PORT`: Server port (default: 3000)
- `SERVER_HOST`: Server host (default: 127.0.0.1)
- `DATABASE_URL`: Database connection string
- `RUST_LOG`: Logging level (info, debug, trace)
- `CACHE_SIZE`: HTML cache size
- `CACHE_TTL_SECONDS`: Cache TTL in seconds

## Build & Run

### Production Build

```bash
# Build everything (frontend + backend)
make build

# Run the production application
make run

# Or manually:
cd frontend && npm run build
cargo build --release
./target/release/rust-scraper-pro
```

The application will be available at: **http://localhost:3000**

### Development Mode

For development with hot reload, run two terminals:

**Terminal 1 - Backend:**
```bash
make dev-backend
# or
cargo run
```

**Terminal 2 - Frontend:**
```bash
make dev-frontend
# or
cd frontend && npm run dev
```

In development mode:
- Backend runs on **http://localhost:3000** (API + static files)
- Frontend dev server runs on **http://localhost:5173** (with hot reload)
- API requests from frontend are proxied to backend via Vite config

## API Endpoints

All API endpoints are prefixed with `/api/`:

### Health & Status
- `GET /api/health` - Health check endpoint
  ```json
  {
    "status": "healthy",
    "service": "rust-scraper-pro",
    "version": "1.0.0"
  }
  ```

### Data Endpoints
- `GET /api/data` - Get scraped data with optional filters
  - Query params: `query`, `source`, `limit`, `offset`, `category`
- `GET /api/search` - Search through scraped data
  - Query params: `query`, `source`, `category`
- `GET /api/sources` - Get list of unique sources
- `GET /api/stats` - Get scraping statistics
- `POST /api/update` - Update scraped data (JSON body)

### Export Endpoints
- `GET /api/export/json` - Export all data as JSON
- `GET /api/export/csv` - Export all data as CSV

## Frontend Features

The React frontend provides:
- **Dashboard**: Overview with statistics and charts
- **Data Table**: Browse and search scraped data
- **Export Panel**: Export data to various formats
- **Settings**: Configure scraping sources and options
- **Scrape Logs**: Real-time scraping status and logs
- **Dark Mode**: Theme switching support
- **Responsive Design**: Mobile-friendly UI

## Key Integration Points

### 1. API Server (src/output/api.rs)

```rust
// CORS configuration for development
let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods([Method::GET, Method::POST, ...])
    .allow_headers(Any);

// Serve static files from frontend/dist
Router::new()
    .merge(api_routes)  // API under /api/*
    .fallback_service(
        ServeDir::new("frontend/dist")
            .append_index_html_on_directories(true)
    )
```

### 2. Vite Configuration (frontend/vite.config.ts)

```typescript
server: {
  port: 5173,
  proxy: {
    '/api': {
      target: 'http://localhost:3000',
      changeOrigin: true,
    },
  },
}
```

### 3. Environment Loading (src/main.rs)

```rust
// Load .env file if exists
dotenvy::dotenv().ok();

// Get port from environment
let port = std::env::var("SERVER_PORT")
    .ok()
    .and_then(|p| p.parse::<u16>().ok())
    .unwrap_or(3000);
```

## Makefile Commands

```bash
make help           # Show all available commands
make install        # Install dependencies
make build          # Build frontend + backend
make build-frontend # Build only frontend
make build-backend  # Build only backend
make run            # Run production build
make dev-frontend   # Run frontend dev server
make dev-backend    # Run backend server
make clean          # Clean build artifacts
make test           # Run tests
make dev-setup      # Quick dev environment setup
```

## Deployment

### Production Deployment Steps

1. **Build the application:**
   ```bash
   make build
   ```

2. **The built artifacts:**
   - Backend binary: `target/release/rust-scraper-pro`
   - Frontend assets: `frontend/dist/` (embedded in server)

3. **Deploy:**
   - Copy the binary and frontend/dist folder to your server
   - Set environment variables (or use .env file)
   - Run: `./rust-scraper-pro`

4. **Docker Deployment** (optional):
   ```dockerfile
   FROM rust:1.70 as backend-builder
   WORKDIR /app
   COPY . .
   RUN cargo build --release

   FROM node:18 as frontend-builder
   WORKDIR /app/frontend
   COPY frontend/package*.json ./
   RUN npm install
   COPY frontend/ .
   RUN npm run build

   FROM debian:bookworm-slim
   RUN apt-get update && apt-get install -y ca-certificates
   COPY --from=backend-builder /app/target/release/rust-scraper-pro /usr/local/bin/
   COPY --from=frontend-builder /app/frontend/dist /app/frontend/dist
   WORKDIR /app
   EXPOSE 3000
   CMD ["rust-scraper-pro"]
   ```

## Troubleshooting

### Frontend Not Loading

**Issue**: API works but no frontend
**Solution**: Build the frontend first
```bash
make build-frontend
```

### CORS Errors in Development

**Issue**: CORS errors when accessing API from dev server
**Solution**: CORS is configured to allow all origins in development. Check that:
1. Backend is running on port 3000
2. Frontend proxy is configured correctly in vite.config.ts

### Port Already in Use

**Issue**: Port 3000 already in use
**Solution**: Change the port in .env:
```bash
echo "SERVER_PORT=8080" >> .env
```

### API Returns 404

**Issue**: API endpoints return 404
**Solution**: Ensure API routes are prefixed with `/api/`:
```javascript
// Correct
fetch('/api/data')

// Incorrect
fetch('/data')
```

## Performance Considerations

1. **Static Assets**: Served directly by Axum with zero-copy when possible
2. **Caching**: HTML cache with configurable size and TTL
3. **CORS**: Optimized for production (restrict origins in production)
4. **Compression**: Consider adding tower-http compression middleware
5. **Rate Limiting**: Implement rate limiting for API endpoints

## Security Recommendations

1. **CORS**: Restrict allowed origins in production
2. **Environment Variables**: Never commit .env file
3. **API Authentication**: Add JWT or session-based auth
4. **Input Validation**: Validate all user inputs
5. **Rate Limiting**: Implement rate limiting on API endpoints
6. **HTTPS**: Use HTTPS in production (consider reverse proxy)

## Future Enhancements

- [ ] WebSocket support for real-time scraping updates
- [ ] Authentication and user management
- [ ] Database integration (PostgreSQL/SQLite)
- [ ] Rate limiting middleware
- [ ] Compression middleware
- [ ] Docker and Docker Compose setup
- [ ] CI/CD pipeline
- [ ] API documentation with OpenAPI/Swagger
- [ ] Frontend E2E tests
- [ ] Backend integration tests

## Resources

- [Axum Documentation](https://docs.rs/axum/)
- [Tower HTTP Documentation](https://docs.rs/tower-http/)
- [Vite Documentation](https://vitejs.dev/)
- [React Documentation](https://react.dev/)
- [TailwindCSS Documentation](https://tailwindcss.com/)

## License

MIT License - See LICENSE file for details

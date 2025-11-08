# Quick Start Guide - Full-Stack Application

This guide will help you run the complete **rust-scraper-pro** web application with React frontend and Rust backend.

---

## Prerequisites

Ensure you have the following installed:
- **Rust** (Edition 2024) - [Install Rust](https://rustup.rs/)
- **Node.js** (v18+) and **npm** - [Install Node.js](https://nodejs.org/)
- **PostgreSQL** (14+) - [Install PostgreSQL](https://www.postgresql.org/download/)

---

## Step 1: Database Setup

### Create the PostgreSQL Database

```bash
# Connect to PostgreSQL
psql -U postgres

# Create database and user
CREATE DATABASE rust_scraper_db;

# Exit psql
\q
```

### Configure Environment Variables

The `.env` file already contains:
```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/rust_scraper_db
```

Adjust the credentials if needed to match your PostgreSQL setup.

---

## Step 2: Install Dependencies

### Install Rust Dependencies
```bash
cargo fetch
```

### Install Frontend Dependencies
```bash
cd frontend
npm install
cd ..
```

Or use the Makefile:
```bash
make install
```

---

## Step 3: Build the Frontend

```bash
cd frontend
npm run build
cd ..
```

Or use the Makefile:
```bash
make build-frontend
```

This creates an optimized production build in `frontend/dist/`

---

## Step 4: Run the Application

### Start the Full-Stack Server

```bash
cargo run
```

Or use the Makefile:
```bash
make dev-backend
```

The server will:
1. Connect to PostgreSQL database
2. Create the `scraped_data` table if needed
3. Scrape initial data from books.toscrape.com
4. Start serving the frontend and API on **http://localhost:3000**

You should see output like:
```
[INFO] Starting Rust Scraper Pro
[INFO] Connected to PostgreSQL database
[INFO] Database schema initialized successfully
[INFO] Scraping from: Books to Scrape
[INFO] Successfully scraped 34 items
[INFO] Serving static files from frontend/dist
[INFO] Starting full-stack server on http://127.0.0.1:3000
[INFO] Frontend available at http://127.0.0.1:3000
[INFO] API endpoints available at http://127.0.0.1:3000/api/*
[INFO] Scraping completed! Data available via API at http://localhost:3000
```

---

## Step 5: Use the Application

### Access the Web Interface
Open your browser and navigate to:
```
http://localhost:3000
```

You will see the **Dashboard** with:
- **Stats Cards**: Total items, items with prices, unique sources, items with content
- **Data Table**: Scraped books with images, titles, prices, categories
- **"New Scrape" Button**: Trigger a new scraping operation

### Trigger a New Scrape

1. Click the **"New Scrape"** button in the top-right
2. Wait for the scraping to complete (button shows "Scraping..." with a spinner)
3. Success message appears: "Successfully scraped X items!"
4. The data table automatically refreshes with new books

### Export Data

- Click **"Export"** button in the data table to download JSON
- Or access:
  - JSON: `http://localhost:3000/api/export/json`
  - CSV: `http://localhost:3000/api/export/csv`

---

## Development Mode (Optional)

For frontend development with **hot reload**:

### Terminal 1 - Backend
```bash
cargo run
```

### Terminal 2 - Frontend Dev Server
```bash
cd frontend
npm run dev
```

Access the dev server at **http://localhost:5173**
- Changes to React components update instantly
- API requests are automatically proxied to `localhost:3000`

---

## API Endpoints

The backend provides the following REST API:

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/health` | GET | Health check |
| `/api/data` | GET | Get scraped data (supports `?limit=N`) |
| `/api/stats` | GET | Get statistics |
| `/api/sources` | GET | List data sources |
| `/api/scrape` | POST | Trigger new scrape |
| `/api/export/json` | GET | Export as JSON |
| `/api/export/csv` | GET | Export as CSV |

### Example API Calls

```bash
# Health check
curl http://localhost:3000/api/health

# Get statistics
curl http://localhost:3000/api/stats

# Get first 10 items
curl "http://localhost:3000/api/data?limit=10"

# Trigger a new scrape
curl -X POST http://localhost:3000/api/scrape

# Get list of sources
curl http://localhost:3000/api/sources
```

---

## Troubleshooting

### Port 3000 Already in Use
```bash
# Find the process
lsof -ti:3000

# Kill it
kill -9 $(lsof -ti:3000)
```

### Database Connection Failed
- Ensure PostgreSQL is running: `brew services list` (macOS) or `systemctl status postgresql` (Linux)
- Check credentials in `.env` file
- Verify database exists: `psql -U postgres -l`

### Frontend Not Loading
- Ensure you ran `npm run build` in the `frontend/` directory
- Check that `frontend/dist/` folder exists
- Rebuild: `cd frontend && npm run build`

### Config File Missing Error
The application requires `config/settings.toml`. It should already exist, but if missing:

```bash
mkdir -p config
# Then check if config/settings.toml exists
ls -la config/
```

---

## What Data is Scraped?

The application scrapes books from **books.toscrape.com**, a website specifically designed for scraping practice.

**Data includes:**
- Book titles
- Prices (in GBP)
- Cover images
- Availability status
- Star ratings
- Categories

This data is:
- Stored in PostgreSQL
- Cached in-memory for fast access
- Displayed in the React frontend
- Exportable as JSON/CSV

---

## Next Steps

1. Explore the dashboard at http://localhost:3000
2. Click "New Scrape" to fetch fresh data
3. Try exporting the data
4. Check the stats cards for real-time metrics
5. View the source code to understand the architecture

For more information, see:
- [FRONTEND_INTEGRATION.md](FRONTEND_INTEGRATION.md) - Complete integration report
- [README.md](README.md) - Project overview and library usage
- [docs/](docs/) - Additional documentation

---

**Enjoy using rust-scraper-pro!** 🚀

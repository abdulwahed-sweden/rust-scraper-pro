#!/bin/bash

# Database Setup Script for Rust Scraper Pro
# This script creates the PostgreSQL database and tables

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration (can be overridden by environment variables)
DB_USER="${DB_USER:-postgres}"
DB_PASSWORD="${DB_PASSWORD:-postgres}"
DB_HOST="${DB_HOST:-localhost}"
DB_PORT="${DB_PORT:-5432}"
DB_NAME="${DB_NAME:-rust_scraper_db}"

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Rust Scraper Pro - Database Setup${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# Function to check if PostgreSQL is running
check_postgres() {
    echo -e "${YELLOW}Checking if PostgreSQL is running...${NC}"
    if pg_isready -h "$DB_HOST" -p "$DB_PORT" > /dev/null 2>&1; then
        echo -e "${GREEN}✓ PostgreSQL is running${NC}"
        return 0
    else
        echo -e "${RED}✗ PostgreSQL is not running${NC}"
        echo -e "${YELLOW}Please start PostgreSQL first:${NC}"
        echo "  macOS:   brew services start postgresql"
        echo "  Linux:   sudo systemctl start postgresql"
        echo "  Docker:  docker run -d -p 5432:5432 -e POSTGRES_PASSWORD=postgres postgres"
        return 1
    fi
}

# Function to create database
create_database() {
    echo ""
    echo -e "${YELLOW}Creating database '$DB_NAME'...${NC}"

    # Check if database already exists
    if PGPASSWORD=$DB_PASSWORD psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -lqt | cut -d \| -f 1 | grep -qw "$DB_NAME"; then
        echo -e "${YELLOW}⚠ Database '$DB_NAME' already exists${NC}"
        read -p "Do you want to drop and recreate it? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            PGPASSWORD=$DB_PASSWORD psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -c "DROP DATABASE IF EXISTS $DB_NAME;" 2>/dev/null || true
            echo -e "${GREEN}✓ Dropped existing database${NC}"
        else
            echo -e "${YELLOW}Skipping database creation${NC}"
            return 0
        fi
    fi

    # Create database
    PGPASSWORD=$DB_PASSWORD psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -c "CREATE DATABASE $DB_NAME;" 2>&1
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Database '$DB_NAME' created successfully${NC}"
    else
        echo -e "${RED}✗ Failed to create database${NC}"
        return 1
    fi
}

# Function to create tables
create_tables() {
    echo ""
    echo -e "${YELLOW}Creating tables...${NC}"

    PGPASSWORD=$DB_PASSWORD psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" <<'SQL'
-- Create scraped_data table
CREATE TABLE IF NOT EXISTS scraped_data (
    id VARCHAR(255) PRIMARY KEY,
    source VARCHAR(255) NOT NULL,
    url TEXT NOT NULL,
    title TEXT,
    content TEXT,
    price DECIMAL(10,2),
    image_url TEXT,
    author VARCHAR(255),
    timestamp TIMESTAMPTZ NOT NULL,
    category VARCHAR(255),
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_scraped_data_source ON scraped_data(source);
CREATE INDEX IF NOT EXISTS idx_scraped_data_timestamp ON scraped_data(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_scraped_data_created_at ON scraped_data(created_at DESC);

-- Create GIN index for JSONB metadata
CREATE INDEX IF NOT EXISTS idx_scraped_data_metadata ON scraped_data USING GIN (metadata);

-- Create full-text search index (optional but recommended)
CREATE INDEX IF NOT EXISTS idx_scraped_data_fulltext ON scraped_data
USING GIN (to_tsvector('english', COALESCE(title, '') || ' ' || COALESCE(content, '')));

SQL

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Tables and indexes created successfully${NC}"
    else
        echo -e "${RED}✗ Failed to create tables${NC}"
        return 1
    fi
}

# Function to verify setup
verify_setup() {
    echo ""
    echo -e "${YELLOW}Verifying database setup...${NC}"

    # Check tables
    TABLE_COUNT=$(PGPASSWORD=$DB_PASSWORD psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -t -c "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public' AND table_name = 'scraped_data';" 2>/dev/null | xargs)

    if [ "$TABLE_COUNT" -eq "1" ]; then
        echo -e "${GREEN}✓ Table 'scraped_data' exists${NC}"
    else
        echo -e "${RED}✗ Table 'scraped_data' not found${NC}"
        return 1
    fi

    # Check indexes
    INDEX_COUNT=$(PGPASSWORD=$DB_PASSWORD psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -t -c "SELECT COUNT(*) FROM pg_indexes WHERE tablename = 'scraped_data';" 2>/dev/null | xargs)

    echo -e "${GREEN}✓ Found $INDEX_COUNT indexes${NC}"

    # Show table structure
    echo ""
    echo -e "${YELLOW}Table structure:${NC}"
    PGPASSWORD=$DB_PASSWORD psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -c "\d scraped_data" 2>/dev/null
}

# Function to create .env file
create_env_file() {
    echo ""
    echo -e "${YELLOW}Creating .env file...${NC}"

    ENV_FILE="../.env"
    CONNECTION_STRING="postgres://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME"

    if [ -f "$ENV_FILE" ]; then
        echo -e "${YELLOW}⚠ .env file already exists${NC}"
        read -p "Do you want to update DATABASE_URL? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            # Update or add DATABASE_URL
            if grep -q "^DATABASE_URL=" "$ENV_FILE"; then
                # macOS and Linux compatible sed
                if [[ "$OSTYPE" == "darwin"* ]]; then
                    sed -i '' "s|^DATABASE_URL=.*|DATABASE_URL=$CONNECTION_STRING|" "$ENV_FILE"
                else
                    sed -i "s|^DATABASE_URL=.*|DATABASE_URL=$CONNECTION_STRING|" "$ENV_FILE"
                fi
                echo -e "${GREEN}✓ Updated DATABASE_URL in .env${NC}"
            else
                echo "DATABASE_URL=$CONNECTION_STRING" >> "$ENV_FILE"
                echo -e "${GREEN}✓ Added DATABASE_URL to .env${NC}"
            fi
        fi
    else
        # Create new .env from template
        if [ -f "../.env.example" ]; then
            cp "../.env.example" "$ENV_FILE"
            echo -e "${GREEN}✓ Created .env from template${NC}"

            # Update DATABASE_URL
            if [[ "$OSTYPE" == "darwin"* ]]; then
                sed -i '' "s|^DATABASE_URL=.*|DATABASE_URL=$CONNECTION_STRING|" "$ENV_FILE"
            else
                sed -i "s|^DATABASE_URL=.*|DATABASE_URL=$CONNECTION_STRING|" "$ENV_FILE"
            fi
            echo -e "${GREEN}✓ Updated DATABASE_URL in .env${NC}"
        else
            echo -e "${YELLOW}⚠ .env.example not found, creating minimal .env${NC}"
            cat > "$ENV_FILE" <<EOF
DATABASE_URL=$CONNECTION_STRING
SERVER_PORT=3000
RUST_LOG=info,rust_scraper_pro=debug
EOF
            echo -e "${GREEN}✓ Created .env file${NC}"
        fi
    fi
}

# Function to test connection
test_connection() {
    echo ""
    echo -e "${YELLOW}Testing database connection...${NC}"

    CONNECTION_STRING="postgres://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME"

    # Test query
    RESULT=$(PGPASSWORD=$DB_PASSWORD psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -t -c "SELECT 'Connection successful!' as status;" 2>/dev/null | xargs)

    if [ "$RESULT" == "Connection successful!" ]; then
        echo -e "${GREEN}✓ Database connection test passed${NC}"
        echo -e "${GREEN}✓ Connection string: $CONNECTION_STRING${NC}"
    else
        echo -e "${RED}✗ Database connection test failed${NC}"
        return 1
    fi
}

# Main execution
main() {
    echo "Database Configuration:"
    echo "  Host:     $DB_HOST:$DB_PORT"
    echo "  User:     $DB_USER"
    echo "  Database: $DB_NAME"
    echo ""

    # Run setup steps
    if ! check_postgres; then
        exit 1
    fi

    if ! create_database; then
        exit 1
    fi

    if ! create_tables; then
        exit 1
    fi

    if ! verify_setup; then
        exit 1
    fi

    create_env_file

    if ! test_connection; then
        exit 1
    fi

    echo ""
    echo -e "${GREEN}========================================${NC}"
    echo -e "${GREEN}✓ Database setup completed successfully!${NC}"
    echo -e "${GREEN}========================================${NC}"
    echo ""
    echo -e "${YELLOW}Next steps:${NC}"
    echo "  1. Review the .env file"
    echo "  2. Run the application: cargo run"
    echo "  3. Check the API: curl http://localhost:3000/api/health"
    echo ""
    echo -e "${YELLOW}Database connection string:${NC}"
    echo "  postgres://$DB_USER:***@$DB_HOST:$DB_PORT/$DB_NAME"
    echo ""
}

# Run main function
main

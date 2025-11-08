import { useState, useEffect } from 'react';
import { Activity, Database, TrendingUp, Zap, RefreshCw } from 'lucide-react';
import { StatsCard } from '../components/StatsCard';
import { DataTable } from '../components/DataTable';

const API_BASE_URL = 'http://localhost:3000';

interface ScrapedData {
  id: string;
  source: string;
  url: string;
  title: string | null;
  content: string | null;
  price: number | null;
  image_url: string | null;
  author: string | null;
  timestamp: string;
  category: string | null;
}

interface Stats {
  total_items: number;
  unique_sources: number;
  items_with_content: number;
  items_with_price: number;
}

export function Dashboard() {
  const [data, setData] = useState<ScrapedData[]>([]);
  const [stats, setStats] = useState<Stats | null>(null);
  const [loading, setLoading] = useState(true);
  const [scraping, setScraping] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [successMessage, setSuccessMessage] = useState<string | null>(null);

  const fetchData = async () => {
    try {
      setLoading(true);
      setError(null);

      const [dataResponse, statsResponse] = await Promise.all([
        fetch(`${API_BASE_URL}/api/data?limit=100`),
        fetch(`${API_BASE_URL}/api/stats`),
      ]);

      if (!dataResponse.ok || !statsResponse.ok) {
        throw new Error('Failed to fetch data from API');
      }

      const dataJson = await dataResponse.json();
      const statsJson = await statsResponse.json();

      setData(dataJson);
      setStats(statsJson);
    } catch (err) {
      console.error('Error fetching data:', err);
      setError('Failed to load data. Make sure the backend server is running.');
    } finally {
      setLoading(false);
    }
  };

  const triggerScrape = async () => {
    try {
      setScraping(true);
      setError(null);
      setSuccessMessage(null);

      const response = await fetch(`${API_BASE_URL}/api/scrape`, {
        method: 'POST',
      });

      if (!response.ok) {
        throw new Error('Failed to trigger scrape');
      }

      const result = await response.json();
      setSuccessMessage(`Successfully scraped ${result.items_scraped} items!`);

      // Refresh data after scraping
      await fetchData();
    } catch (err) {
      console.error('Error triggering scrape:', err);
      setError('Failed to trigger scrape. Please try again.');
    } finally {
      setScraping(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-display font-bold text-text-primary dark:text-text-dark mb-2">
            Dashboard Overview
          </h2>
          <p className="text-text-secondary">
            Monitor your scraping operations and data flow in real-time
          </p>
        </div>

        <button
          onClick={triggerScrape}
          disabled={scraping}
          className="btn-primary flex items-center gap-2 px-6 py-3 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <RefreshCw className={`w-5 h-5 ${scraping ? 'animate-spin' : ''}`} />
          {scraping ? 'Scraping...' : 'New Scrape'}
        </button>
      </div>

      {error && (
        <div className="bg-red-100 dark:bg-red-900/30 border border-red-400 dark:border-red-600 text-red-700 dark:text-red-400 px-4 py-3 rounded">
          {error}
        </div>
      )}

      {successMessage && (
        <div className="bg-green-100 dark:bg-green-900/30 border border-green-400 dark:border-green-600 text-green-700 dark:text-green-400 px-4 py-3 rounded">
          {successMessage}
        </div>
      )}

      {loading ? (
        <div className="text-center py-12">
          <div className="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
          <p className="mt-4 text-text-secondary">Loading data...</p>
        </div>
      ) : (
        <>
          <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
            <StatsCard
              title="Total Items"
              value={stats?.total_items.toLocaleString() || '0'}
              icon={Activity}
            />
            <StatsCard
              title="With Prices"
              value={stats?.items_with_price.toLocaleString() || '0'}
              icon={Database}
            />
            <StatsCard
              title="Unique Sources"
              value={stats?.unique_sources.toString() || '0'}
              icon={Zap}
            />
            <StatsCard
              title="With Content"
              value={stats?.items_with_content.toLocaleString() || '0'}
              icon={TrendingUp}
            />
          </div>

          <DataTable data={data} onRefresh={fetchData} />
        </>
      )}
    </div>
  );
}

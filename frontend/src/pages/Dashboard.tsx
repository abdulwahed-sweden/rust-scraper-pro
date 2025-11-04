import { Activity, Database, TrendingUp, Zap } from 'lucide-react';
import { StatsCard } from '../components/StatsCard';
import { ChartCard } from '../components/ChartCard';
import { ScrapeStatusCard } from '../components/ScrapeStatusCard';
import { DataTable } from '../components/DataTable';
import { ScrapeLog } from '../components/ScrapeLog';

export function Dashboard() {
  const mockJobs = [
    {
      id: '1',
      source: 'example.com/products',
      status: 'running' as const,
      timestamp: '2 minutes ago',
      itemsScraped: 1243,
    },
    {
      id: '2',
      source: 'api.data-source.io/items',
      status: 'completed' as const,
      timestamp: '15 minutes ago',
      itemsScraped: 5678,
    },
    {
      id: '3',
      source: 'news-site.com/articles',
      status: 'pending' as const,
      timestamp: 'Scheduled for 3:00 PM',
    },
  ];

  const mockData = [
    {
      id: '1',
      title: 'Product ABC - Premium Edition',
      url: 'https://example.com/products/abc-premium',
      timestamp: '2024-01-15 14:32:15',
      status: 'scraped',
    },
    {
      id: '2',
      title: 'Data Entry XYZ - Analytics Report',
      url: 'https://api.data-source.io/items/xyz-analytics',
      timestamp: '2024-01-15 14:28:42',
      status: 'scraped',
    },
    {
      id: '3',
      title: 'Article: Breaking Tech News',
      url: 'https://news-site.com/articles/breaking-tech',
      timestamp: '2024-01-15 14:15:03',
      status: 'scraped',
    },
  ];

  const mockLogs = [
    {
      id: '1',
      level: 'success' as const,
      message: 'Successfully scraped 5,678 items from api.data-source.io',
      timestamp: '14:28:42',
    },
    {
      id: '2',
      level: 'info' as const,
      message: 'Started scraping example.com/products',
      timestamp: '14:30:15',
    },
    {
      id: '3',
      level: 'warning' as const,
      message: 'Rate limit approaching for news-site.com',
      timestamp: '14:25:08',
    },
    {
      id: '4',
      level: 'error' as const,
      message: 'Connection timeout to legacy-api.com',
      timestamp: '14:22:33',
    },
  ];

  const chartData = [
    { label: 'Monday', value: 245 },
    { label: 'Tuesday', value: 389 },
    { label: 'Wednesday', value: 521 },
    { label: 'Thursday', value: 412 },
    { label: 'Friday', value: 678 },
    { label: 'Saturday', value: 334 },
    { label: 'Sunday', value: 198 },
  ];

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-display font-bold text-text-primary dark:text-text-dark mb-2">
          Dashboard Overview
        </h2>
        <p className="text-text-secondary">
          Monitor your scraping operations and data flow in real-time
        </p>
      </div>

      <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
        <StatsCard
          title="Total Scrapes"
          value="12,847"
          icon={Activity}
          trend={{ value: '12%', positive: true }}
        />
        <StatsCard
          title="Data Collected"
          value="2.4 GB"
          icon={Database}
          trend={{ value: '8%', positive: true }}
        />
        <StatsCard
          title="Active Sources"
          value="24"
          icon={Zap}
          subtitle="3 running now"
        />
        <StatsCard
          title="Success Rate"
          value="98.3%"
          icon={TrendingUp}
          trend={{ value: '2.1%', positive: true }}
        />
      </div>

      <div className="grid lg:grid-cols-3 gap-6">
        <div className="lg:col-span-2">
          <ScrapeStatusCard jobs={mockJobs} />
        </div>
        <ChartCard title="Weekly Scrape Activity" data={chartData} />
      </div>

      <DataTable data={mockData} />

      <ScrapeLog logs={mockLogs} />
    </div>
  );
}

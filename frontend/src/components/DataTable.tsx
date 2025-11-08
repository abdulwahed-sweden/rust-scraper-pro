import { Download, ExternalLink, RefreshCw } from 'lucide-react';

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

interface DataTableProps {
  data: ScrapedData[];
  onRefresh?: () => void;
}

export function DataTable({ data, onRefresh }: DataTableProps) {
  const formatTimestamp = (timestamp: string) => {
    try {
      const date = new Date(timestamp);
      return date.toLocaleString('en-US', {
        month: 'short',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit',
      });
    } catch {
      return timestamp;
    }
  };

  const handleExport = () => {
    window.open('http://localhost:3000/api/export/json', '_blank');
  };

  return (
    <div className="card">
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-lg font-heading font-semibold text-text-primary dark:text-text-dark">
          Scraped Books Data
        </h3>
        <div className="flex items-center gap-2">
          {onRefresh && (
            <button onClick={onRefresh} className="btn-ghost text-sm flex items-center gap-2">
              <RefreshCw className="w-4 h-4" />
              Refresh
            </button>
          )}
          <button onClick={handleExport} className="btn-ghost text-sm flex items-center gap-2">
            <Download className="w-4 h-4" />
            Export
          </button>
        </div>
      </div>

      <div className="overflow-x-auto">
        <table className="w-full">
          <thead>
            <tr className="border-b border-border-light dark:border-border-dark">
              <th className="text-left py-3 px-4 text-sm font-semibold text-text-secondary">Title</th>
              <th className="text-left py-3 px-4 text-sm font-semibold text-text-secondary">Price</th>
              <th className="text-left py-3 px-4 text-sm font-semibold text-text-secondary">Category</th>
              <th className="text-left py-3 px-4 text-sm font-semibold text-text-secondary">Source</th>
              <th className="text-left py-3 px-4 text-sm font-semibold text-text-secondary">Timestamp</th>
              <th className="text-right py-3 px-4 text-sm font-semibold text-text-secondary">Link</th>
            </tr>
          </thead>
          <tbody>
            {data.length === 0 ? (
              <tr>
                <td colSpan={6} className="text-center py-8 text-text-secondary">
                  No data available. Click "New Scrape" to fetch books from books.toscrape.com
                </td>
              </tr>
            ) : (
              data.map((row) => (
                <tr
                  key={row.id}
                  className="border-b border-border-light dark:border-border-dark hover:bg-background-lighter dark:hover:bg-background-darker transition-colors"
                >
                  <td className="py-3 px-4">
                    <div className="flex items-center gap-3">
                      {row.image_url && (
                        <img
                          src={row.image_url}
                          alt={row.title || 'Book'}
                          className="w-10 h-14 object-cover rounded"
                        />
                      )}
                      <span className="font-medium text-text-primary dark:text-text-dark">
                        {row.title || 'Untitled'}
                      </span>
                    </div>
                  </td>
                  <td className="py-3 px-4">
                    {row.price ? (
                      <span className="font-semibold text-primary">£{row.price.toFixed(2)}</span>
                    ) : (
                      <span className="text-text-secondary text-sm">N/A</span>
                    )}
                  </td>
                  <td className="py-3 px-4">
                    {row.category && (
                      <span className="px-2.5 py-0.5 rounded-full text-xs font-medium bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-400">
                        {row.category}
                      </span>
                    )}
                  </td>
                  <td className="py-3 px-4">
                    <span className="text-sm text-text-secondary">{row.source}</span>
                  </td>
                  <td className="py-3 px-4">
                    <span className="text-sm text-text-secondary">{formatTimestamp(row.timestamp)}</span>
                  </td>
                  <td className="py-3 px-4 text-right">
                    <a
                      href={row.url}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="inline-flex items-center gap-1 text-primary hover:text-primary-600 transition-colors"
                    >
                      <ExternalLink className="w-4 h-4" />
                    </a>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}

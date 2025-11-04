import { Download, ExternalLink } from 'lucide-react';

interface DataRow {
  id: string;
  title: string;
  url: string;
  timestamp: string;
  status: string;
}

interface DataTableProps {
  data: DataRow[];
  onExport?: () => void;
}

export function DataTable({ data, onExport }: DataTableProps) {
  return (
    <div className="card">
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-lg font-heading font-semibold text-text-primary dark:text-text-dark">
          Recent Scraped Data
        </h3>
        <button onClick={onExport} className="btn-ghost text-sm flex items-center gap-2">
          <Download className="w-4 h-4" />
          Export
        </button>
      </div>

      <div className="overflow-x-auto">
        <table className="w-full">
          <thead>
            <tr className="border-b border-border-light dark:border-border-dark">
              <th className="text-left py-3 px-4 text-sm font-semibold text-text-secondary">Title</th>
              <th className="text-left py-3 px-4 text-sm font-semibold text-text-secondary">URL</th>
              <th className="text-left py-3 px-4 text-sm font-semibold text-text-secondary">Timestamp</th>
              <th className="text-left py-3 px-4 text-sm font-semibold text-text-secondary">Status</th>
              <th className="text-right py-3 px-4 text-sm font-semibold text-text-secondary">Actions</th>
            </tr>
          </thead>
          <tbody>
            {data.length === 0 ? (
              <tr>
                <td colSpan={5} className="text-center py-8 text-text-secondary">
                  No data available
                </td>
              </tr>
            ) : (
              data.map((row) => (
                <tr
                  key={row.id}
                  className="border-b border-border-light dark:border-border-dark hover:bg-background-lighter dark:hover:bg-background-darker transition-colors"
                >
                  <td className="py-3 px-4">
                    <span className="font-medium text-text-primary dark:text-text-dark">
                      {row.title}
                    </span>
                  </td>
                  <td className="py-3 px-4">
                    <code className="text-xs text-text-secondary">
                      {row.url.length > 40 ? `${row.url.slice(0, 40)}...` : row.url}
                    </code>
                  </td>
                  <td className="py-3 px-4">
                    <span className="text-sm text-text-secondary">{row.timestamp}</span>
                  </td>
                  <td className="py-3 px-4">
                    <span className="px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400">
                      {row.status}
                    </span>
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

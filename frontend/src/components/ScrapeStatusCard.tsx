import { Activity, CheckCircle, XCircle, Clock } from 'lucide-react';

interface ScrapeJob {
  id: string;
  source: string;
  status: 'running' | 'completed' | 'failed' | 'pending';
  timestamp: string;
  itemsScraped?: number;
}

interface ScrapeStatusCardProps {
  jobs: ScrapeJob[];
}

export function ScrapeStatusCard({ jobs }: ScrapeStatusCardProps) {
  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'running':
        return <Activity className="w-4 h-4 text-primary animate-pulse" />;
      case 'completed':
        return <CheckCircle className="w-4 h-4 text-green-500" />;
      case 'failed':
        return <XCircle className="w-4 h-4 text-red-500" />;
      case 'pending':
        return <Clock className="w-4 h-4 text-secondary" />;
      default:
        return null;
    }
  };

  const getStatusBadge = (status: string) => {
    const baseClasses = 'px-2.5 py-0.5 rounded-full text-xs font-medium';
    switch (status) {
      case 'running':
        return `${baseClasses} bg-primary/10 text-primary`;
      case 'completed':
        return `${baseClasses} bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400`;
      case 'failed':
        return `${baseClasses} bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-400`;
      case 'pending':
        return `${baseClasses} bg-secondary/10 text-secondary-700`;
      default:
        return baseClasses;
    }
  };

  return (
    <div className="card">
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-lg font-heading font-semibold text-text-primary dark:text-text-dark">
          Active Scrapes
        </h3>
        <button className="btn-primary text-sm">
          New Scrape
        </button>
      </div>

      <div className="space-y-3">
        {jobs.length === 0 ? (
          <p className="text-center text-text-secondary py-8">No active scrapes</p>
        ) : (
          jobs.map((job) => (
            <div
              key={job.id}
              className="flex items-center justify-between p-4 bg-background-lighter dark:bg-background-darker rounded-lg hover:shadow-md transition-shadow duration-200"
            >
              <div className="flex items-center gap-3 flex-1">
                {getStatusIcon(job.status)}
                <div className="flex-1">
                  <p className="font-medium text-text-primary dark:text-text-dark">
                    {job.source}
                  </p>
                  <p className="text-xs text-text-secondary mt-0.5">
                    {job.timestamp}
                    {job.itemsScraped !== undefined && ` • ${job.itemsScraped} items`}
                  </p>
                </div>
              </div>
              <span className={getStatusBadge(job.status)}>
                {job.status}
              </span>
            </div>
          ))
        )}
      </div>
    </div>
  );
}

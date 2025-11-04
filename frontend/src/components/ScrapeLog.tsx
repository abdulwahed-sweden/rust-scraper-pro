import { Info, AlertCircle, CheckCircle, AlertTriangle } from 'lucide-react';

interface LogEntry {
  id: string;
  level: 'info' | 'warning' | 'error' | 'success';
  message: string;
  timestamp: string;
}

interface ScrapeLogProps {
  logs: LogEntry[];
}

export function ScrapeLog({ logs }: ScrapeLogProps) {
  const getLogIcon = (level: string) => {
    switch (level) {
      case 'info':
        return <Info className="w-4 h-4 text-blue-500" />;
      case 'warning':
        return <AlertTriangle className="w-4 h-4 text-secondary" />;
      case 'error':
        return <AlertCircle className="w-4 h-4 text-red-500" />;
      case 'success':
        return <CheckCircle className="w-4 h-4 text-green-500" />;
      default:
        return null;
    }
  };

  const getLogColor = (level: string) => {
    switch (level) {
      case 'info':
        return 'text-blue-600 dark:text-blue-400';
      case 'warning':
        return 'text-secondary-700 dark:text-secondary';
      case 'error':
        return 'text-red-600 dark:text-red-400';
      case 'success':
        return 'text-green-600 dark:text-green-400';
      default:
        return 'text-text-secondary';
    }
  };

  return (
    <div className="card">
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-lg font-heading font-semibold text-text-primary dark:text-text-dark">
          System Logs
        </h3>
        <button className="text-sm text-text-secondary hover:text-text-primary dark:hover:text-text-dark transition-colors">
          Clear
        </button>
      </div>

      <div className="space-y-2 max-h-96 overflow-y-auto">
        {logs.length === 0 ? (
          <p className="text-center text-text-secondary py-8">No logs available</p>
        ) : (
          logs.map((log) => (
            <div
              key={log.id}
              className="flex items-start gap-3 p-3 bg-background-lighter dark:bg-background-darker rounded-lg"
            >
              {getLogIcon(log.level)}
              <div className="flex-1 min-w-0">
                <p className={`text-sm font-medium ${getLogColor(log.level)}`}>
                  {log.message}
                </p>
                <p className="text-xs text-text-secondary mt-1 font-mono">{log.timestamp}</p>
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
}

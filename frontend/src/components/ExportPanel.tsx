import { Download, FileJson, FileText, Database } from 'lucide-react';

export function ExportPanel() {
  const exportFormats = [
    {
      id: 'json',
      name: 'JSON',
      description: 'Export as JSON file',
      icon: FileJson,
      color: 'text-primary',
    },
    {
      id: 'csv',
      name: 'CSV',
      description: 'Export as CSV spreadsheet',
      icon: FileText,
      color: 'text-secondary',
    },
    {
      id: 'postgresql',
      name: 'PostgreSQL',
      description: 'Direct export to database',
      icon: Database,
      color: 'text-green-600',
    },
  ];

  return (
    <div className="space-y-6">
      <div className="card">
        <h3 className="text-lg font-heading font-semibold text-text-primary dark:text-text-dark mb-6">
          Export Formats
        </h3>

        <div className="grid md:grid-cols-3 gap-4">
          {exportFormats.map((format) => {
            const Icon = format.icon;
            return (
              <button
                key={format.id}
                className="p-6 bg-background-lighter dark:bg-background-darker rounded-lg hover:shadow-md transition-all duration-200 text-left group"
              >
                <Icon className={`w-8 h-8 ${format.color} mb-4`} />
                <h4 className="font-heading font-semibold text-text-primary dark:text-text-dark mb-2">
                  {format.name}
                </h4>
                <p className="text-sm text-text-secondary mb-4">{format.description}</p>
                <div className="flex items-center gap-2 text-primary group-hover:gap-3 transition-all">
                  <Download className="w-4 h-4" />
                  <span className="text-sm font-medium">Export</span>
                </div>
              </button>
            );
          })}
        </div>
      </div>

      <div className="card">
        <h3 className="text-lg font-heading font-semibold text-text-primary dark:text-text-dark mb-6">
          Export History
        </h3>

        <div className="space-y-3">
          {[
            { name: 'scrape_data_2024_01_15.json', size: '2.4 MB', date: '2024-01-15 14:32' },
            { name: 'export_products.csv', size: '1.8 MB', date: '2024-01-14 09:21' },
            { name: 'database_backup.sql', size: '5.2 MB', date: '2024-01-13 18:45' },
          ].map((file, index) => (
            <div
              key={index}
              className="flex items-center justify-between p-4 bg-background-lighter dark:bg-background-darker rounded-lg"
            >
              <div>
                <p className="font-mono text-sm font-medium text-text-primary dark:text-text-dark">
                  {file.name}
                </p>
                <p className="text-xs text-text-secondary mt-1">
                  {file.size} • {file.date}
                </p>
              </div>
              <button className="text-primary hover:text-primary-600 transition-colors">
                <Download className="w-5 h-5" />
              </button>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

import { Save } from 'lucide-react';

export function SettingsPanel() {
  return (
    <div className="space-y-6">
      <div className="card">
        <h3 className="text-lg font-heading font-semibold text-text-primary dark:text-text-dark mb-6">
          Scraper Configuration
        </h3>

        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-text-primary dark:text-text-dark mb-2">
              Rate Limit (requests/second)
            </label>
            <input
              type="number"
              defaultValue={10}
              className="input w-full"
              placeholder="Enter rate limit"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-text-primary dark:text-text-dark mb-2">
              User Agent
            </label>
            <input
              type="text"
              defaultValue="Mozilla/5.0 (compatible; RustScraperPro/1.0)"
              className="input w-full"
              placeholder="Enter user agent"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-text-primary dark:text-text-dark mb-2">
              Timeout (seconds)
            </label>
            <input
              type="number"
              defaultValue={30}
              className="input w-full"
              placeholder="Enter timeout"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-text-primary dark:text-text-dark mb-2">
              Max Concurrent Requests
            </label>
            <input
              type="number"
              defaultValue={5}
              className="input w-full"
              placeholder="Enter max concurrent requests"
            />
          </div>
        </div>
      </div>

      <div className="card">
        <h3 className="text-lg font-heading font-semibold text-text-primary dark:text-text-dark mb-6">
          Database Connection
        </h3>

        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-text-primary dark:text-text-dark mb-2">
              Database URL
            </label>
            <input
              type="text"
              defaultValue="postgresql://localhost:5432/scraper"
              className="input w-full font-mono text-sm"
              placeholder="Enter database URL"
            />
          </div>

          <div className="flex items-center gap-2">
            <input type="checkbox" id="auto-save" className="w-4 h-4 text-primary" defaultChecked />
            <label htmlFor="auto-save" className="text-sm text-text-primary dark:text-text-dark">
              Auto-save scraped data to database
            </label>
          </div>
        </div>
      </div>

      <button className="btn-primary flex items-center gap-2">
        <Save className="w-4 h-4" />
        Save Settings
      </button>
    </div>
  );
}

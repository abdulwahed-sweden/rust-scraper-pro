import { useState } from 'react';
import { Navbar } from './components/Navbar';
import { Sidebar } from './components/Sidebar';
import { MobileMenu } from './components/MobileMenu';
import { Dashboard } from './pages/Dashboard';
import { SettingsPanel } from './components/SettingsPanel';
import { ExportPanel } from './components/ExportPanel';

function App() {
  const [activeSection, setActiveSection] = useState('dashboard');
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);

  const renderContent = () => {
    switch (activeSection) {
      case 'dashboard':
        return <Dashboard />;
      case 'settings':
        return <SettingsPanel />;
      case 'exports':
        return <ExportPanel />;
      case 'sources':
        return (
          <div className="card">
            <h2 className="text-2xl font-display font-bold text-text-primary dark:text-text-dark mb-4">
              Scraping Sources
            </h2>
            <p className="text-text-secondary">Configure your data sources here</p>
          </div>
        );
      case 'database':
        return (
          <div className="card">
            <h2 className="text-2xl font-display font-bold text-text-primary dark:text-text-dark mb-4">
              Database Management
            </h2>
            <p className="text-text-secondary">Manage your database connections and data</p>
          </div>
        );
      default:
        return <Dashboard />;
    }
  };

  return (
    <div className="min-h-screen">
      <Navbar onMenuToggle={() => setIsMobileMenuOpen(true)} />
      <MobileMenu
        isOpen={isMobileMenuOpen}
        onClose={() => setIsMobileMenuOpen(false)}
        activeSection={activeSection}
        onSectionChange={setActiveSection}
      />
      <div className="flex">
        <Sidebar activeSection={activeSection} onSectionChange={setActiveSection} />
        <main className="flex-1 p-4 sm:p-6 lg:p-8 overflow-y-auto h-[calc(100vh-73px)]">
          <div className="max-w-7xl mx-auto">
            {renderContent()}
          </div>
        </main>
      </div>
    </div>
  );
}

export default App;

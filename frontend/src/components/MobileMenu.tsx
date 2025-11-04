import { X, LayoutDashboard, Database, Download, Settings, Globe } from 'lucide-react';

interface MobileMenuProps {
  isOpen: boolean;
  onClose: () => void;
  activeSection: string;
  onSectionChange: (section: string) => void;
}

export function MobileMenu({ isOpen, onClose, activeSection, onSectionChange }: MobileMenuProps) {
  const menuItems = [
    { id: 'dashboard', label: 'Dashboard', icon: LayoutDashboard },
    { id: 'sources', label: 'Sources', icon: Globe },
    { id: 'database', label: 'Database', icon: Database },
    { id: 'exports', label: 'Exports', icon: Download },
    { id: 'settings', label: 'Settings', icon: Settings },
  ];

  const handleSectionChange = (section: string) => {
    onSectionChange(section);
    onClose();
  };

  if (!isOpen) return null;

  return (
    <>
      <div
        className="fixed inset-0 bg-black/50 z-40 lg:hidden"
        onClick={onClose}
      />
      <div className="fixed inset-y-0 left-0 w-64 bg-background-light dark:bg-background-dark border-r border-border-light dark:border-border-dark z-50 lg:hidden">
        <div className="flex items-center justify-between p-4 border-b border-border-light dark:border-border-dark">
          <h2 className="font-display font-bold text-text-primary dark:text-text-dark">Menu</h2>
          <button
            onClick={onClose}
            className="p-2 rounded-lg hover:bg-background-lighter dark:hover:bg-background-darker transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        <div className="flex flex-col gap-2 p-4">
          {menuItems.map((item) => {
            const Icon = item.icon;
            const isActive = activeSection === item.id;

            return (
              <button
                key={item.id}
                onClick={() => handleSectionChange(item.id)}
                className={`flex items-center gap-3 px-4 py-3 rounded-lg transition-all duration-200 ${
                  isActive
                    ? 'bg-primary text-white shadow-md'
                    : 'text-text-secondary hover:bg-background-lighter dark:hover:bg-background-darker hover:text-text-primary dark:hover:text-text-dark'
                }`}
              >
                <Icon className="w-5 h-5" />
                <span className="font-medium">{item.label}</span>
              </button>
            );
          })}
        </div>
      </div>
    </>
  );
}

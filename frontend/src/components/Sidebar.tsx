import { LayoutDashboard, Database, Download, Settings, Globe } from 'lucide-react';

interface SidebarProps {
  activeSection: string;
  onSectionChange: (section: string) => void;
}

export function Sidebar({ activeSection, onSectionChange }: SidebarProps) {
  const menuItems = [
    { id: 'dashboard', label: 'Dashboard', icon: LayoutDashboard },
    { id: 'sources', label: 'Sources', icon: Globe },
    { id: 'database', label: 'Database', icon: Database },
    { id: 'exports', label: 'Exports', icon: Download },
    { id: 'settings', label: 'Settings', icon: Settings },
  ];

  return (
    <aside className="hidden lg:flex flex-col w-64 bg-background-light dark:bg-background-dark border-r border-border-light dark:border-border-dark h-[calc(100vh-73px)]">
      <div className="flex flex-col gap-2 p-4">
        {menuItems.map((item) => {
          const Icon = item.icon;
          const isActive = activeSection === item.id;

          return (
            <button
              key={item.id}
              onClick={() => onSectionChange(item.id)}
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
    </aside>
  );
}

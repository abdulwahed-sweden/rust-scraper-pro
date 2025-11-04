import { Settings, Moon, Sun, Menu } from 'lucide-react';
import { useTheme } from '../contexts/ThemeContext';

interface NavbarProps {
  onMenuToggle: () => void;
}

export function Navbar({ onMenuToggle }: NavbarProps) {
  const { theme, toggleTheme } = useTheme();

  return (
    <nav className="sticky top-0 z-50 bg-background-light dark:bg-background-dark border-b border-border-light dark:border-border-dark">
      <div className="px-4 sm:px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <button
              onClick={onMenuToggle}
              className="lg:hidden p-2 rounded-lg hover:bg-background-lighter dark:hover:bg-background-darker transition-colors"
              aria-label="Toggle menu"
            >
              <Menu className="w-5 h-5 text-text-primary dark:text-text-dark" />
            </button>
            <div className="w-8 h-8 sm:w-10 sm:h-10 bg-primary rounded-lg flex items-center justify-center">
              <Settings className="w-5 h-5 sm:w-6 sm:h-6 text-white" />
            </div>
            <div>
              <h1 className="text-lg sm:text-xl font-display font-bold text-text-primary dark:text-text-dark">
                Rust Scraper Pro
              </h1>
              <p className="text-xs text-text-secondary hidden sm:block">Modern Web Scraping & Analysis</p>
            </div>
          </div>

          <div className="flex items-center gap-2 sm:gap-4">
            <button
              onClick={toggleTheme}
              className="p-2 rounded-lg hover:bg-background-lighter dark:hover:bg-background-darker transition-colors"
              aria-label="Toggle theme"
            >
              {theme === 'light' ? (
                <Moon className="w-5 h-5 text-text-secondary" />
              ) : (
                <Sun className="w-5 h-5 text-text-secondary" />
              )}
            </button>
          </div>
        </div>
      </div>
    </nav>
  );
}

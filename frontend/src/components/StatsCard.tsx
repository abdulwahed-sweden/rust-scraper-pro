import { LucideIcon } from 'lucide-react';

interface StatsCardProps {
  title: string;
  value: string | number;
  icon: LucideIcon;
  trend?: {
    value: string;
    positive: boolean;
  };
  subtitle?: string;
}

export function StatsCard({ title, value, icon: Icon, trend, subtitle }: StatsCardProps) {
  return (
    <div className="card">
      <div className="flex items-start justify-between">
        <div className="flex-1">
          <p className="text-sm font-medium text-text-secondary mb-1">{title}</p>
          <h3 className="text-3xl font-display font-bold text-text-primary dark:text-text-dark">
            {value}
          </h3>
          {subtitle && (
            <p className="text-xs text-text-secondary mt-1">{subtitle}</p>
          )}
          {trend && (
            <div className="flex items-center gap-1 mt-2">
              <span className={`text-sm font-medium ${trend.positive ? 'text-green-500' : 'text-red-500'}`}>
                {trend.positive ? '↑' : '↓'} {trend.value}
              </span>
              <span className="text-xs text-text-secondary">vs last week</span>
            </div>
          )}
        </div>
        <div className="w-12 h-12 rounded-lg bg-primary/10 flex items-center justify-center">
          <Icon className="w-6 h-6 text-primary" />
        </div>
      </div>
    </div>
  );
}

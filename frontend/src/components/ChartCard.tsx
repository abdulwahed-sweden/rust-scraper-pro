import { TrendingUp } from 'lucide-react';

interface DataPoint {
  label: string;
  value: number;
}

interface ChartCardProps {
  title: string;
  data: DataPoint[];
}

export function ChartCard({ title, data }: ChartCardProps) {
  const maxValue = Math.max(...data.map(d => d.value));

  return (
    <div className="card">
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-lg font-heading font-semibold text-text-primary dark:text-text-dark">
          {title}
        </h3>
        <TrendingUp className="w-5 h-5 text-primary" />
      </div>

      <div className="space-y-4">
        {data.map((item, index) => {
          const percentage = (item.value / maxValue) * 100;

          return (
            <div key={index} className="space-y-2">
              <div className="flex items-center justify-between text-sm">
                <span className="text-text-secondary">{item.label}</span>
                <span className="font-mono font-medium text-text-primary dark:text-text-dark">
                  {item.value}
                </span>
              </div>
              <div className="h-2 bg-background-lighter dark:bg-background-darker rounded-full overflow-hidden">
                <div
                  className="h-full bg-gradient-to-r from-primary to-secondary rounded-full transition-all duration-500"
                  style={{ width: `${percentage}%` }}
                />
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}

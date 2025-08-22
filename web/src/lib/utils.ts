import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}


export function formatRelativeTime(ts: number): string {
  const now = Date.now();
  const diffMs = now - ts;
  const units = [
    { limit: 1000 * 30,           text: () => 'just now' },               // ≤ 30 s
    { limit: 1000 * 60,           text: (n: number) => `${n} second${n > 1 ? 's' : ''} ago` },
    { limit: 1000 * 60 * 60,      text: (n: number) => `${n} minute${n > 1 ? 's' : ''} ago` },
    { limit: 1000 * 60 * 60 * 24, text: (n: number) => `${n} hour${n > 1 ? 's' : ''} ago` },
    { limit: 1000 * 60 * 60 * 24 * 7,   text: (n: number) => `${n} day${n > 1 ? 's' : ''} ago` },
    { limit: 1000 * 60 * 60 * 24 * 30,  text: (n: number) => `${n} week${n > 1 ? 's' : ''} ago` },
    { limit: 1000 * 60 * 60 * 24 * 365, text: (n: number) => `${n} month${n > 1 ? 's' : ''} ago` },
  ];

  for (const unit of units) {
    if (diffMs < unit.limit) {
      const amount = Math.floor(diffMs / (unit.limit / (unit === units[0] ? 1 : (unit.limit / (units[units.indexOf(unit) - 1]?.limit || unit.limit))))); // 计算当前单位数量
      return unit.text(amount);
    }
  }
  const years = Math.floor(diffMs / (1000 * 60 * 60 * 24 * 365));
  return `${years} year${years > 1 ? 's' : ''} ago`;
}
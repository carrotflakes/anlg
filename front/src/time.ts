export function formatDateTimeRelative(date: Date): string {
  const now = new Date();
  const diff = now.getTime() - date.getTime();
  if (diff < 60 * 1000) {
    return "just now";
  } else if (diff < 60 * 60 * 1000) {
    return `${Math.floor(diff / (60 * 1000))}m ago`;
  } else if (diff < 24 * 60 * 60 * 1000) {
    return `${Math.floor(diff / (60 * 60 * 1000))}h ago`;
  } else {
    return `${Math.floor(diff / (24 * 60 * 60 * 1000))}d ago`;
  }
}

export function formatDateTime(date: Date): string {
  return date.toISOString().replace(/T/, " ").replace(/\..+/, "");
}

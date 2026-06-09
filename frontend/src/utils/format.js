export function formatBytes(value) {
  if (!Number.isFinite(value)) {
    return "--";
  }
  if (value < 1024) {
    return `${value} B`;
  }
  return `${(value / 1024).toFixed(1)} KB`;
}

export function formatRange(min, max) {
  return min && max ? `${min}-${max}` : "未知";
}

import type { SettingsDto } from '../api/types';

export const defaultSettings: SettingsDto = {
  default_download_dir: '~/Downloads',
  status_refresh_ms: 1000,
  log_level: 'info'
};

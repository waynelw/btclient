export type TaskStatus = 'queued' | 'checking' | 'downloading' | 'paused' | 'completed' | 'error';

export interface TaskErrorDto {
  code: string;
  message: string;
  occurred_at: string;
}

export interface TaskDto {
  id: string;
  name: string;
  status: TaskStatus;
  torrent_path: string;
  download_dir: string;
  total_bytes: number;
  downloaded_bytes: number;
  progress: number;
  download_speed_bps: number;
  upload_speed_bps: number;
  peers_connected: number;
  peers_total: number;
  eta_seconds: number | null;
  created_at: string;
  updated_at: string;
  error: TaskErrorDto | null;
}

export interface SettingsDto {
  default_download_dir: string;
  status_refresh_ms: number;
  log_level: 'error' | 'warn' | 'info' | 'debug' | 'trace';
}

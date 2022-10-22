// import * as git2rs from '../git2rs/api.ts';
import { invoke } from '@tauri-apps/api/tauri';

export async function add(): Promise<String> {
  return invoke('add');
}

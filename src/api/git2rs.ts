// import * as git2rs from '../git2rs/api.ts';
import { invoke } from '@tauri-apps/api/tauri';

export async function add(name): Promise<bool> {
  return invoke('add', {args: name});
}

export async function remove(name): Promise<bool> {
  return invoke('remove', {args: name});
}
export async function resetStage(name): Promise<bool> {
  return invoke('reset_stage', {args: name});
}

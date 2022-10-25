// import * as git2rs from '../git2rs/api.ts';
import { invoke } from '@tauri-apps/api/tauri';

export async function add(name): Promise<bool> {
  return invoke('add', {args: name});
}

export async function reset(name): Promise<bool> {
  return invoke('reset', {args: name});
}

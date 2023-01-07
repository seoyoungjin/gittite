// settings
export interface Profile {
  name: string;
  email: string;
  image_url: string;
}

export interface Settings {
  profile: Profile;
  all_repository: string[];
}

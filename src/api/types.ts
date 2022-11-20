export type Settings = {
  repo: string;
};

export type StatusItem = {
  path: string;
  stage?: string;
  wtree?: string;
};

export type DiffOptions = {
  ignore_whitespace: boolean;
  ontext: number;
  interhunk_lines: number;
};

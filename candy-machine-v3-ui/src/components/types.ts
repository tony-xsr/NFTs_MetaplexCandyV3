export type MintGroupMetadata = {
  label: string;
  title?: string;
  description?: string;
  allowList?:string[];
};

export type MintGroupsMetadata = {
  title: string;
  description?: string;
  groups: MintGroupMetadata[];
};

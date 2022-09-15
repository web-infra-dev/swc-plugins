export type config = ImportItem[];

export interface ImportItem {
  fromSource: string;
  replaceJs?: {
    ignoreEsComponent?: string[];
    replaceExpr?: (member: string) => (string | false);
    replaceTpl?: string;
    lower?: boolean;
    camel2DashComponentName?: boolean;
    transformToDefaultImport?: boolean;
  };
  replaceCss?: {
    ignoreStyleComponent?: string[];
    replaceExpr?: (member: string) => (string | false);
    replaceTpl?: string;
    lower?: boolean;
    camel2DashComponentName?: boolean;
  };
}

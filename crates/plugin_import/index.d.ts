export type config<Async extends boolean> = ImportItem<Async>[];

export interface ImportItem<Async extends boolean> {
  fromSource: string;
  replace_js?: {
    ignore_es_component?: string[];
    replace_expr?: Async extends true
      ? never
      : (member: string) => string | false;
    replace_tpl?: string;
    lower?: boolean;
    camel2_dash_component_name?: boolean;
  };
  replace_css?: {
    ignore_style_component?: string[];
    replace_expr?: Async extends true
      ? never
      : (member: string) => string | false;
    replace_tpl?: string;
    lower?: boolean;
    camel2_dash_component_name?: boolean;
  };
}

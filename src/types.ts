export interface GameInfo {
  id: string;
  title: string;
}

export interface PgnHeaders {
  Title?: string;
  Event?: string;
  Site?: string;
  Date?: string;
  Round?: string;
  White?: string;
  Black?: string;
  Result?: string;
  [key: string]: string | undefined;
}

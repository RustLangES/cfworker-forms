export type ApiResponse<T> = {
  success: true;
  data: T;
  errors: string[];
  messages?: string[];
} | {
  success: false;
  data?: undefined;
  errors: string[];
  messages?: string[];
};

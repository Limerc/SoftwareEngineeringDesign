export type Method = 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH'
export enum ContentType {
  FORM = 'multipart/form-data',
  JSON = 'application/json',
  URL = 'application/x-www-form-urlencoded',
  BIN = 'application/octet-stream'
}
export interface RequestOptions {
  url: string
  method?: Method
  params?: any
  contentType?: ContentType
  data?: any
  noAuth?: any
  showLoading?: boolean
  showError?: boolean
}
export const AUTH = 'Authorization'

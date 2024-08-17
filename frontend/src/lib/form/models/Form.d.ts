import type { FormStep } from "./Step";

export interface Form {
  id: number,

  title: string,
  edition: string,
  description: string,

  multiple_times: bool,
  require_login: bool,

  questions: Array<FormStep>,

  created_at: string,
  deleted: boolean,
}

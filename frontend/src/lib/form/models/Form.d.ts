import type { FormStep } from "./Step";

export interface Form {
  title: string,
  steps: Array<FormStep>
}

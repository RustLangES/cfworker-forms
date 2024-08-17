type Step<Type extends string, Data extends object> = {
  id: number,
  title: string,
  description: string,

  type: Type,
  data: Data,
}

export type FormStep = 
  | FormStepText
  | FormStepQuestionText
  | FormStepOptions;

export type FormStepText = Step<"text", {}>;

export type FormStepQuestionText = Step<"questionText", { required?: boolean }>;

export type FormStepOptions = Step<"options", {
  canMultiple: boolean,
  required: boolean,

  options: Array<string>,
}>;

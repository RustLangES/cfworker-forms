type CreateStep<Type extends string, Data> = {
  type: Type,
  data: Data,
}

export type FormStep = 
  | FormStepText
  | FormStepQuestionText
  | FormStepOptions;

export type FormStepText = CreateStep<"text", {
  title: string,
  description: string,
}>;

export type FormStepQuestionText = CreateStep<"questionText", {
  question: string,
  description: string,

  required: boolean
}>;

export type FormStepOptions = CreateStep<"options", {
  question: string,
  description: string,

  options: Array<string>,

  required: boolean,
  canMultiple: boolean
}>;

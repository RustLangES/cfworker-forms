import type { Form } from "$lib/form/models/Form";

export function load(): Form {
  return {
    title: "Test Form",
    steps: [
      {
        type: "text",
        data: {
          title: "Hello World!",
          description: "Hello World!",
        }
      },
      {
        type: "questionText",
        data: {
          question: "Question Text",
          description: "Hello World!",

          required: true
        }
      },
      {
        type: "options",
        data: {
          question: "Options",
          description: "Hello World!",

          required: false,
          canMultiple: false,

          options: [
            "A",
            "B",
            "C"
          ]
        }
      },
      {
        type: "options",
        data: {
          question: "Options (Multiple)",
          description: "Hello World!",

          required: false,
          canMultiple: true,

          options: [
            "A",
            "B",
            "C"
          ]
        }
      }
    ]
  }
}

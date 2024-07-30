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
          title: "Question Text",
          description: "Hello World!\nHello World!\nHello World!\nHello Worldalssdkljhasdlkjh asdlkjhasdlk jhasdkljhasdkjhaskdjhd!\n",

          required: true
        }
      },
      {
        type: "options",
        data: {
          title: "Options",
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
          title: "Options (Multiple)",
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

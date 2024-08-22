import type { Form } from "$lib/form/models/Form";

export async function load({ platform }: App.LoadServerEvent) {
  const FORM_URL = `${platform.env.API_HOST}/api/form`;

  console.log("Loading forms from:", FORM_URL);
  const forms_res = await fetch(FORM_URL);

  if (!forms_res.ok) {
    console.error("Cannot get forms: status", forms_res.status);

    const msg = await forms_res.json();
    console.error(msg);
  }

  const forms = await forms_res.json() as Form[];

  return { forms };
}

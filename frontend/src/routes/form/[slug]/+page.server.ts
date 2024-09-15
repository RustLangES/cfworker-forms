import type { Answer } from "$lib/form/models/Answer.d";
import type { Form } from "$lib/form/models/Form";
import type { ApiResponse } from "$lib/models/api";
import { error, redirect } from "@sveltejs/kit";

export async function load(
  { url, params, platform, cookies, request }: App.LoadServerEvent<{ slug: string }>,
) {
  if (url.searchParams.has("code")) {
    const code = url.searchParams.get("code")!;

    cookies.set("external-code", code, { path: "/" });

    // Remove code from url to prevent leaks if anyone is streaming haha
    return redirect(301, `/form/${params.slug}`);
  }

  const FORM_URL = `${platform!.env.API_HOST}/api/form/${params.slug}`;

  console.log(`Loading form ${params.slug} from: ${FORM_URL}`);
  const form_res = await fetch(FORM_URL);
  const form = await form_res.json() as ApiResponse<Form>;
  console.log("Form loaded:", form);

  if (form.success) {
    const code = cookies.get("external-code");
    console.log("User has code: ", !!code)

    if (form.data.require_login && !code) {
      return {
        form: form.data,
        user: null,
      };
    }

    console.log(`Loading session from: ${FORM_URL}/session`);
    const session_res = await fetch(`${FORM_URL}/session`, {
      headers: code
        ? {
          "Authorization": `Bearer ${code}`,
          "user-agent": request.headers.get("user-agent"),
        }
        : {
          "user-agent": request.headers.get("user-agent"),
        },
    });

    const session = await session_res.json() as ApiResponse<string>;
    console.log("Session loaded:", session);

    if (session.success) {
      const user = session.data!;

      const answers_res = await fetch(`${FORM_URL}/answer`, {
        headers: { "Authorization": `Bearer ${user}` },
      });

      const answers = await answers_res.json() as ApiResponse<Answer[]>

      if (answers.success) {
        return {
          form: form.data,
          API_HOST: platform!.env.API_HOST,
          user,
          answers: answers.data,
        };
      }

      return {
        form: form.data,
        API_HOST: platform!.env.API_HOST,
        user,
        answers: [],
      };
    } else if (session_res.status === 403) {
      return error(session_res.status, "Already answered");
    } else if (session_res.status === 401) {
      // External Code is expired
      cookies.delete("external-code", { path: "/" });
      return redirect(301, `/form/${params.slug}`);
    } else if (!session.success) {
      return error(session_res.status, session.errors[0]);
    }

    return {
      form: form.data,
      user: null,
    };
  } else {
    return error(form_res.status, form.errors[0]);
  }
}

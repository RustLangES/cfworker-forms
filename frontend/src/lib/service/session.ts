import type { ApiResponse } from "$lib/models/api";
import { derived, writable } from "svelte/store";

const getFormKey = (form: number) => `form-${form.toFixed(0).padStart(2, "0")}`;

const _activeSession = writable<[form: number, session: string]>();

_activeSession.subscribe(([form, session]) => {
  window.localStorage.setItem(getFormKey(form), session);
});

export const activeSession = derived(_activeSession, ([_, session]) => session);
export const activeForm = derived(_activeSession, ([form, _]) => form);

export async function loadSession(
  form: number,
  platform: App.Platform,
): Promise<string | undefined> {
  const lastSession = window.localStorage.getItem(getFormKey(form));

  if (typeof lastSession === "string") {
    _activeSession.set([form, lastSession]);
    return;
  }

  const res =
    await (await fetch(`${platform.env.API_HOST}/api/form/${form}/session`))
      .json() as ApiResponse<string>;

  if (res.success) {
    _activeSession.set([form, res.data]);
  } else {
    return res.errors[0];
  }
}

export function unloadSession(form: number) {
  window.localStorage.removeItem(getFormKey(form));
}

import { redirect } from "@sveltejs/kit";

export const GET = ({ url, platform }: App.LoadServerEvent) => {
  const form = url.searchParams.get("form");

  const query = form ? `?redirect_to=${url.origin}/form/${form}` : "";

  return redirect(301, `${platform!.env.API_HOST}/api/login/github${query}`);
};

import "@sveltejs/adapter-cloudflare-workers";

declare global {
  namespace App {
    type LoadServerEvent<
      Params extends Partial<Record<string, string>> = Partial<
        Record<string, string>
      >,
    > = import("@sveltejs/kit").ServerLoadEvent<Params>;

    // interface Error {}
    // interface Locals {}
    // interface PageData {}
    // interface PageState {}
    interface Platform {
      env: {
        API_HOST?: string;
      };
    }

    interface ViewTransition {
      updateCallbackDone: Promise<void>;
      ready: Promise<void>;
      finished: Promise<void>;
      skipTransition: () => void;
    }

    interface Document {
      startViewTransition(updateCallback: () => Promise<void>): ViewTransition;
    }
  }
}

export {};

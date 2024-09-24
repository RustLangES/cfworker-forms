import { nanoid } from "nanoid";

const DEVICE_ID = "DEVICE_ID";

let id = localStorage.getItem(DEVICE_ID)

if (typeof id !== "string") {
  id = nanoid();

  localStorage.setItem(DEVICE_ID, id);
}

import { cubicOut } from "svelte/easing";
import type { EasingFunction, TransitionConfig } from "svelte/transition";

export const enum SlideAxis {
  X,
  Y,
}

export const enum SlideDirection {
  Forward,
  Backward,
}

export interface SlideParams {
  delay?: number;
  duration?: number;
  easing?: EasingFunction;

  axis: SlideAxis;
  direction?: SlideDirection;
  distance?: number;
}

export function slide(
  {
    delay = 0,
    duration = 400,
    easing = cubicOut,
    axis,
    direction = SlideDirection.Forward,
    distance = 20,
  }: SlideParams,
): [(node: Element) => TransitionConfig, (node: Element) => TransitionConfig] {
  const getOffset = (node: Element, isOut: boolean): TransitionConfig => {
    const style = getComputedStyle(node);
    const opacity = +style.opacity;
    const transform = style.transform.slice(0, -1);

    const translate = "translate" + (axis === SlideAxis.X ? "X" : "Y");
    const dir = direction === SlideDirection.Forward ? -1 : 1;

    const offsetXY = transform.slice(
      transform.lastIndexOf(",", transform.lastIndexOf(",") - 1) + 1,
    ).split(",").map((v) => +v.trimStart());

    const offset = offsetXY[axis === SlideAxis.X ? 0 : 1];

    return {
      delay,
      duration,
      easing,
      css: (t, u) => {
        const offs = u * (isOut ? -distance : distance) * dir;

        const offs_s = offs < 0
          ? " - " + -offs.toFixed(2)
          : " + " + offs.toFixed(2);

        return `opacity: ${t * opacity};` +
          `transform: ${translate}(calc(${offset}px ${offs_s}px));`;
      },
    };
  };

  return [
    (node) => getOffset(node, false),
    (node) => getOffset(node, true),
  ];
}

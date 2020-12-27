import { RefObject, useEffect, useRef, useState } from "react";
import { Observable, Subject, Subscription } from "rxjs";

export type EventCallback<T> = (event: T) => void;

export function useEvents<T, V = T>(
  setup?: (observable: Observable<T>) => void | Observable<V>
): [Observable<V>, EventCallback<T>] {
  const obsRef = useRef<Subject<T>>();
  const mapRef = useRef<Observable<V>>();

  if (!obsRef.current) {
    obsRef.current = new Subject<T>();
    mapRef.current = (obsRef.current as unknown) as Observable<V>;

    if (setup) {
      const newObs = setup(obsRef.current);
      if (newObs) {
        mapRef.current = newObs;
      }
    }
  }

  useEffect(() => () => obsRef.current!.complete(), []);
  return [mapRef.current!, ev => obsRef.current!.next(ev)];
}

export function useObservable<T>(observable: Observable<T>, initialValue: T): T;
export function useObservable<T>(observable: Observable<T>): T | undefined;

export function useObservable<T>(
  observableFactory: () => Observable<T>,
  initialValue: T
): T;
export function useObservable<T>(
  observableFactory: () => Observable<T>
): T | undefined;

export function useObservable<T>(
  obs: Observable<T> | (() => Observable<T>),
  initialValue?: undefined
): T | undefined {
  const obsRef = useRef<{ obs: Observable<T>; sub: Subscription }>();

  if (!obsRef.current) {
    if (typeof (obs as any).getValue === "function") {
      initialValue = (obs as any).getValue();
    } else if (typeof (obs as any).value !== "undefined") {
      initialValue = (obs as any).value;
    }
  }

  const [val, setVal] = useState<T | undefined>(initialValue);

  if (!obsRef.current) {
    if (typeof obs === "function") {
      obs = obs();
    }

    if (typeof (obs as any).getValue === "function") {
      setVal((obs as any).getValue());
    } else if (typeof (obs as any).value !== "undefined") {
      setVal((obs as any).value);
    }

    obsRef.current = {
      obs,
      sub: obs.subscribe(setVal),
    };
  }

  useEffect(() => () => obsRef.current?.sub.unsubscribe(), []);

  return val;
}

export function useObservableState<T, V = T>(
  initialValue: V,
  setup?: (observable: Observable<T>) => void | Observable<V>
): [V, EventCallback<T>, Observable<V>];

export function useObservableState<T, V = T>(
  initialValue?: undefined,
  setup?: (observable: Observable<T>) => void | Observable<V>
): [V | undefined, EventCallback<T>, Observable<V>];

export function useObservableState<T, V = T>(
  initialValue?: V,
  setup?: (observable: Observable<T>) => void | Observable<V>
): [V | undefined, EventCallback<T>, Observable<V>] {
  const [event$, cb] = useEvents(setup);
  return [useObservable(event$, initialValue), cb, event$];
}

export function useObservableRef<T, V = T>(
  initialValue: V,
  setup?: (observable: Observable<T>) => void | Observable<V>
): [RefObject<V>, EventCallback<T>, Observable<V>];

export function useObservableRef<T, V = T>(
  initialValue?: undefined,
  setup?: (observable: Observable<T>) => void | Observable<V>
): [RefObject<V | undefined>, EventCallback<T>, Observable<V>];

export function useObservableRef<T, V = T>(
  initialValue?: V,
  setup?: (observable: Observable<T>) => void | Observable<V>
): [RefObject<V | undefined>, EventCallback<T>, Observable<V>] {
  const r = useRef<V>();
  if (typeof initialValue !== "undefined" && typeof r.current === "undefined") {
    r.current = initialValue;
  }
  const [event$, cb] = useEvents(setup);
  event$.subscribe(v => (r.current = v));
  return [r, cb, event$];
}

export function useObservableStateRef<T, V = T>(
  initialValue: V,
  setup?: (observable: Observable<T>) => void | Observable<V>
): [RefObject<V>, EventCallback<T>, Observable<V>];

export function useObservableStateRef<T, V = T>(
  initialValue?: undefined,
  setup?: (observable: Observable<T>) => void | Observable<V>
): [RefObject<V | undefined>, EventCallback<T>, Observable<V>];

export function useObservableStateRef<T, V = T>(
  initialValue?: V,
  setup?: (observable: Observable<T>) => void | Observable<V>
): [RefObject<V | undefined>, EventCallback<T>, Observable<V>] {
  let [_, setV] = useState(initialValue);
  const r = useRef<V>();
  if (typeof initialValue !== "undefined" && typeof r.current === "undefined") {
    r.current = initialValue;
  }
  const [event$, cb] = useEvents(setup);
  event$.subscribe(v => {
    r.current = v;
    setV(v);
  });
  return [r, cb, event$];
}

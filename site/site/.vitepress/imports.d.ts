declare module "*.svg" {
  const v: string;
  export default v;
}

declare module "*?raw" {
  const v: string;
  export default v;
}

declare module "*?worker" {
  const v: <T extends Worker = Worker>() => T;
  export default v;
}

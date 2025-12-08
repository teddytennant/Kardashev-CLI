export type CodexOptions = {
  codexPathOverride?: string;
  baseUrl?: string;
  apiKey?: string;
  /**
   * Environment variables passed to the Kardashev CLI process. When provided, the SDK
   * will not inherit variables from `process.env`.
   */
  env?: Record<string, string>;
};

export function normalizeError(error: Error | unknown): Error {
  return error instanceof Error
    ? error
    : new Error("An unknown error occurred.");
}

import CONSOLE from "src/utils/console";

export enum StatusCode {
    ArbitraryJSError = -2,
    JsonDecodeError = -1,

    Unauthenticated = 1000,
}

export type ApiError = {
    status_code: StatusCode;
    message: string;
};

/**
 * Parse a response's body into an {@link ApiError}
 *
 * This function assumes but doesn't check, that the response is an error.
 */
export async function parseError(response: Response): Promise<ApiError> {
    try {
        return await response.json();
    } catch {
        CONSOLE.error("Got invalid json", response.body);
        return {
            status_code: StatusCode.JsonDecodeError,
            message: "The server's response was invalid json",
        };
    }
}

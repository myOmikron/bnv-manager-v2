/* tslint:disable */
/* eslint-disable */
/**
 * The response that is sent in a case of an error
 * @export
 * @interface ApiErrorResponse
 */
export interface ApiErrorResponse {
    /**
     * A human-readable error message.
     * 
     * May be used for displaying purposes
     * @type {string}
     * @memberof ApiErrorResponse
     */
    message: string;
    /**
     * The Status code for the error.
     * 
     * Important: Does not match http status codes
     * @type {ApiStatusCode}
     * @memberof ApiErrorResponse
     */
    status_code: ApiStatusCode;
}



/**
 * The Status code that are returned throughout the API
 * @export
 */
export const ApiStatusCode = {
    NUMBER_1000: 1000,
    NUMBER_1001: 1001,
    NUMBER_1002: 1002,
    NUMBER_1003: 1003,
    NUMBER_2000: 2000
} as const;
export type ApiStatusCode = typeof ApiStatusCode[keyof typeof ApiStatusCode];

/**
 * Response for the discovery endpoint
 * @export
 * @interface DiscoveryResponse
 */
export interface DiscoveryResponse {
    /**
     * 
     * @type {string}
     * @memberof DiscoveryResponse
     */
    authorization_endpoint: string;
    /**
     * 
     * @type {Array<string>}
     * @memberof DiscoveryResponse
     */
    id_token_signing_alg_values_supported: Array<string>;
    /**
     * 
     * @type {string}
     * @memberof DiscoveryResponse
     */
    issuer: string;
    /**
     * 
     * @type {string}
     * @memberof DiscoveryResponse
     */
    jwks_uri: string;
    /**
     * 
     * @type {Array<string>}
     * @memberof DiscoveryResponse
     */
    response_types_supported: Array<string>;
    /**
     * 
     * @type {Array<string>}
     * @memberof DiscoveryResponse
     */
    subject_types_supported: Array<string>;
    /**
     * 
     * @type {string}
     * @memberof DiscoveryResponse
     */
    token_endpoint: string;
    /**
     * 
     * @type {string}
     * @memberof DiscoveryResponse
     */
    userinfo_endpoint: string;
}
/**
 * Sign in request
 * @export
 * @interface SignInRequest
 */
export interface SignInRequest {
    /**
     * Password
     * @type {string}
     * @memberof SignInRequest
     */
    password: string;
    /**
     * Username
     * @type {string}
     * @memberof SignInRequest
     */
    username: string;
}
/**
 * 
 * @export
 * @interface TokenResponse
 */
export interface TokenResponse {
    /**
     * 
     * @type {string}
     * @memberof TokenResponse
     */
    access_token: string;
    /**
     * 
     * @type {number}
     * @memberof TokenResponse
     */
    expires_in: number;
    /**
     * 
     * @type {string}
     * @memberof TokenResponse
     */
    id_token: string;
    /**
     * 
     * @type {string}
     * @memberof TokenResponse
     */
    token_type: string;
}

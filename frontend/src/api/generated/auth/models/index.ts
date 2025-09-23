/* tslint:disable */
/* eslint-disable */
/**
 * The response that is sent in a case of an error the caller should report to an admin
 * @export
 * @interface ApiErrorResponse
 */
export interface ApiErrorResponse {
    /**
     * ID of the opentelemetry trace this error originated in
     * @type {string}
     * @memberof ApiErrorResponse
     */
    trace_id: string;
}
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

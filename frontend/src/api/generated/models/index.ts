/* tslint:disable */
/* eslint-disable */
/**
 * 
 * @export
 * @interface AcceptInviteRequest
 */
export interface AcceptInviteRequest {
    /**
     * 
     * @type {string}
     * @memberof AcceptInviteRequest
     */
    password: string;
}
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
     * 
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
 * 
 * @export
 * @interface FullInvite
 */
export interface FullInvite {
    /**
     * 
     * @type {string}
     * @memberof FullInvite
     */
    display_name: string;
    /**
     * 
     * @type {string}
     * @memberof FullInvite
     */
    expires_at: string;
    /**
     * 
     * @type {UserRole}
     * @memberof FullInvite
     */
    role: UserRole;
    /**
     * 
     * @type {string}
     * @memberof FullInvite
     */
    username: string;
    /**
     * 
     * @type {string}
     * @memberof FullInvite
     */
    uuid: string;
}
/**
 * @type UserRole
 * Roles for a user
 * @export
 */
export type UserRole = string;

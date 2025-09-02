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
 * A single club
 * @export
 * @interface Club
 */
export interface Club {
    /**
     * The number of admins in the club
     * @type {number}
     * @memberof Club
     */
    admin_count: number;
    /**
     * The point in time the club was created
     * @type {string}
     * @memberof Club
     */
    created_at: string;
    /**
     * The number of members in the club
     * @type {number}
     * @memberof Club
     */
    member_count: number;
    /**
     * The last point in time the club was modified
     * @type {string}
     * @memberof Club
     */
    modified_at: string;
    /**
     * Name of the club
     * @type {string}
     * @memberof Club
     */
    name: string;
    /**
     * Primary key of a club
     * @type {string}
     * @memberof Club
     */
    uuid: string;
}
/**
 * A page of items
 * @export
 * @interface PageForSimpleAccount
 */
export interface PageForSimpleAccount {
    /**
     * The page's items
     * @type {Array<SimpleAccount>}
     * @memberof PageForSimpleAccount
     */
    items: Array<SimpleAccount>;
    /**
     * The limit this page was requested with
     * @type {number}
     * @memberof PageForSimpleAccount
     */
    limit: number;
    /**
     * The offset this page was requested with
     * @type {number}
     * @memberof PageForSimpleAccount
     */
    offset: number;
    /**
     * The total number of items this page is a subset of
     * @type {number}
     * @memberof PageForSimpleAccount
     */
    total: number;
}
/**
 * Simple representation of an account.
 * @export
 * @interface SimpleAccount
 */
export interface SimpleAccount {
    /**
     * The account's display name.
     * @type {string}
     * @memberof SimpleAccount
     */
    display_name: string;
    /**
     * The account's username.
     * @type {string}
     * @memberof SimpleAccount
     */
    username: string;
    /**
     * The account's UUID.
     * @type {string}
     * @memberof SimpleAccount
     */
    uuid: string;
}

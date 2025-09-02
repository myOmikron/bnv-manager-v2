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
 * Errors that can occur while creating an invitation
 * @export
 * @interface CreateInviteError
 */
export interface CreateInviteError {
    /**
     * Username is already taken
     * @type {boolean}
     * @memberof CreateInviteError
     */
    username_already_occupied: boolean;
}
/**
 * Request to create an invitation
 * @export
 * @interface CreateMemberInviteRequest
 */
export interface CreateMemberInviteRequest {
    /**
     * Display-name of the user
     * @type {string}
     * @memberof CreateMemberInviteRequest
     */
    display_name: string;
    /**
     * Reserved username
     * @type {string}
     * @memberof CreateMemberInviteRequest
     */
    username: string;
    /**
     * The point in time the invite expires
     * @type {number}
     * @memberof CreateMemberInviteRequest
     */
    valid_days: number;
}
/**
 * @type FormResultForSingleLinkAndCreateInviteError
 * A `Result` with a custom serialization
 * @export
 */
export type FormResultForSingleLinkAndCreateInviteError = FormResultForSingleLinkAndCreateInviteErrorOneOf | FormResultForSingleLinkAndCreateInviteErrorOneOf1;
/**
 * 
 * @export
 * @interface FormResultForSingleLinkAndCreateInviteErrorOneOf
 */
export interface FormResultForSingleLinkAndCreateInviteErrorOneOf {
    /**
     * 
     * @type {string}
     * @memberof FormResultForSingleLinkAndCreateInviteErrorOneOf
     */
    result: FormResultForSingleLinkAndCreateInviteErrorOneOfResultEnum;
    /**
     * 
     * @type {SingleLink}
     * @memberof FormResultForSingleLinkAndCreateInviteErrorOneOf
     */
    value: SingleLink;
}


/**
 * @export
 */
export const FormResultForSingleLinkAndCreateInviteErrorOneOfResultEnum = {
    Ok: 'Ok'
} as const;
export type FormResultForSingleLinkAndCreateInviteErrorOneOfResultEnum = typeof FormResultForSingleLinkAndCreateInviteErrorOneOfResultEnum[keyof typeof FormResultForSingleLinkAndCreateInviteErrorOneOfResultEnum];

/**
 * 
 * @export
 * @interface FormResultForSingleLinkAndCreateInviteErrorOneOf1
 */
export interface FormResultForSingleLinkAndCreateInviteErrorOneOf1 {
    /**
     * 
     * @type {CreateInviteError}
     * @memberof FormResultForSingleLinkAndCreateInviteErrorOneOf1
     */
    error: CreateInviteError;
    /**
     * 
     * @type {string}
     * @memberof FormResultForSingleLinkAndCreateInviteErrorOneOf1
     */
    result: FormResultForSingleLinkAndCreateInviteErrorOneOf1ResultEnum;
}


/**
 * @export
 */
export const FormResultForSingleLinkAndCreateInviteErrorOneOf1ResultEnum = {
    Err: 'Err'
} as const;
export type FormResultForSingleLinkAndCreateInviteErrorOneOf1ResultEnum = typeof FormResultForSingleLinkAndCreateInviteErrorOneOf1ResultEnum[keyof typeof FormResultForSingleLinkAndCreateInviteErrorOneOf1ResultEnum];

/**
 * API representation of an invitation
 * @export
 * @interface GetInvite
 */
export interface GetInvite {
    /**
     * The point in time the invite was created
     * @type {string}
     * @memberof GetInvite
     */
    created_at: string;
    /**
     * Display-name of the user
     * @type {string}
     * @memberof GetInvite
     */
    display_name: string;
    /**
     * The point in time the invite expires
     * @type {string}
     * @memberof GetInvite
     */
    expires_at: string;
    /**
     * Public link for accessing the invite
     * @type {string}
     * @memberof GetInvite
     */
    link: string;
    /**
     * Reserved username
     * @type {string}
     * @memberof GetInvite
     */
    username: string;
    /**
     * Primary key of the invite
     * @type {string}
     * @memberof GetInvite
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
/**
 * A single string representing a link wrapped in a struct
 * @export
 * @interface SingleLink
 */
export interface SingleLink {
    /**
     * 
     * @type {string}
     * @memberof SingleLink
     */
    link: string;
}

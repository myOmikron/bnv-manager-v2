/* tslint:disable */
/* eslint-disable */
/**
 * Accept an open invite
 * @export
 * @interface AcceptInvite
 */
export interface AcceptInvite {
    /**
     * The new password to set
     * @type {string}
     * @memberof AcceptInvite
     */
    password: string;
}
/**
 * Errors that can occur while accepting an invitation
 * @export
 * @interface AcceptInviteError
 */
export interface AcceptInviteError {
    /**
     * Empty password was supplied
     * @type {boolean}
     * @memberof AcceptInviteError
     */
    empty_password: boolean;
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
 * @type FormResultForNullAndAcceptInviteError
 * A `Result` with a custom serialization
 * @export
 */
export type FormResultForNullAndAcceptInviteError = FormResultForNullAndAcceptInviteErrorOneOf | FormResultForNullAndAcceptInviteErrorOneOf1;
/**
 * 
 * @export
 * @interface FormResultForNullAndAcceptInviteErrorOneOf
 */
export interface FormResultForNullAndAcceptInviteErrorOneOf {
    /**
     * 
     * @type {string}
     * @memberof FormResultForNullAndAcceptInviteErrorOneOf
     */
    result: FormResultForNullAndAcceptInviteErrorOneOfResultEnum;
    /**
     * 
     * @type {any}
     * @memberof FormResultForNullAndAcceptInviteErrorOneOf
     */
    value: any | null;
}


/**
 * @export
 */
export const FormResultForNullAndAcceptInviteErrorOneOfResultEnum = {
    Ok: 'Ok'
} as const;
export type FormResultForNullAndAcceptInviteErrorOneOfResultEnum = typeof FormResultForNullAndAcceptInviteErrorOneOfResultEnum[keyof typeof FormResultForNullAndAcceptInviteErrorOneOfResultEnum];

/**
 * 
 * @export
 * @interface FormResultForNullAndAcceptInviteErrorOneOf1
 */
export interface FormResultForNullAndAcceptInviteErrorOneOf1 {
    /**
     * 
     * @type {AcceptInviteError}
     * @memberof FormResultForNullAndAcceptInviteErrorOneOf1
     */
    error: AcceptInviteError;
    /**
     * 
     * @type {string}
     * @memberof FormResultForNullAndAcceptInviteErrorOneOf1
     */
    result: FormResultForNullAndAcceptInviteErrorOneOf1ResultEnum;
}


/**
 * @export
 */
export const FormResultForNullAndAcceptInviteErrorOneOf1ResultEnum = {
    Err: 'Err'
} as const;
export type FormResultForNullAndAcceptInviteErrorOneOf1ResultEnum = typeof FormResultForNullAndAcceptInviteErrorOneOf1ResultEnum[keyof typeof FormResultForNullAndAcceptInviteErrorOneOf1ResultEnum];

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
 * Representation of the currently logged-in user.
 * @export
 * @interface Me
 */
export interface Me {
    /**
     * The user's display name.
     * @type {string}
     * @memberof Me
     */
    display_name: string;
    /**
     * The user's roles.
     * @type {Array<Role>}
     * @memberof Me
     */
    roles: Array<Role>;
    /**
     * The user's username.
     * @type {string}
     * @memberof Me
     */
    username: string;
    /**
     * The user's UUID.
     * @type {string}
     * @memberof Me
     */
    uuid: string;
}
/**
 * @type Role
 * The available roles of the manager
 * @export
 */
export type Role = RoleOneOf | RoleOneOf1 | string;
/**
 * The admin of a club. Can manage users and settings of its club
 * @export
 * @interface RoleOneOf
 */
export interface RoleOneOf {
    /**
     * New-type for the primary key of the club
     * @type {string}
     * @memberof RoleOneOf
     */
    ClubAdmin: string;
}
/**
 * A member of a club.
 * @export
 * @interface RoleOneOf1
 */
export interface RoleOneOf1 {
    /**
     * New-type for the primary key of the club
     * @type {string}
     * @memberof RoleOneOf1
     */
    ClubMember: string;
}

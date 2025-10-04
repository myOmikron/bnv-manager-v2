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
 * A single club
 * @export
 * @interface ClubSchema
 */
export interface ClubSchema {
    /**
     * The number of admins in the club
     * @type {number}
     * @memberof ClubSchema
     */
    admin_count: number;
    /**
     * The point in time the club was created
     * @type {string}
     * @memberof ClubSchema
     */
    created_at: string;
    /**
     * The number of members in the club
     * @type {number}
     * @memberof ClubSchema
     */
    member_count: number;
    /**
     * The last point in time the club was modified
     * @type {string}
     * @memberof ClubSchema
     */
    modified_at: string;
    /**
     * Name of the club
     * @type {string}
     * @memberof ClubSchema
     */
    name: string;
    /**
     * Primary domain of the club
     * @type {string}
     * @memberof ClubSchema
     */
    primary_domain: string;
    /**
     * Primary key of a club
     * @type {string}
     * @memberof ClubSchema
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
     * Email of the user
     * @type {string}
     * @memberof CreateMemberInviteRequest
     */
    email: string;
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
 * @interface PageForSimpleMemberAccountSchema
 */
export interface PageForSimpleMemberAccountSchema {
    /**
     * The page's items
     * @type {Array<SimpleMemberAccountSchema>}
     * @memberof PageForSimpleMemberAccountSchema
     */
    items: Array<SimpleMemberAccountSchema>;
    /**
     * The limit this page was requested with
     * @type {number}
     * @memberof PageForSimpleMemberAccountSchema
     */
    limit: number;
    /**
     * The offset this page was requested with
     * @type {number}
     * @memberof PageForSimpleMemberAccountSchema
     */
    offset: number;
    /**
     * The total number of items this page is a subset of
     * @type {number}
     * @memberof PageForSimpleMemberAccountSchema
     */
    total: number;
}
/**
 * Simple representation of an account.
 * @export
 * @interface SimpleMemberAccountSchema
 */
export interface SimpleMemberAccountSchema {
    /**
     * The account's display name.
     * @type {string}
     * @memberof SimpleMemberAccountSchema
     */
    display_name: string;
    /**
     * The account's email
     * @type {string}
     * @memberof SimpleMemberAccountSchema
     */
    email: string;
    /**
     * The account's username.
     * @type {string}
     * @memberof SimpleMemberAccountSchema
     */
    username: string;
    /**
     * The account's UUID.
     * @type {string}
     * @memberof SimpleMemberAccountSchema
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

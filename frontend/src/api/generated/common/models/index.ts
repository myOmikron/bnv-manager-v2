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
    /**
     * Invite has expired
     * @type {boolean}
     * @memberof AcceptInviteError
     */
    expired: boolean;
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
    /**
     * ID of the opentelemetry trace this error originated in
     * @type {string}
     * @memberof ApiErrorResponse
     */
    trace_id: string;
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
 * A club membership role.
 * @export
 * @interface ClubAdminRole
 */
export interface ClubAdminRole {
    /**
     * The club's name.
     * @type {string}
     * @memberof ClubAdminRole
     */
    club_name: string;
    /**
     * The club's UUID.
     * @type {string}
     * @memberof ClubAdminRole
     */
    club_uuid: string;
}
/**
 * A club membership role.
 * @export
 * @interface ClubMemberRole
 */
export interface ClubMemberRole {
    /**
     * The club's name.
     * @type {string}
     * @memberof ClubMemberRole
     */
    club_name: string;
    /**
     * The club's UUID.
     * @type {string}
     * @memberof ClubMemberRole
     */
    club_uuid: string;
}
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
     * @type {Roles}
     * @memberof Me
     */
    roles: Roles;
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
 * The roles of a user.
 * @export
 * @interface Roles
 */
export interface Roles {
    /**
     * The user's admin roles.
     * @type {Array<ClubAdminRole>}
     * @memberof Roles
     */
    admins: Array<ClubAdminRole>;
    /**
     * The user's membership roles
     * @type {Array<ClubMemberRole>}
     * @memberof Roles
     */
    member: Array<ClubMemberRole>;
    /**
     * Whether the user is a super admin.
     * @type {boolean}
     * @memberof Roles
     */
    super_admin: boolean;
}
/**
 * Request to update the currently logged-in user
 * @export
 * @interface SetPasswordRequest
 */
export interface SetPasswordRequest {
    /**
     * The display name of the user
     * @type {string}
     * @memberof SetPasswordRequest
     */
    password: string;
}
/**
 * Request to update the currently logged-in user
 * @export
 * @interface UpdateMeRequest
 */
export interface UpdateMeRequest {
    /**
     * The display name of the user
     * @type {string}
     * @memberof UpdateMeRequest
     */
    display_name: string;
}

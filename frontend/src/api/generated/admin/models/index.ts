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
     * Primary domain of the club
     * @type {string}
     * @memberof Club
     */
    primary_domain: string;
    /**
     * Primary key of a club
     * @type {string}
     * @memberof Club
     */
    uuid: string;
}
/**
 * Error when creating a club
 * @export
 * @interface CreateClubError
 */
export interface CreateClubError {
    /**
     * The domain is already associated with another club and can't be reused
     * @type {boolean}
     * @memberof CreateClubError
     */
    domain_already_associated: boolean;
    /**
     * Whether the club name already exists
     * @type {boolean}
     * @memberof CreateClubError
     */
    name_already_exists: boolean;
}
/**
 * Request to create a club
 * @export
 * @interface CreateClubRequest
 */
export interface CreateClubRequest {
    /**
     * Name of the club
     * @type {string}
     * @memberof CreateClubRequest
     */
    name: string;
    /**
     * Primary domain of the club
     * @type {string}
     * @memberof CreateClubRequest
     */
    primary_domain: string;
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
 * @interface CreateInviteRequestAdmin
 */
export interface CreateInviteRequestAdmin {
    /**
     * Display-name of the user
     * @type {string}
     * @memberof CreateInviteRequestAdmin
     */
    display_name: string;
    /**
     * Roles to assign to the user
     * @type {Array<Role>}
     * @memberof CreateInviteRequestAdmin
     */
    roles: Array<Role>;
    /**
     * Reserved username
     * @type {string}
     * @memberof CreateInviteRequestAdmin
     */
    username: string;
    /**
     * The point in time the invite expires
     * @type {number}
     * @memberof CreateInviteRequestAdmin
     */
    valid_days: number;
}
/**
 * Request to create an oidc provider
 * @export
 * @interface CreateOidcProvider
 */
export interface CreateOidcProvider {
    /**
     * Name of the oidc provider
     * @type {string}
     * @memberof CreateOidcProvider
     */
    name: string;
    /**
     * Redirect url of the oidc provider
     * @type {string}
     * @memberof CreateOidcProvider
     */
    redirect_uri: string;
}
/**
 * The representation of a domain
 * @export
 * @interface Domain
 */
export interface Domain {
    /**
     * The domain
     * @type {string}
     * @memberof Domain
     */
    domain: string;
    /**
     * Internal identifier of the domain
     * @type {string}
     * @memberof Domain
     */
    uuid: string;
}
/**
 * @type FormResultForClubUuidAndCreateClubError
 * A `Result` with a custom serialization
 * @export
 */
export type FormResultForClubUuidAndCreateClubError = FormResultForClubUuidAndCreateClubErrorOneOf | FormResultForClubUuidAndCreateClubErrorOneOf1;
/**
 * 
 * @export
 * @interface FormResultForClubUuidAndCreateClubErrorOneOf
 */
export interface FormResultForClubUuidAndCreateClubErrorOneOf {
    /**
     * 
     * @type {string}
     * @memberof FormResultForClubUuidAndCreateClubErrorOneOf
     */
    result: FormResultForClubUuidAndCreateClubErrorOneOfResultEnum;
    /**
     * New-type for the primary key of the club
     * @type {string}
     * @memberof FormResultForClubUuidAndCreateClubErrorOneOf
     */
    value: string;
}


/**
 * @export
 */
export const FormResultForClubUuidAndCreateClubErrorOneOfResultEnum = {
    Ok: 'Ok'
} as const;
export type FormResultForClubUuidAndCreateClubErrorOneOfResultEnum = typeof FormResultForClubUuidAndCreateClubErrorOneOfResultEnum[keyof typeof FormResultForClubUuidAndCreateClubErrorOneOfResultEnum];

/**
 * 
 * @export
 * @interface FormResultForClubUuidAndCreateClubErrorOneOf1
 */
export interface FormResultForClubUuidAndCreateClubErrorOneOf1 {
    /**
     * 
     * @type {CreateClubError}
     * @memberof FormResultForClubUuidAndCreateClubErrorOneOf1
     */
    error: CreateClubError;
    /**
     * 
     * @type {string}
     * @memberof FormResultForClubUuidAndCreateClubErrorOneOf1
     */
    result: FormResultForClubUuidAndCreateClubErrorOneOf1ResultEnum;
}


/**
 * @export
 */
export const FormResultForClubUuidAndCreateClubErrorOneOf1ResultEnum = {
    Err: 'Err'
} as const;
export type FormResultForClubUuidAndCreateClubErrorOneOf1ResultEnum = typeof FormResultForClubUuidAndCreateClubErrorOneOf1ResultEnum[keyof typeof FormResultForClubUuidAndCreateClubErrorOneOf1ResultEnum];

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
 * A single OIDC Provider
 * @export
 * @interface OidcProvider
 */
export interface OidcProvider {
    /**
     * client id of the provider
     * @type {string}
     * @memberof OidcProvider
     */
    client_id: string;
    /**
     * Secret of the provider
     * @type {string}
     * @memberof OidcProvider
     */
    client_secret: string;
    /**
     * Human-readable name
     * @type {string}
     * @memberof OidcProvider
     */
    name: string;
    /**
     * Redirect url associated with the provider
     * @type {string}
     * @memberof OidcProvider
     */
    redirect_uri: string;
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
 * @type Role
 * The available roles of the manager
 * @export
 */
export type Role = RoleOneOf | RoleOneOf1 | RoleOneOf2;
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
    club_uuid: string;
    /**
     * 
     * @type {string}
     * @memberof RoleOneOf
     */
    type: RoleOneOfTypeEnum;
}


/**
 * @export
 */
export const RoleOneOfTypeEnum = {
    ClubAdmin: 'ClubAdmin'
} as const;
export type RoleOneOfTypeEnum = typeof RoleOneOfTypeEnum[keyof typeof RoleOneOfTypeEnum];

/**
 * A member of a club.
 * @export
 * @interface RoleOneOf1
 */
export interface RoleOneOf1 {
    /**
     * UUID of the club
     * @type {string}
     * @memberof RoleOneOf1
     */
    club_uuid: string;
    /**
     * Mail of the user for that club
     * @type {string}
     * @memberof RoleOneOf1
     */
    email: string;
    /**
     * 
     * @type {string}
     * @memberof RoleOneOf1
     */
    type: RoleOneOf1TypeEnum;
}


/**
 * @export
 */
export const RoleOneOf1TypeEnum = {
    ClubMember: 'ClubMember'
} as const;
export type RoleOneOf1TypeEnum = typeof RoleOneOf1TypeEnum[keyof typeof RoleOneOf1TypeEnum];

/**
 * The super administrator. Has rights to manager clubs.
 * @export
 * @interface RoleOneOf2
 */
export interface RoleOneOf2 {
    /**
     * 
     * @type {string}
     * @memberof RoleOneOf2
     */
    type: RoleOneOf2TypeEnum;
}


/**
 * @export
 */
export const RoleOneOf2TypeEnum = {
    SuperAdmin: 'SuperAdmin'
} as const;
export type RoleOneOf2TypeEnum = typeof RoleOneOf2TypeEnum[keyof typeof RoleOneOf2TypeEnum];

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

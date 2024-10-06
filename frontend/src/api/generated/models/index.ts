/* tslint:disable */
/* eslint-disable */
/**
 * Accept the invite with a password
 * @export
 * @interface AcceptInvitePwRequest
 */
export interface AcceptInvitePwRequest {
    /**
     * The password that should be set
     * @type {string}
     * @memberof AcceptInvitePwRequest
     */
    password: string;
}
/**
 * @type AddDomainToWebsiteForm
 * The request to add a domain to a website
 * @export
 */
export type AddDomainToWebsiteForm = AddDomainToWebsiteFormOneOf;
/**
 * The request to add a domain to a website
 * @export
 * @interface AddDomainToWebsiteFormOneOf
 */
export interface AddDomainToWebsiteFormOneOf {
    /**
     * 
     * @type {string}
     * @memberof AddDomainToWebsiteFormOneOf
     */
    field: AddDomainToWebsiteFormOneOfFieldEnum;
}


/**
 * @export
 */
export const AddDomainToWebsiteFormOneOfFieldEnum = {
    Domain: 'Domain'
} as const;
export type AddDomainToWebsiteFormOneOfFieldEnum = typeof AddDomainToWebsiteFormOneOfFieldEnum[keyof typeof AddDomainToWebsiteFormOneOfFieldEnum];

/**
 * 
 * @export
 * @interface AddDomainToWebsiteFormOneOfAllOfOneOf
 */
export interface AddDomainToWebsiteFormOneOfAllOfOneOf {
    /**
     * 
     * @type {string}
     * @memberof AddDomainToWebsiteFormOneOfAllOfOneOf
     */
    field: AddDomainToWebsiteFormOneOfAllOfOneOfFieldEnum;
}


/**
 * @export
 */
export const AddDomainToWebsiteFormOneOfAllOfOneOfFieldEnum = {
    AlreadyRegistered: 'AlreadyRegistered'
} as const;
export type AddDomainToWebsiteFormOneOfAllOfOneOfFieldEnum = typeof AddDomainToWebsiteFormOneOfAllOfOneOfFieldEnum[keyof typeof AddDomainToWebsiteFormOneOfAllOfOneOfFieldEnum];

/**
 * The request to add a domain to a website
 * @export
 * @interface AddDomainToWebsiteRequest
 */
export interface AddDomainToWebsiteRequest {
    /**
     * The domain to add
     * @type {string}
     * @memberof AddDomainToWebsiteRequest
     */
    domain: string;
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
 * The request to change user information
 * @export
 * @interface ChangeMeRequest
 */
export interface ChangeMeRequest {
    /**
     * The new name
     * @type {string}
     * @memberof ChangeMeRequest
     */
    display_name?: string | null;
    /**
     * The preferred user language
     * @type {string}
     * @memberof ChangeMeRequest
     */
    preferred_lang?: string | null;
}
/**
 * The fields of the change password request
 * @export
 * @interface ChangePwErrors
 */
export interface ChangePwErrors {
    /**
     * 
     * @type {boolean}
     * @memberof ChangePwErrors
     */
    current_pw: boolean;
}
/**
 * The request to change the password
 * @export
 * @interface ChangePwRequest
 */
export interface ChangePwRequest {
    /**
     * The current password of the user
     * @type {string}
     * @memberof ChangePwRequest
     */
    current_pw: string;
    /**
     * The password that should be set
     * @type {string}
     * @memberof ChangePwRequest
     */
    new_pw: string;
}
/**
 * A list of clubs
 * @export
 * @interface ClubList
 */
export interface ClubList {
    /**
     * List of all clubs
     * @type {Array<SimpleClub>}
     * @memberof ClubList
     */
    clubs: Array<SimpleClub>;
}
/**
 * Errors that may occur during creation of a club
 * @export
 * @interface CreateClubErrors
 */
export interface CreateClubErrors {
    /**
     * Name is already in use
     * @type {boolean}
     * @memberof CreateClubErrors
     */
    name_in_use: boolean;
}
/**
 * The request to create a club
 * @export
 * @interface CreateClubRequest
 */
export interface CreateClubRequest {
    /**
     * The name of the club
     * @type {string}
     * @memberof CreateClubRequest
     */
    name: string;
}
/**
 * The errors that can occur on creation of user invites
 * @export
 * @interface CreateUserInviteErrors
 */
export interface CreateUserInviteErrors {
    /**
     * The username is already in use
     * @type {boolean}
     * @memberof CreateUserInviteErrors
     */
    username_in_use: boolean;
}
/**
 * The request when creating a user invite
 * @export
 * @interface CreateUserInviteRequestAdmin
 */
export interface CreateUserInviteRequestAdmin {
    /**
     * The display name of the new user
     * @type {string}
     * @memberof CreateUserInviteRequestAdmin
     */
    display_name: string;
    /**
     * Preferred language of the new user
     * @type {string}
     * @memberof CreateUserInviteRequestAdmin
     */
    preferred_lang: string;
    /**
     * 
     * @type {UserRoleWithClub}
     * @memberof CreateUserInviteRequestAdmin
     */
    role: UserRoleWithClub;
    /**
     * The username for the new user
     * @type {string}
     * @memberof CreateUserInviteRequestAdmin
     */
    username: string;
}
/**
 * The request when creating a user invite
 * @export
 * @interface CreateUserInviteRequestClubAdmin
 */
export interface CreateUserInviteRequestClubAdmin {
    /**
     * The display name of the new user
     * @type {string}
     * @memberof CreateUserInviteRequestClubAdmin
     */
    display_name: string;
    /**
     * Preferred language of the new user
     * @type {string}
     * @memberof CreateUserInviteRequestClubAdmin
     */
    preferred_lang: string;
    /**
     * The username for the new user
     * @type {string}
     * @memberof CreateUserInviteRequestClubAdmin
     */
    username: string;
}
/**
 * The response when creating a user invite
 * @export
 * @interface CreateUserInviteResponse
 */
export interface CreateUserInviteResponse {
    /**
     * The link of a user
     * @type {string}
     * @memberof CreateUserInviteResponse
     */
    link: string;
}
/**
 * The request to create a website
 * @export
 * @interface CreateWebsiteRequest
 */
export interface CreateWebsiteRequest {
    /**
     * The name of the website
     * @type {string}
     * @memberof CreateWebsiteRequest
     */
    name: string;
}
/**
 * @type DeployState
 * The current deploy state
 * @export
 */
export type DeployState = DeployStateOneOf | DeployStateOneOf1 | DeployStateOneOf2;
/**
 * The current state is deployed
 * @export
 * @interface DeployStateOneOf
 */
export interface DeployStateOneOf {
    /**
     * 
     * @type {string}
     * @memberof DeployStateOneOf
     */
    type: DeployStateOneOfTypeEnum;
}


/**
 * @export
 */
export const DeployStateOneOfTypeEnum = {
    Deployed: 'Deployed'
} as const;
export type DeployStateOneOfTypeEnum = typeof DeployStateOneOfTypeEnum[keyof typeof DeployStateOneOfTypeEnum];

/**
 * There are pending changes
 * @export
 * @interface DeployStateOneOf1
 */
export interface DeployStateOneOf1 {
    /**
     * 
     * @type {string}
     * @memberof DeployStateOneOf1
     */
    type: DeployStateOneOf1TypeEnum;
}


/**
 * @export
 */
export const DeployStateOneOf1TypeEnum = {
    PendingChanges: 'PendingChanges'
} as const;
export type DeployStateOneOf1TypeEnum = typeof DeployStateOneOf1TypeEnum[keyof typeof DeployStateOneOf1TypeEnum];

/**
 * Deployment failed
 * @export
 * @interface DeployStateOneOf2
 */
export interface DeployStateOneOf2 {
    /**
     * 
     * @type {string}
     * @memberof DeployStateOneOf2
     */
    type: DeployStateOneOf2TypeEnum;
}


/**
 * @export
 */
export const DeployStateOneOf2TypeEnum = {
    DeploymentFailed: 'DeploymentFailed'
} as const;
export type DeployStateOneOf2TypeEnum = typeof DeployStateOneOf2TypeEnum[keyof typeof DeployStateOneOf2TypeEnum];

/**
 * @type FormResultForCreateUserInviteResponseAndCreateUserInviteErrors
 * A `Result` with a custom serialization
 * @export
 */
export type FormResultForCreateUserInviteResponseAndCreateUserInviteErrors = FormResultForCreateUserInviteResponseAndCreateUserInviteErrorsOneOf | FormResultForCreateUserInviteResponseAndCreateUserInviteErrorsOneOf1;
/**
 * 
 * @export
 * @interface FormResultForCreateUserInviteResponseAndCreateUserInviteErrorsOneOf
 */
export interface FormResultForCreateUserInviteResponseAndCreateUserInviteErrorsOneOf {
    /**
     * 
     * @type {string}
     * @memberof FormResultForCreateUserInviteResponseAndCreateUserInviteErrorsOneOf
     */
    result: FormResultForCreateUserInviteResponseAndCreateUserInviteErrorsOneOfResultEnum;
    /**
     * 
     * @type {CreateUserInviteResponse}
     * @memberof FormResultForCreateUserInviteResponseAndCreateUserInviteErrorsOneOf
     */
    value: CreateUserInviteResponse;
}


/**
 * @export
 */
export const FormResultForCreateUserInviteResponseAndCreateUserInviteErrorsOneOfResultEnum = {
    Ok: 'Ok'
} as const;
export type FormResultForCreateUserInviteResponseAndCreateUserInviteErrorsOneOfResultEnum = typeof FormResultForCreateUserInviteResponseAndCreateUserInviteErrorsOneOfResultEnum[keyof typeof FormResultForCreateUserInviteResponseAndCreateUserInviteErrorsOneOfResultEnum];

/**
 * 
 * @export
 * @interface FormResultForCreateUserInviteResponseAndCreateUserInviteErrorsOneOf1
 */
export interface FormResultForCreateUserInviteResponseAndCreateUserInviteErrorsOneOf1 {
    /**
     * 
     * @type {CreateUserInviteErrors}
     * @memberof FormResultForCreateUserInviteResponseAndCreateUserInviteErrorsOneOf1
     */
    error: CreateUserInviteErrors;
    /**
     * 
     * @type {string}
     * @memberof FormResultForCreateUserInviteResponseAndCreateUserInviteErrorsOneOf1
     */
    result: FormResultForCreateUserInviteResponseAndCreateUserInviteErrorsOneOf1ResultEnum;
}


/**
 * @export
 */
export const FormResultForCreateUserInviteResponseAndCreateUserInviteErrorsOneOf1ResultEnum = {
    Err: 'Err'
} as const;
export type FormResultForCreateUserInviteResponseAndCreateUserInviteErrorsOneOf1ResultEnum = typeof FormResultForCreateUserInviteResponseAndCreateUserInviteErrorsOneOf1ResultEnum[keyof typeof FormResultForCreateUserInviteResponseAndCreateUserInviteErrorsOneOf1ResultEnum];

/**
 * @type FormResultForFullUserInviteAndGetUserInviteErrors
 * A `Result` with a custom serialization
 * @export
 */
export type FormResultForFullUserInviteAndGetUserInviteErrors = FormResultForFullUserInviteAndGetUserInviteErrorsOneOf | FormResultForFullUserInviteAndGetUserInviteErrorsOneOf1;
/**
 * 
 * @export
 * @interface FormResultForFullUserInviteAndGetUserInviteErrorsOneOf
 */
export interface FormResultForFullUserInviteAndGetUserInviteErrorsOneOf {
    /**
     * 
     * @type {string}
     * @memberof FormResultForFullUserInviteAndGetUserInviteErrorsOneOf
     */
    result: FormResultForFullUserInviteAndGetUserInviteErrorsOneOfResultEnum;
    /**
     * 
     * @type {FullUserInvite}
     * @memberof FormResultForFullUserInviteAndGetUserInviteErrorsOneOf
     */
    value: FullUserInvite;
}


/**
 * @export
 */
export const FormResultForFullUserInviteAndGetUserInviteErrorsOneOfResultEnum = {
    Ok: 'Ok'
} as const;
export type FormResultForFullUserInviteAndGetUserInviteErrorsOneOfResultEnum = typeof FormResultForFullUserInviteAndGetUserInviteErrorsOneOfResultEnum[keyof typeof FormResultForFullUserInviteAndGetUserInviteErrorsOneOfResultEnum];

/**
 * 
 * @export
 * @interface FormResultForFullUserInviteAndGetUserInviteErrorsOneOf1
 */
export interface FormResultForFullUserInviteAndGetUserInviteErrorsOneOf1 {
    /**
     * 
     * @type {GetUserInviteErrors}
     * @memberof FormResultForFullUserInviteAndGetUserInviteErrorsOneOf1
     */
    error: GetUserInviteErrors;
    /**
     * 
     * @type {string}
     * @memberof FormResultForFullUserInviteAndGetUserInviteErrorsOneOf1
     */
    result: FormResultForFullUserInviteAndGetUserInviteErrorsOneOf1ResultEnum;
}


/**
 * @export
 */
export const FormResultForFullUserInviteAndGetUserInviteErrorsOneOf1ResultEnum = {
    Err: 'Err'
} as const;
export type FormResultForFullUserInviteAndGetUserInviteErrorsOneOf1ResultEnum = typeof FormResultForFullUserInviteAndGetUserInviteErrorsOneOf1ResultEnum[keyof typeof FormResultForFullUserInviteAndGetUserInviteErrorsOneOf1ResultEnum];

/**
 * @type FormResultForNullAndChangePwErrors
 * A `Result` with a custom serialization
 * @export
 */
export type FormResultForNullAndChangePwErrors = FormResultForNullAndChangePwErrorsOneOf | FormResultForNullAndChangePwErrorsOneOf1;
/**
 * 
 * @export
 * @interface FormResultForNullAndChangePwErrorsOneOf
 */
export interface FormResultForNullAndChangePwErrorsOneOf {
    /**
     * 
     * @type {string}
     * @memberof FormResultForNullAndChangePwErrorsOneOf
     */
    result: FormResultForNullAndChangePwErrorsOneOfResultEnum;
    /**
     * 
     * @type {any}
     * @memberof FormResultForNullAndChangePwErrorsOneOf
     */
    value: any | null;
}


/**
 * @export
 */
export const FormResultForNullAndChangePwErrorsOneOfResultEnum = {
    Ok: 'Ok'
} as const;
export type FormResultForNullAndChangePwErrorsOneOfResultEnum = typeof FormResultForNullAndChangePwErrorsOneOfResultEnum[keyof typeof FormResultForNullAndChangePwErrorsOneOfResultEnum];

/**
 * 
 * @export
 * @interface FormResultForNullAndChangePwErrorsOneOf1
 */
export interface FormResultForNullAndChangePwErrorsOneOf1 {
    /**
     * 
     * @type {ChangePwErrors}
     * @memberof FormResultForNullAndChangePwErrorsOneOf1
     */
    error: ChangePwErrors;
    /**
     * 
     * @type {string}
     * @memberof FormResultForNullAndChangePwErrorsOneOf1
     */
    result: FormResultForNullAndChangePwErrorsOneOf1ResultEnum;
}


/**
 * @export
 */
export const FormResultForNullAndChangePwErrorsOneOf1ResultEnum = {
    Err: 'Err'
} as const;
export type FormResultForNullAndChangePwErrorsOneOf1ResultEnum = typeof FormResultForNullAndChangePwErrorsOneOf1ResultEnum[keyof typeof FormResultForNullAndChangePwErrorsOneOf1ResultEnum];

/**
 * @type FormResultForNullAndLoginErrors
 * A `Result` with a custom serialization
 * @export
 */
export type FormResultForNullAndLoginErrors = FormResultForNullAndChangePwErrorsOneOf | FormResultForNullAndLoginErrorsOneOf;
/**
 * 
 * @export
 * @interface FormResultForNullAndLoginErrorsOneOf
 */
export interface FormResultForNullAndLoginErrorsOneOf {
    /**
     * 
     * @type {LoginErrors}
     * @memberof FormResultForNullAndLoginErrorsOneOf
     */
    error: LoginErrors;
    /**
     * 
     * @type {string}
     * @memberof FormResultForNullAndLoginErrorsOneOf
     */
    result: FormResultForNullAndLoginErrorsOneOfResultEnum;
}


/**
 * @export
 */
export const FormResultForNullAndLoginErrorsOneOfResultEnum = {
    Err: 'Err'
} as const;
export type FormResultForNullAndLoginErrorsOneOfResultEnum = typeof FormResultForNullAndLoginErrorsOneOfResultEnum[keyof typeof FormResultForNullAndLoginErrorsOneOfResultEnum];

/**
 * @type FormResultForNullAndUpdateClubErrors
 * A `Result` with a custom serialization
 * @export
 */
export type FormResultForNullAndUpdateClubErrors = FormResultForNullAndChangePwErrorsOneOf | FormResultForNullAndUpdateClubErrorsOneOf;
/**
 * 
 * @export
 * @interface FormResultForNullAndUpdateClubErrorsOneOf
 */
export interface FormResultForNullAndUpdateClubErrorsOneOf {
    /**
     * 
     * @type {UpdateClubErrors}
     * @memberof FormResultForNullAndUpdateClubErrorsOneOf
     */
    error: UpdateClubErrors;
    /**
     * 
     * @type {string}
     * @memberof FormResultForNullAndUpdateClubErrorsOneOf
     */
    result: FormResultForNullAndUpdateClubErrorsOneOfResultEnum;
}


/**
 * @export
 */
export const FormResultForNullAndUpdateClubErrorsOneOfResultEnum = {
    Err: 'Err'
} as const;
export type FormResultForNullAndUpdateClubErrorsOneOfResultEnum = typeof FormResultForNullAndUpdateClubErrorsOneOfResultEnum[keyof typeof FormResultForNullAndUpdateClubErrorsOneOfResultEnum];

/**
 * @type FormResultForSingleUuidAndAddDomainToWebsiteForm
 * A `Result` with a custom serialization
 * @export
 */
export type FormResultForSingleUuidAndAddDomainToWebsiteForm = FormResultForSingleUuidAndAddDomainToWebsiteFormOneOf | FormResultForSingleUuidAndAddDomainToWebsiteFormOneOf1;
/**
 * 
 * @export
 * @interface FormResultForSingleUuidAndAddDomainToWebsiteFormOneOf
 */
export interface FormResultForSingleUuidAndAddDomainToWebsiteFormOneOf {
    /**
     * 
     * @type {string}
     * @memberof FormResultForSingleUuidAndAddDomainToWebsiteFormOneOf
     */
    result: FormResultForSingleUuidAndAddDomainToWebsiteFormOneOfResultEnum;
    /**
     * 
     * @type {SingleUuid}
     * @memberof FormResultForSingleUuidAndAddDomainToWebsiteFormOneOf
     */
    value: SingleUuid;
}


/**
 * @export
 */
export const FormResultForSingleUuidAndAddDomainToWebsiteFormOneOfResultEnum = {
    Ok: 'Ok'
} as const;
export type FormResultForSingleUuidAndAddDomainToWebsiteFormOneOfResultEnum = typeof FormResultForSingleUuidAndAddDomainToWebsiteFormOneOfResultEnum[keyof typeof FormResultForSingleUuidAndAddDomainToWebsiteFormOneOfResultEnum];

/**
 * 
 * @export
 * @interface FormResultForSingleUuidAndAddDomainToWebsiteFormOneOf1
 */
export interface FormResultForSingleUuidAndAddDomainToWebsiteFormOneOf1 {
    /**
     * 
     * @type {AddDomainToWebsiteForm}
     * @memberof FormResultForSingleUuidAndAddDomainToWebsiteFormOneOf1
     */
    error: AddDomainToWebsiteForm;
    /**
     * 
     * @type {string}
     * @memberof FormResultForSingleUuidAndAddDomainToWebsiteFormOneOf1
     */
    result: FormResultForSingleUuidAndAddDomainToWebsiteFormOneOf1ResultEnum;
}


/**
 * @export
 */
export const FormResultForSingleUuidAndAddDomainToWebsiteFormOneOf1ResultEnum = {
    Err: 'Err'
} as const;
export type FormResultForSingleUuidAndAddDomainToWebsiteFormOneOf1ResultEnum = typeof FormResultForSingleUuidAndAddDomainToWebsiteFormOneOf1ResultEnum[keyof typeof FormResultForSingleUuidAndAddDomainToWebsiteFormOneOf1ResultEnum];

/**
 * @type FormResultForSingleUuidAndCreateClubErrors
 * A `Result` with a custom serialization
 * @export
 */
export type FormResultForSingleUuidAndCreateClubErrors = FormResultForSingleUuidAndAddDomainToWebsiteFormOneOf | FormResultForSingleUuidAndCreateClubErrorsOneOf;
/**
 * 
 * @export
 * @interface FormResultForSingleUuidAndCreateClubErrorsOneOf
 */
export interface FormResultForSingleUuidAndCreateClubErrorsOneOf {
    /**
     * 
     * @type {CreateClubErrors}
     * @memberof FormResultForSingleUuidAndCreateClubErrorsOneOf
     */
    error: CreateClubErrors;
    /**
     * 
     * @type {string}
     * @memberof FormResultForSingleUuidAndCreateClubErrorsOneOf
     */
    result: FormResultForSingleUuidAndCreateClubErrorsOneOfResultEnum;
}


/**
 * @export
 */
export const FormResultForSingleUuidAndCreateClubErrorsOneOfResultEnum = {
    Err: 'Err'
} as const;
export type FormResultForSingleUuidAndCreateClubErrorsOneOfResultEnum = typeof FormResultForSingleUuidAndCreateClubErrorsOneOfResultEnum[keyof typeof FormResultForSingleUuidAndCreateClubErrorsOneOfResultEnum];

/**
 * A full representation of a club
 * @export
 * @interface FullClub
 */
export interface FullClub {
    /**
     * The users that are admins of the club
     * @type {Array<SimpleUser>}
     * @memberof FullClub
     */
    admins: Array<SimpleUser>;
    /**
     * Name of the club
     * @type {string}
     * @memberof FullClub
     */
    name: string;
    /**
     * User count associated with the club
     * @type {number}
     * @memberof FullClub
     */
    user_count: number;
    /**
     * Primary key
     * @type {string}
     * @memberof FullClub
     */
    uuid: string;
}
/**
 * The full representation for the user
 * @export
 * @interface FullUser
 */
export interface FullUser {
    /**
     * 
     * @type {string}
     * @memberof FullUser
     */
    created_at: string;
    /**
     * Used for displaying purposes
     * @type {string}
     * @memberof FullUser
     */
    display_name: string;
    /**
     * 
     * @type {string}
     * @memberof FullUser
     */
    last_login?: string;
    /**
     * Preferred language of the user
     * @type {string}
     * @memberof FullUser
     */
    preferred_lang: string;
    /**
     * 
     * @type {UserRoleWithClub}
     * @memberof FullUser
     */
    role: UserRoleWithClub;
    /**
     * The username
     * @type {string}
     * @memberof FullUser
     */
    username: string;
    /**
     * The identifier of the user
     * @type {string}
     * @memberof FullUser
     */
    uuid: string;
}
/**
 * A user invite
 * @export
 * @interface FullUserInvite
 */
export interface FullUserInvite {
    /**
     * 
     * @type {string}
     * @memberof FullUserInvite
     */
    created_at: string;
    /**
     * The name that is used for displaying purposes
     * @type {string}
     * @memberof FullUserInvite
     */
    display_name: string;
    /**
     * The preferred language of the user
     * @type {string}
     * @memberof FullUserInvite
     */
    preferred_lang: string;
    /**
     * The username
     * @type {string}
     * @memberof FullUserInvite
     */
    username: string;
    /**
     * Primary key of a user invite
     * @type {string}
     * @memberof FullUserInvite
     */
    uuid: string;
}
/**
 * The full representation of a website
 * @export
 * @interface FullWebsite
 */
export interface FullWebsite {
    /**
     * 
     * @type {string}
     * @memberof FullWebsite
     */
    created_at: string;
    /**
     * 
     * @type {DeployState}
     * @memberof FullWebsite
     */
    deploy_state: DeployState;
    /**
     * A list of domains for this website
     * @type {Array<FullWebsiteDomain>}
     * @memberof FullWebsite
     */
    domains: Array<FullWebsiteDomain>;
    /**
     * 
     * @type {string}
     * @memberof FullWebsite
     */
    last_deployment?: string;
    /**
     * Descriptive name of the website
     * @type {string}
     * @memberof FullWebsite
     */
    name: string;
    /**
     * The unique key of a website
     * @type {string}
     * @memberof FullWebsite
     */
    uuid: string;
}
/**
 * The full representation of a domain that is attached to a website
 * @export
 * @interface FullWebsiteDomain
 */
export interface FullWebsiteDomain {
    /**
     * The attached domain
     * @type {string}
     * @memberof FullWebsiteDomain
     */
    domain: string;
    /**
     * The identifier of a specific domain
     * @type {string}
     * @memberof FullWebsiteDomain
     */
    uuid: string;
}
/**
 * The errors that may occur while retrieving an invitation
 * @export
 * @interface GetUserInviteErrors
 */
export interface GetUserInviteErrors {
    /**
     * The invite was already used or invalid in the first place
     * @type {boolean}
     * @memberof GetUserInviteErrors
     */
    invite_invalid: boolean;
}
/**
 * A list of websites
 * @export
 * @interface ListWebsites
 */
export interface ListWebsites {
    /**
     * The list of websites
     * @type {Array<SimpleWebsite>}
     * @memberof ListWebsites
     */
    websites: Array<SimpleWebsite>;
}
/**
 * The request for local authentication
 * @export
 * @interface LoginErrors
 */
export interface LoginErrors {
    /**
     * Login has failed
     * @type {boolean}
     * @memberof LoginErrors
     */
    login_failed: boolean;
}
/**
 * The request for local authentication
 * @export
 * @interface LoginRequest
 */
export interface LoginRequest {
    /**
     * The password for the user
     * @type {string}
     * @memberof LoginRequest
     */
    password: string;
    /**
     * The username that is used for logging in
     * @type {string}
     * @memberof LoginRequest
     */
    username: string;
}
/**
 * A simple representation of a club
 * @export
 * @interface SimpleClub
 */
export interface SimpleClub {
    /**
     * Name of the club
     * @type {string}
     * @memberof SimpleClub
     */
    name: string;
    /**
     * User count associated with the club
     * @type {number}
     * @memberof SimpleClub
     */
    user_count: number;
    /**
     * Primary key
     * @type {string}
     * @memberof SimpleClub
     */
    uuid: string;
}
/**
 * The simple representation for the user
 * @export
 * @interface SimpleUser
 */
export interface SimpleUser {
    /**
     * Used for displaying purposes
     * @type {string}
     * @memberof SimpleUser
     */
    display_name: string;
    /**
     * 
     * @type {UserRole}
     * @memberof SimpleUser
     */
    role: UserRole;
    /**
     * The username of the user
     * @type {string}
     * @memberof SimpleUser
     */
    username: string;
    /**
     * The identifier of the user
     * @type {string}
     * @memberof SimpleUser
     */
    uuid: string;
}
/**
 * The simple representation of a website
 * @export
 * @interface SimpleWebsite
 */
export interface SimpleWebsite {
    /**
     * 
     * @type {string}
     * @memberof SimpleWebsite
     */
    created_at: string;
    /**
     * 
     * @type {DeployState}
     * @memberof SimpleWebsite
     */
    deploy_state: DeployState;
    /**
     * 
     * @type {string}
     * @memberof SimpleWebsite
     */
    last_deployment?: string;
    /**
     * Descriptive name of the website
     * @type {string}
     * @memberof SimpleWebsite
     */
    name: string;
    /**
     * The unique key of a website
     * @type {string}
     * @memberof SimpleWebsite
     */
    uuid: string;
}
/**
 * A single uuid wrapped in a struct
 * @export
 * @interface SingleUuid
 */
export interface SingleUuid {
    /**
     * 
     * @type {string}
     * @memberof SingleUuid
     */
    uuid: string;
}
/**
 * Errors that may occur in an update club request
 * @export
 * @interface UpdateClubErrors
 */
export interface UpdateClubErrors {
    /**
     * The new name is already in use
     * @type {boolean}
     * @memberof UpdateClubErrors
     */
    name_in_use: boolean;
}
/**
 * The request to update a club
 * @export
 * @interface UpdateClubRequest
 */
export interface UpdateClubRequest {
    /**
     * The name of the club
     * @type {string}
     * @memberof UpdateClubRequest
     */
    name?: string | null;
}
/**
 * The request to update websites
 * @export
 * @interface UpdateWebsiteRequest
 */
export interface UpdateWebsiteRequest {
    /**
     * The name of the website
     * @type {string}
     * @memberof UpdateWebsiteRequest
     */
    name: string;
}

/**
 * The role of a user
 * @export
 */
export const UserRole = {
    Administrator: 'Administrator',
    ClubAdmin: 'ClubAdmin',
    User: 'User'
} as const;
export type UserRole = typeof UserRole[keyof typeof UserRole];

/**
 * @type UserRoleWithClub
 * the user role with the corresponding club associated to it
 * @export
 */
export type UserRoleWithClub = UserRoleWithClubOneOf | UserRoleWithClubOneOf1 | UserRoleWithClubOneOf2;
/**
 * 
 * @export
 * @interface UserRoleWithClubOneOf
 */
export interface UserRoleWithClubOneOf {
    /**
     * 
     * @type {string}
     * @memberof UserRoleWithClubOneOf
     */
    role: UserRoleWithClubOneOfRoleEnum;
}


/**
 * @export
 */
export const UserRoleWithClubOneOfRoleEnum = {
    Administrator: 'Administrator'
} as const;
export type UserRoleWithClubOneOfRoleEnum = typeof UserRoleWithClubOneOfRoleEnum[keyof typeof UserRoleWithClubOneOfRoleEnum];

/**
 * 
 * @export
 * @interface UserRoleWithClubOneOf1
 */
export interface UserRoleWithClubOneOf1 {
    /**
     * 
     * @type {string}
     * @memberof UserRoleWithClubOneOf1
     */
    club: string;
    /**
     * 
     * @type {string}
     * @memberof UserRoleWithClubOneOf1
     */
    role: UserRoleWithClubOneOf1RoleEnum;
}


/**
 * @export
 */
export const UserRoleWithClubOneOf1RoleEnum = {
    ClubAdmin: 'ClubAdmin'
} as const;
export type UserRoleWithClubOneOf1RoleEnum = typeof UserRoleWithClubOneOf1RoleEnum[keyof typeof UserRoleWithClubOneOf1RoleEnum];

/**
 * 
 * @export
 * @interface UserRoleWithClubOneOf2
 */
export interface UserRoleWithClubOneOf2 {
    /**
     * 
     * @type {string}
     * @memberof UserRoleWithClubOneOf2
     */
    club: string;
    /**
     * 
     * @type {string}
     * @memberof UserRoleWithClubOneOf2
     */
    role: UserRoleWithClubOneOf2RoleEnum;
}


/**
 * @export
 */
export const UserRoleWithClubOneOf2RoleEnum = {
    User: 'User'
} as const;
export type UserRoleWithClubOneOf2RoleEnum = typeof UserRoleWithClubOneOf2RoleEnum[keyof typeof UserRoleWithClubOneOf2RoleEnum];


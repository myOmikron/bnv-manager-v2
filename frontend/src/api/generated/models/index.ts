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
 * 
 * @export
 * @interface AdminAccount
 */
export interface AdminAccount {
    /**
     * 
     * @type {string}
     * @memberof AdminAccount
     */
    created_at: string;
    /**
     * 
     * @type {boolean}
     * @memberof AdminAccount
     */
    disabled: boolean;
    /**
     * 
     * @type {string}
     * @memberof AdminAccount
     */
    display_name: string;
    /**
     * 
     * @type {string}
     * @memberof AdminAccount
     */
    username: string;
    /**
     * 
     * @type {string}
     * @memberof AdminAccount
     */
    uuid: string;
}
/**
 * 
 * @export
 * @interface AdminCreateInviteError
 */
export interface AdminCreateInviteError {
    /**
     * 
     * @type {Array<string>}
     * @memberof AdminCreateInviteError
     */
    invalid_clubs: Array<string>;
    /**
     * 
     * @type {boolean}
     * @memberof AdminCreateInviteError
     */
    username_already_occupied: boolean;
    /**
     * 
     * @type {boolean}
     * @memberof AdminCreateInviteError
     */
    valid_days_too_small: boolean;
}
/**
 * 
 * @export
 * @interface AdminCreateInviteRequest
 */
export interface AdminCreateInviteRequest {
    /**
     * 
     * @type {string}
     * @memberof AdminCreateInviteRequest
     */
    display_name: string;
    /**
     * 
     * @type {Permissions}
     * @memberof AdminCreateInviteRequest
     */
    permissions: Permissions;
    /**
     * 
     * @type {string}
     * @memberof AdminCreateInviteRequest
     */
    username: string;
    /**
     * 
     * @type {number}
     * @memberof AdminCreateInviteRequest
     */
    valid_days: number;
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
 * @interface CreateClubRequest
 */
export interface CreateClubRequest {
    /**
     * 
     * @type {string}
     * @memberof CreateClubRequest
     */
    name: string;
}
/**
 * 
 * @export
 * @interface CreateClubResponseError
 */
export interface CreateClubResponseError {
    /**
     * 
     * @type {boolean}
     * @memberof CreateClubResponseError
     */
    name_already_occupied: boolean;
}
/**
 * @type FormResultForInviteResponseAndAdminCreateInviteError
 * A `Result` with a custom serialization
 * @export
 */
export type FormResultForInviteResponseAndAdminCreateInviteError = FormResultForInviteResponseAndAdminCreateInviteErrorOneOf | FormResultForInviteResponseAndAdminCreateInviteErrorOneOf1;
/**
 * 
 * @export
 * @interface FormResultForInviteResponseAndAdminCreateInviteErrorOneOf
 */
export interface FormResultForInviteResponseAndAdminCreateInviteErrorOneOf {
    /**
     * 
     * @type {string}
     * @memberof FormResultForInviteResponseAndAdminCreateInviteErrorOneOf
     */
    result: FormResultForInviteResponseAndAdminCreateInviteErrorOneOfResultEnum;
    /**
     * 
     * @type {InviteResponse}
     * @memberof FormResultForInviteResponseAndAdminCreateInviteErrorOneOf
     */
    value: InviteResponse;
}


/**
 * @export
 */
export const FormResultForInviteResponseAndAdminCreateInviteErrorOneOfResultEnum = {
    Ok: 'Ok'
} as const;
export type FormResultForInviteResponseAndAdminCreateInviteErrorOneOfResultEnum = typeof FormResultForInviteResponseAndAdminCreateInviteErrorOneOfResultEnum[keyof typeof FormResultForInviteResponseAndAdminCreateInviteErrorOneOfResultEnum];

/**
 * 
 * @export
 * @interface FormResultForInviteResponseAndAdminCreateInviteErrorOneOf1
 */
export interface FormResultForInviteResponseAndAdminCreateInviteErrorOneOf1 {
    /**
     * 
     * @type {AdminCreateInviteError}
     * @memberof FormResultForInviteResponseAndAdminCreateInviteErrorOneOf1
     */
    error: AdminCreateInviteError;
    /**
     * 
     * @type {string}
     * @memberof FormResultForInviteResponseAndAdminCreateInviteErrorOneOf1
     */
    result: FormResultForInviteResponseAndAdminCreateInviteErrorOneOf1ResultEnum;
}


/**
 * @export
 */
export const FormResultForInviteResponseAndAdminCreateInviteErrorOneOf1ResultEnum = {
    Err: 'Err'
} as const;
export type FormResultForInviteResponseAndAdminCreateInviteErrorOneOf1ResultEnum = typeof FormResultForInviteResponseAndAdminCreateInviteErrorOneOf1ResultEnum[keyof typeof FormResultForInviteResponseAndAdminCreateInviteErrorOneOf1ResultEnum];

/**
 * @type FormResultForNullAndLoginResponse
 * A `Result` with a custom serialization
 * @export
 */
export type FormResultForNullAndLoginResponse = FormResultForNullAndLoginResponseOneOf | FormResultForNullAndLoginResponseOneOf1;
/**
 * 
 * @export
 * @interface FormResultForNullAndLoginResponseOneOf
 */
export interface FormResultForNullAndLoginResponseOneOf {
    /**
     * 
     * @type {string}
     * @memberof FormResultForNullAndLoginResponseOneOf
     */
    result: FormResultForNullAndLoginResponseOneOfResultEnum;
    /**
     * 
     * @type {Null}
     * @memberof FormResultForNullAndLoginResponseOneOf
     */
    value: Null;
}


/**
 * @export
 */
export const FormResultForNullAndLoginResponseOneOfResultEnum = {
    Ok: 'Ok'
} as const;
export type FormResultForNullAndLoginResponseOneOfResultEnum = typeof FormResultForNullAndLoginResponseOneOfResultEnum[keyof typeof FormResultForNullAndLoginResponseOneOfResultEnum];

/**
 * 
 * @export
 * @interface FormResultForNullAndLoginResponseOneOf1
 */
export interface FormResultForNullAndLoginResponseOneOf1 {
    /**
     * 
     * @type {LoginResponse}
     * @memberof FormResultForNullAndLoginResponseOneOf1
     */
    error: LoginResponse;
    /**
     * 
     * @type {string}
     * @memberof FormResultForNullAndLoginResponseOneOf1
     */
    result: FormResultForNullAndLoginResponseOneOf1ResultEnum;
}


/**
 * @export
 */
export const FormResultForNullAndLoginResponseOneOf1ResultEnum = {
    Err: 'Err'
} as const;
export type FormResultForNullAndLoginResponseOneOf1ResultEnum = typeof FormResultForNullAndLoginResponseOneOf1ResultEnum[keyof typeof FormResultForNullAndLoginResponseOneOf1ResultEnum];

/**
 * @type FormResultForSingleUuidAndCreateClubResponseError
 * A `Result` with a custom serialization
 * @export
 */
export type FormResultForSingleUuidAndCreateClubResponseError = FormResultForSingleUuidAndCreateClubResponseErrorOneOf | FormResultForSingleUuidAndCreateClubResponseErrorOneOf1;
/**
 * 
 * @export
 * @interface FormResultForSingleUuidAndCreateClubResponseErrorOneOf
 */
export interface FormResultForSingleUuidAndCreateClubResponseErrorOneOf {
    /**
     * 
     * @type {string}
     * @memberof FormResultForSingleUuidAndCreateClubResponseErrorOneOf
     */
    result: FormResultForSingleUuidAndCreateClubResponseErrorOneOfResultEnum;
    /**
     * 
     * @type {SingleUuid}
     * @memberof FormResultForSingleUuidAndCreateClubResponseErrorOneOf
     */
    value: SingleUuid;
}


/**
 * @export
 */
export const FormResultForSingleUuidAndCreateClubResponseErrorOneOfResultEnum = {
    Ok: 'Ok'
} as const;
export type FormResultForSingleUuidAndCreateClubResponseErrorOneOfResultEnum = typeof FormResultForSingleUuidAndCreateClubResponseErrorOneOfResultEnum[keyof typeof FormResultForSingleUuidAndCreateClubResponseErrorOneOfResultEnum];

/**
 * 
 * @export
 * @interface FormResultForSingleUuidAndCreateClubResponseErrorOneOf1
 */
export interface FormResultForSingleUuidAndCreateClubResponseErrorOneOf1 {
    /**
     * 
     * @type {CreateClubResponseError}
     * @memberof FormResultForSingleUuidAndCreateClubResponseErrorOneOf1
     */
    error: CreateClubResponseError;
    /**
     * 
     * @type {string}
     * @memberof FormResultForSingleUuidAndCreateClubResponseErrorOneOf1
     */
    result: FormResultForSingleUuidAndCreateClubResponseErrorOneOf1ResultEnum;
}


/**
 * @export
 */
export const FormResultForSingleUuidAndCreateClubResponseErrorOneOf1ResultEnum = {
    Err: 'Err'
} as const;
export type FormResultForSingleUuidAndCreateClubResponseErrorOneOf1ResultEnum = typeof FormResultForSingleUuidAndCreateClubResponseErrorOneOf1ResultEnum[keyof typeof FormResultForSingleUuidAndCreateClubResponseErrorOneOf1ResultEnum];

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
 * 
 * @export
 * @interface InviteResponse
 */
export interface InviteResponse {
    /**
     * 
     * @type {string}
     * @memberof InviteResponse
     */
    link: string;
}
/**
 * 
 * @export
 * @interface LoginRequest
 */
export interface LoginRequest {
    /**
     * 
     * @type {string}
     * @memberof LoginRequest
     */
    password: string;
    /**
     * 
     * @type {string}
     * @memberof LoginRequest
     */
    username: string;
}
/**
 * 
 * @export
 * @interface LoginResponse
 */
export interface LoginResponse {
    /**
     * 
     * @type {boolean}
     * @memberof LoginResponse
     */
    username_or_password: boolean;
}
/**
 * 
 * @export
 * @interface Me
 */
export interface Me {
    /**
     * 
     * @type {string}
     * @memberof Me
     */
    display_name: string;
    /**
     * 
     * @type {Permissions}
     * @memberof Me
     */
    permissions: Permissions;
    /**
     * 
     * @type {string}
     * @memberof Me
     */
    username: string;
    /**
     * 
     * @type {string}
     * @memberof Me
     */
    uuid: string;
}
/**
 * Permissions of a session
 * @export
 * @interface Permissions
 */
export interface Permissions {
    /**
     * User is admin
     * @type {boolean}
     * @memberof Permissions
     */
    admin: boolean;
    /**
     * The clubs an account is admin in
     * @type {Array<string>}
     * @memberof Permissions
     */
    club_admin: Array<string>;
    /**
     * The clubs an account is user in
     * @type {Array<string>}
     * @memberof Permissions
     */
    club_user: Array<string>;
}
/**
 * 
 * @export
 * @interface SimpleClub
 */
export interface SimpleClub {
    /**
     * 
     * @type {string}
     * @memberof SimpleClub
     */
    created_at: string;
    /**
     * 
     * @type {string}
     * @memberof SimpleClub
     */
    name: string;
    /**
     * 
     * @type {string}
     * @memberof SimpleClub
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

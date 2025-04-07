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
 */
export const AdminCreateInviteError = {
    UsernameAlreadyOccupied: 'UsernameAlreadyOccupied'
} as const;
export type AdminCreateInviteError = typeof AdminCreateInviteError[keyof typeof AdminCreateInviteError];

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
     * @type {string}
     * @memberof AdminCreateInviteRequest
     */
    username: string;
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
 * @type FormResultForSingleUuidAndAdminCreateInviteError
 * A `Result` with a custom serialization
 * @export
 */
export type FormResultForSingleUuidAndAdminCreateInviteError = FormResultForSingleUuidAndAdminCreateInviteErrorOneOf | FormResultForSingleUuidAndAdminCreateInviteErrorOneOf1;
/**
 * 
 * @export
 * @interface FormResultForSingleUuidAndAdminCreateInviteErrorOneOf
 */
export interface FormResultForSingleUuidAndAdminCreateInviteErrorOneOf {
    /**
     * 
     * @type {string}
     * @memberof FormResultForSingleUuidAndAdminCreateInviteErrorOneOf
     */
    result: FormResultForSingleUuidAndAdminCreateInviteErrorOneOfResultEnum;
    /**
     * 
     * @type {SingleUuid}
     * @memberof FormResultForSingleUuidAndAdminCreateInviteErrorOneOf
     */
    value: SingleUuid;
}


/**
 * @export
 */
export const FormResultForSingleUuidAndAdminCreateInviteErrorOneOfResultEnum = {
    Ok: 'Ok'
} as const;
export type FormResultForSingleUuidAndAdminCreateInviteErrorOneOfResultEnum = typeof FormResultForSingleUuidAndAdminCreateInviteErrorOneOfResultEnum[keyof typeof FormResultForSingleUuidAndAdminCreateInviteErrorOneOfResultEnum];

/**
 * 
 * @export
 * @interface FormResultForSingleUuidAndAdminCreateInviteErrorOneOf1
 */
export interface FormResultForSingleUuidAndAdminCreateInviteErrorOneOf1 {
    /**
     * 
     * @type {AdminCreateInviteError}
     * @memberof FormResultForSingleUuidAndAdminCreateInviteErrorOneOf1
     */
    error: AdminCreateInviteError;
    /**
     * 
     * @type {string}
     * @memberof FormResultForSingleUuidAndAdminCreateInviteErrorOneOf1
     */
    result: FormResultForSingleUuidAndAdminCreateInviteErrorOneOf1ResultEnum;
}


/**
 * @export
 */
export const FormResultForSingleUuidAndAdminCreateInviteErrorOneOf1ResultEnum = {
    Err: 'Err'
} as const;
export type FormResultForSingleUuidAndAdminCreateInviteErrorOneOf1ResultEnum = typeof FormResultForSingleUuidAndAdminCreateInviteErrorOneOf1ResultEnum[keyof typeof FormResultForSingleUuidAndAdminCreateInviteErrorOneOf1ResultEnum];

/**
 * @type FormResultForSingleUuidAndCreateClubResponseError
 * A `Result` with a custom serialization
 * @export
 */
export type FormResultForSingleUuidAndCreateClubResponseError = FormResultForSingleUuidAndAdminCreateInviteErrorOneOf | FormResultForSingleUuidAndCreateClubResponseErrorOneOf;
/**
 * 
 * @export
 * @interface FormResultForSingleUuidAndCreateClubResponseErrorOneOf
 */
export interface FormResultForSingleUuidAndCreateClubResponseErrorOneOf {
    /**
     * 
     * @type {CreateClubResponseError}
     * @memberof FormResultForSingleUuidAndCreateClubResponseErrorOneOf
     */
    error: CreateClubResponseError;
    /**
     * 
     * @type {string}
     * @memberof FormResultForSingleUuidAndCreateClubResponseErrorOneOf
     */
    result: FormResultForSingleUuidAndCreateClubResponseErrorOneOfResultEnum;
}


/**
 * @export
 */
export const FormResultForSingleUuidAndCreateClubResponseErrorOneOfResultEnum = {
    Err: 'Err'
} as const;
export type FormResultForSingleUuidAndCreateClubResponseErrorOneOfResultEnum = typeof FormResultForSingleUuidAndCreateClubResponseErrorOneOfResultEnum[keyof typeof FormResultForSingleUuidAndCreateClubResponseErrorOneOfResultEnum];

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
     * @type {SessionPermissions}
     * @memberof Me
     */
    permissions: SessionPermissions;
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
 * @interface SessionPermissions
 */
export interface SessionPermissions {
    /**
     * User is admin
     * @type {boolean}
     * @memberof SessionPermissions
     */
    admin: boolean;
    /**
     * The clubs an account is admin in
     * @type {Array<string>}
     * @memberof SessionPermissions
     */
    club_admin: Array<string>;
    /**
     * The clubs an account is user in
     * @type {Array<string>}
     * @memberof SessionPermissions
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

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

/* tslint:disable */
/* eslint-disable */
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
 * @type ChangePwFormFields
 * The fields of the change password request
 * @export
 */
export type ChangePwFormFields = ChangePwFormFieldsOneOf;
/**
 * The fields of the change password request
 * @export
 * @interface ChangePwFormFieldsOneOf
 */
export interface ChangePwFormFieldsOneOf {
    /**
     * 
     * @type {string}
     * @memberof ChangePwFormFieldsOneOf
     */
    error: ChangePwFormFieldsOneOfErrorEnum;
    /**
     * 
     * @type {string}
     * @memberof ChangePwFormFieldsOneOf
     */
    field: ChangePwFormFieldsOneOfFieldEnum;
}


/**
 * @export
 */
export const ChangePwFormFieldsOneOfErrorEnum = {
    Incorrect: 'Incorrect'
} as const;
export type ChangePwFormFieldsOneOfErrorEnum = typeof ChangePwFormFieldsOneOfErrorEnum[keyof typeof ChangePwFormFieldsOneOfErrorEnum];

/**
 * @export
 */
export const ChangePwFormFieldsOneOfFieldEnum = {
    CurrentPw: 'CurrentPw'
} as const;
export type ChangePwFormFieldsOneOfFieldEnum = typeof ChangePwFormFieldsOneOfFieldEnum[keyof typeof ChangePwFormFieldsOneOfFieldEnum];

/**
 * 
 * @export
 * @interface ChangePwFormFieldsOneOfAllOfOneOf
 */
export interface ChangePwFormFieldsOneOfAllOfOneOf {
    /**
     * 
     * @type {string}
     * @memberof ChangePwFormFieldsOneOfAllOfOneOf
     */
    error: ChangePwFormFieldsOneOfAllOfOneOfErrorEnum;
}


/**
 * @export
 */
export const ChangePwFormFieldsOneOfAllOfOneOfErrorEnum = {
    Incorrect: 'Incorrect'
} as const;
export type ChangePwFormFieldsOneOfAllOfOneOfErrorEnum = typeof ChangePwFormFieldsOneOfAllOfOneOfErrorEnum[keyof typeof ChangePwFormFieldsOneOfAllOfOneOfErrorEnum];

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
 * The result from a dns query
 * @export
 * @interface DnsQueryResult
 */
export interface DnsQueryResult {
    /**
     * 
     * @type {ResolveResult}
     * @memberof DnsQueryResult
     */
    result: ResolveResult;
    /**
     * The uuid of the domain
     * @type {string}
     * @memberof DnsQueryResult
     */
    uuid: string;
}
/**
 * The response that should be used for inform the user about errors in the form
 * @export
 * @interface FormErrorForAddDomainToWebsiteForm
 */
export interface FormErrorForAddDomainToWebsiteForm {
    /**
     * The errors that occurred
     * @type {Array<FormFieldErrorForAddDomainToWebsiteForm>}
     * @memberof FormErrorForAddDomainToWebsiteForm
     */
    errors: Array<FormFieldErrorForAddDomainToWebsiteForm>;
}
/**
 * The response that should be used for inform the user about errors in the form
 * @export
 * @interface FormErrorForChangePwFormFields
 */
export interface FormErrorForChangePwFormFields {
    /**
     * The errors that occurred
     * @type {Array<FormFieldErrorForChangePwFormFields>}
     * @memberof FormErrorForChangePwFormFields
     */
    errors: Array<FormFieldErrorForChangePwFormFields>;
}
/**
 * An error in a form field
 * @export
 * @interface FormFieldErrorForAddDomainToWebsiteForm
 */
export interface FormFieldErrorForAddDomainToWebsiteForm {
    /**
     * 
     * @type {AddDomainToWebsiteForm}
     * @memberof FormFieldErrorForAddDomainToWebsiteForm
     */
    field: AddDomainToWebsiteForm;
}
/**
 * An error in a form field
 * @export
 * @interface FormFieldErrorForChangePwFormFields
 */
export interface FormFieldErrorForChangePwFormFields {
    /**
     * 
     * @type {ChangePwFormFields}
     * @memberof FormFieldErrorForChangePwFormFields
     */
    field: ChangePwFormFields;
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
 * The result of a resolver
 * @export
 * @interface ResolveResult
 */
export interface ResolveResult {
    /**
     * Ipv4 address
     * @type {string}
     * @memberof ResolveResult
     */
    ipv4?: string | null;
    /**
     * Ipv6 address
     * @type {string}
     * @memberof ResolveResult
     */
    ipv6?: string | null;
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
 * a uuid
 * @export
 * @interface UuidSchema
 */
export interface UuidSchema {
    /**
     * The uuid
     * @type {string}
     * @memberof UuidSchema
     */
    uuid: string;
}
/**
 * @type WebconfUpdateResult
 * The result of a webconf update request
 * @export
 */
export type WebconfUpdateResult = WebconfUpdateResultOneOf | WebconfUpdateResultOneOf1;
/**
 * 
 * @export
 * @interface WebconfUpdateResultOneOf
 */
export interface WebconfUpdateResultOneOf {
    /**
     * 
     * @type {string}
     * @memberof WebconfUpdateResultOneOf
     */
    res: WebconfUpdateResultOneOfResEnum;
}


/**
 * @export
 */
export const WebconfUpdateResultOneOfResEnum = {
    Success: 'Success'
} as const;
export type WebconfUpdateResultOneOfResEnum = typeof WebconfUpdateResultOneOfResEnum[keyof typeof WebconfUpdateResultOneOfResEnum];

/**
 * 
 * @export
 * @interface WebconfUpdateResultOneOf1
 */
export interface WebconfUpdateResultOneOf1 {
    /**
     * 
     * @type {string}
     * @memberof WebconfUpdateResultOneOf1
     */
    res: WebconfUpdateResultOneOf1ResEnum;
}


/**
 * @export
 */
export const WebconfUpdateResultOneOf1ResEnum = {
    Fail: 'Fail'
} as const;
export type WebconfUpdateResultOneOf1ResEnum = typeof WebconfUpdateResultOneOf1ResEnum[keyof typeof WebconfUpdateResultOneOf1ResEnum];

/**
 * Websocket messages that originate from the client
 * @export
 * @interface WsClientMsg
 */
export interface WsClientMsg {
}
/**
 * @type WsServerMsg
 * Websocket messages that originate from the server
 * @export
 */
export type WsServerMsg = WsServerMsgOneOf | WsServerMsgOneOf1 | WsServerMsgOneOf2;
/**
 * Deployment state has updated
 * @export
 * @interface WsServerMsgOneOf
 */
export interface WsServerMsgOneOf {
    /**
     * 
     * @type {WebconfUpdateResult}
     * @memberof WsServerMsgOneOf
     */
    state: WebconfUpdateResult;
    /**
     * The task uuid
     * @type {string}
     * @memberof WsServerMsgOneOf
     */
    task: string;
    /**
     * 
     * @type {string}
     * @memberof WsServerMsgOneOf
     */
    type: WsServerMsgOneOfTypeEnum;
}


/**
 * @export
 */
export const WsServerMsgOneOfTypeEnum = {
    DeployUpdate: 'DeployUpdate'
} as const;
export type WsServerMsgOneOfTypeEnum = typeof WsServerMsgOneOfTypeEnum[keyof typeof WsServerMsgOneOfTypeEnum];

/**
 * DNS query update
 * @export
 * @interface WsServerMsgOneOf1
 */
export interface WsServerMsgOneOf1 {
    /**
     * 
     * @type {DnsQueryResult}
     * @memberof WsServerMsgOneOf1
     */
    result: DnsQueryResult;
    /**
     * The task uuid
     * @type {string}
     * @memberof WsServerMsgOneOf1
     */
    task: string;
    /**
     * 
     * @type {string}
     * @memberof WsServerMsgOneOf1
     */
    type: WsServerMsgOneOf1TypeEnum;
}


/**
 * @export
 */
export const WsServerMsgOneOf1TypeEnum = {
    DnsUpdate: 'DnsUpdate'
} as const;
export type WsServerMsgOneOf1TypeEnum = typeof WsServerMsgOneOf1TypeEnum[keyof typeof WsServerMsgOneOf1TypeEnum];

/**
 * DNS task finished
 * @export
 * @interface WsServerMsgOneOf2
 */
export interface WsServerMsgOneOf2 {
    /**
     * The task uuid
     * @type {string}
     * @memberof WsServerMsgOneOf2
     */
    task: string;
    /**
     * 
     * @type {string}
     * @memberof WsServerMsgOneOf2
     */
    type: WsServerMsgOneOf2TypeEnum;
}


/**
 * @export
 */
export const WsServerMsgOneOf2TypeEnum = {
    DnsFinished: 'DnsFinished'
} as const;
export type WsServerMsgOneOf2TypeEnum = typeof WsServerMsgOneOf2TypeEnum[keyof typeof WsServerMsgOneOf2TypeEnum];


/* tslint:disable */
/* eslint-disable */
/**
 * Frontend
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v0.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { mapValues } from '../runtime';
import type { ChangePwFormFields } from './ChangePwFormFields';
import {
    ChangePwFormFieldsFromJSON,
    ChangePwFormFieldsFromJSONTyped,
    ChangePwFormFieldsToJSON,
} from './ChangePwFormFields';

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
 * Check if a given object implements the FormFieldErrorForChangePwFormFields interface.
 */
export function instanceOfFormFieldErrorForChangePwFormFields(value: object): value is FormFieldErrorForChangePwFormFields {
    if (!('field' in value) || value['field'] === undefined) return false;
    return true;
}

export function FormFieldErrorForChangePwFormFieldsFromJSON(json: any): FormFieldErrorForChangePwFormFields {
    return FormFieldErrorForChangePwFormFieldsFromJSONTyped(json, false);
}

export function FormFieldErrorForChangePwFormFieldsFromJSONTyped(json: any, ignoreDiscriminator: boolean): FormFieldErrorForChangePwFormFields {
    if (json == null) {
        return json;
    }
    return {
        
        'field': ChangePwFormFieldsFromJSON(json['field']),
    };
}

export function FormFieldErrorForChangePwFormFieldsToJSON(value?: FormFieldErrorForChangePwFormFields | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'field': ChangePwFormFieldsToJSON(value['field']),
    };
}

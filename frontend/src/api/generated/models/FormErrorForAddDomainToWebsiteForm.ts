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
import type { FormFieldErrorForAddDomainToWebsiteForm } from './FormFieldErrorForAddDomainToWebsiteForm';
import {
    FormFieldErrorForAddDomainToWebsiteFormFromJSON,
    FormFieldErrorForAddDomainToWebsiteFormFromJSONTyped,
    FormFieldErrorForAddDomainToWebsiteFormToJSON,
} from './FormFieldErrorForAddDomainToWebsiteForm';

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
 * Check if a given object implements the FormErrorForAddDomainToWebsiteForm interface.
 */
export function instanceOfFormErrorForAddDomainToWebsiteForm(value: object): value is FormErrorForAddDomainToWebsiteForm {
    if (!('errors' in value) || value['errors'] === undefined) return false;
    return true;
}

export function FormErrorForAddDomainToWebsiteFormFromJSON(json: any): FormErrorForAddDomainToWebsiteForm {
    return FormErrorForAddDomainToWebsiteFormFromJSONTyped(json, false);
}

export function FormErrorForAddDomainToWebsiteFormFromJSONTyped(json: any, ignoreDiscriminator: boolean): FormErrorForAddDomainToWebsiteForm {
    if (json == null) {
        return json;
    }
    return {
        
        'errors': ((json['errors'] as Array<any>).map(FormFieldErrorForAddDomainToWebsiteFormFromJSON)),
    };
}

export function FormErrorForAddDomainToWebsiteFormToJSON(value?: FormErrorForAddDomainToWebsiteForm | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'errors': ((value['errors'] as Array<any>).map(FormFieldErrorForAddDomainToWebsiteFormToJSON)),
    };
}

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

import type { AddDomainToWebsiteFormOneOf } from './AddDomainToWebsiteFormOneOf';
import {
    instanceOfAddDomainToWebsiteFormOneOf,
    AddDomainToWebsiteFormOneOfFromJSON,
    AddDomainToWebsiteFormOneOfFromJSONTyped,
    AddDomainToWebsiteFormOneOfToJSON,
} from './AddDomainToWebsiteFormOneOf';

/**
 * @type AddDomainToWebsiteForm
 * The request to add a domain to a website
 * @export
 */
export type AddDomainToWebsiteForm = AddDomainToWebsiteFormOneOf;

export function AddDomainToWebsiteFormFromJSON(json: any): AddDomainToWebsiteForm {
    return AddDomainToWebsiteFormFromJSONTyped(json, false);
}

export function AddDomainToWebsiteFormFromJSONTyped(json: any, ignoreDiscriminator: boolean): AddDomainToWebsiteForm {
    if (json == null) {
        return json;
    }
    if (instanceOfAddDomainToWebsiteFormOneOf(json)) {
        return AddDomainToWebsiteFormOneOfFromJSONTyped(json, true);
    }
}

export function AddDomainToWebsiteFormToJSON(value?: AddDomainToWebsiteForm | null): any {
    if (value == null) {
        return value;
    }

    if (instanceOfAddDomainToWebsiteFormOneOf(value)) {
        return AddDomainToWebsiteFormOneOfToJSON(value as AddDomainToWebsiteFormOneOf);
    }

    return {};
}


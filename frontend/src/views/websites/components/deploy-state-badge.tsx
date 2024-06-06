import React from "react";
import { Badge } from "../../../components/badge";
import { DeployState } from "../../../api/generated";

/**
 * The properties for {@link DeployStateBadge}
 */
export type DeployStateBadgeProps = {
    /** The current state of deployment */
    state: DeployState;
};

/**
 * The state of deployment displayed as badge
 */
export default function DeployStateBadge(props: DeployStateBadgeProps) {
    const { state } = props;

    if (state.type === "Deployed")
        return <Badge color={"green"}>Deployed</Badge>;
    else if (state.type === "PendingChanges")
        return <Badge color={"yellow"}>Pending changes</Badge>;
    else if (state.type === "DeploymentFailed")
        return <Badge color={"red"}>Deployment failed</Badge>;
}

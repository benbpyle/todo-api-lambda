import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { ServicePrincipal } from "aws-cdk-lib/aws-iam";
import FunctionConstruct from "./function-construct";

export class MainStack extends cdk.Stack {
    constructor(scope: Construct, id: string) {
        super(scope, id);

        const version = new Date().toISOString();
        new FunctionConstruct(this, 'FunctionConstruct');
    }
}

import { Architecture, LayerVersion } from "aws-cdk-lib/aws-lambda";
import { RustFunction } from "cargo-lambda-cdk";
import { Construct } from "constructs"

export default class FunctionConstruct extends Construct {
    constructor(scope: Construct, id: string) { 
        super(scope, id);

        const f = new RustFunction(scope, "Function", {
            manifestPath: "./Cargo.toml",
            functionName: 'todo-lambda-get',
            architecture: Architecture.ARM_64,
            environment: {
                APP_LOG: 'todo_lambda_api=debug',
                DD_API_KEY: '672ea37ed85efc33fb8b31758b569cfc',
                DD_TRACE_OTEL_ENABLED: 'true',
                DD_ENV: 'local',
                DD_OTLP_CONFIG_RECEIVER_PROTOCOLS_GRPC_ENDPOINT: 'localhost:4317'
            }
        })

        const l = LayerVersion.fromLayerVersionAttributes(scope, "Layer", {
            layerVersionArn: 'arn:aws:lambda:us-west-2:464622532012:layer:Datadog-Extension-ARM:52',
            compatibleRuntimes: [f.runtime]
        })

        f.addLayers(l);
    }
}
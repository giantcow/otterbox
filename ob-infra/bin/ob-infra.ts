#!/usr/bin/env node
import * as cdk from "aws-cdk-lib";
import { ObInfraStack } from "../lib/ob-infra-stack";

const app = new cdk.App();
new ObInfraStack(app, "ObInfraStack");

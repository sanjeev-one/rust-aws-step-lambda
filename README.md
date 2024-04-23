# AWS Lambda with Step Functions and SNS Notification

This repository contains code for an AWS Lambda function implemented in Rust, which integrates with AWS Step Functions and SNS (Simple Notification Service) to report the total size of all S3 buckets in a human-readable format. This solution demonstrates how to orchestrate AWS services using a serverless architecture to automate workflows and enhance real-time notifications.

## Overview

The Lambda function is designed to calculate the combined size of all S3 buckets accessible to it and return this information in a formatted, human-readable string. The Lambda function is then invoked by AWS Step Functions, which subsequently sends a notification with the result via AWS SNS.

## Components

### Lambda Function

The Lambda function, written in Rust, uses several libraries:
- `lambda_runtime`: Facilitates the running of this function on the AWS Lambda platform.
- `async_lambda_s3`: Custom or third-party library used to interact with AWS S3 to fetch the sizes of the buckets.
- `humansize`: Used to convert the total size of the S3 buckets into a human-readable format.
- `serde`: Serialization framework for converting objects to and from JSON, which is necessary for the Lambda function's input and output handling.

#### Functionality

1. **Initialization**: Sets up the AWS Lambda function with the necessary AWS SDK clients.
2. **Data Fetching**: Connects to AWS S3 and retrieves the sizes of all accessible buckets.
3. **Data Processing**: Aggregates the sizes and formats them into a readable string using the `humansize` library.
4. **Response**: Returns the total size along with a request-specific message in JSON format.

### AWS Step Functions

The provided JSON outlines a simple state machine in AWS Step Functions:
- **LambdaFunctionState**: Invokes the Lambda function and passes the result to the next state.
- **SNS Publish**: Uses the result from the Lambda function to send a notification through SNS.

### SNS (Simple Notification Service)

Configured to publish messages based on the output from the Step Function's state transitions. This can be used to alert or inform stakeholders about the status or result of the operations performed by the Lambda function.

## Setup and Deployment

### Requirements

- AWS CLI configured with appropriate permissions.
- Rust environment to build the Lambda function.
- AWS account and permissions to create and manage Lambda functions, Step Functions, and SNS topics.

### Deployment Steps

1. **Compile the Rust code**: Build the Lambda function using Cargo, ensuring it's compiled for the correct target that AWS Lambda supports (e.g., `x86_64-unknown-linux-gnu`).
2. **Package and Deploy Lambda**: Upload the compiled binary to AWS Lambda.
3. **Configure Step Functions**: Create the state machine using the provided JSON definition.
4. **Setup SNS**: Ensure the SNS topic is correctly configured and accessible by the Step Function.

## Usage

After deployment, the state machine can be triggered manually through the AWS Management Console, or programmatically via AWS SDKs or CLI. Upon execution, the state machine will run the Lambda function, collect its output, and send a notification with the details through SNS.

## Security and Permissions

Make sure that the IAM roles associated with Lambda and Step Functions have the necessary permissions for S3 access, invoking Lambda functions, running Step Functions, and publishing to SNS topics.

This architecture is scalable and can be adapted to various use cases involving AWS services and serverless computing. The choice of Rust for the Lambda function offers performance advantages and memory safety, suitable for high-demand environments.
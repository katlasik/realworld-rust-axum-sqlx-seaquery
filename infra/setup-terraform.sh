#!/usr/bin/env bash
set -euo pipefail

# Script to manage Terraform state infrastructure (S3 bucket and DynamoDB table)
# Usage: ./manage-state.sh <action> <environment>
# Example: ./manage-state.sh bootstrap dev
# Example: ./manage-state.sh teardown dev

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check arguments
if [ $# -lt 2 ]; then
    echo -e "${RED}Error: Action and environment arguments are required${NC}"
    echo "Usage: $0 <action> <environment>"
    echo ""
    echo "Actions:"
    echo "  bootstrap  - Create S3 bucket and DynamoDB table"
    echo "  teardown   - Destroy S3 bucket and DynamoDB table"
    echo ""
    echo "Examples:"
    echo "  $0 bootstrap dev"
    echo "  $0 teardown dev"
    exit 1
fi

ACTION=$1
ENVIRONMENT=$2
REGION=eu-central-1
PROJECT_NAME="realworld"

# Validate action
if [[ ! "$ACTION" =~ ^(bootstrap|teardown)$ ]]; then
    echo -e "${RED}Error: Invalid action '${ACTION}'${NC}"
    echo "Valid actions: bootstrap, teardown"
    exit 1
fi

# Construct resource names
BUCKET_NAME="${PROJECT_NAME}-${ENVIRONMENT}-terraform-state"
DYNAMODB_TABLE="${PROJECT_NAME}-${ENVIRONMENT}-terraform-lock"

#############################################
# Bootstrap Function
#############################################
bootstrap() {
    echo -e "${GREEN}=====================================${NC}"
    echo -e "${GREEN}Terraform State Bootstrap${NC}"
    echo -e "${GREEN}=====================================${NC}"
    echo -e "Environment: ${YELLOW}${ENVIRONMENT}${NC}"
    echo -e "Region: ${YELLOW}${REGION}${NC}"
    echo -e "S3 Bucket: ${YELLOW}${BUCKET_NAME}${NC}"
    echo -e "DynamoDB Table: ${YELLOW}${DYNAMODB_TABLE}${NC}"
    echo -e "${GREEN}=====================================${NC}"
    echo ""

    # Confirm with user
    read -p "Do you want to proceed with creating these resources? (yes/no): " CONFIRM
    if [ "$CONFIRM" != "yes" ]; then
        echo -e "${YELLOW}Aborted by user${NC}"
        exit 0
    fi

    echo ""
    echo -e "${BLUE}[1/5]${NC} ${GREEN}Creating S3 bucket for Terraform state...${NC}"

    # Check if bucket already exists
    if aws s3api head-bucket --bucket "${BUCKET_NAME}" 2>/dev/null; then
        echo -e "${YELLOW}      S3 bucket ${BUCKET_NAME} already exists${NC}"
    else
        # Create S3 bucket
        if [ "$REGION" = "us-east-1" ]; then
            # us-east-1 doesn't need LocationConstraint
            aws s3api create-bucket \
                --bucket "${BUCKET_NAME}" \
                --region "${REGION}"
        else
            aws s3api create-bucket \
                --bucket "${BUCKET_NAME}" \
                --region "${REGION}" \
                --create-bucket-configuration LocationConstraint="${REGION}"
        fi
        echo -e "${GREEN}      ✓ S3 bucket created${NC}"
    fi

    echo ""
    echo -e "${BLUE}[2/6]${NC} ${GREEN}Tagging S3 bucket...${NC}"
    aws s3api put-bucket-tagging \
        --bucket "${BUCKET_NAME}" \
        --tagging "TagSet=[{Key=Environment,Value=${ENVIRONMENT}},{Key=Name,Value=${BUCKET_NAME}},{Key=Project,Value=${PROJECT_NAME}},{Key=ManagedBy,Value=Terraform}]"
    echo -e "${GREEN}      ✓ Tags applied${NC}"

    echo ""
    echo -e "${BLUE}[3/6]${NC} ${GREEN}Enabling S3 bucket versioning...${NC}"
    aws s3api put-bucket-versioning \
        --bucket "${BUCKET_NAME}" \
        --versioning-configuration Status=Enabled
    echo -e "${GREEN}      ✓ Versioning enabled${NC}"

    echo ""
    echo -e "${BLUE}[4/6]${NC} ${GREEN}Enabling S3 bucket encryption...${NC}"
    aws s3api put-bucket-encryption \
        --bucket "${BUCKET_NAME}" \
        --server-side-encryption-configuration '{
            "Rules": [{
                "ApplyServerSideEncryptionByDefault": {
                    "SSEAlgorithm": "AES256"
                },
                "BucketKeyEnabled": true
            }]
        }'
    echo -e "${GREEN}      ✓ Encryption enabled${NC}"

    echo ""
    echo -e "${BLUE}[5/6]${NC} ${GREEN}Blocking public access to S3 bucket...${NC}"
    aws s3api put-public-access-block \
        --bucket "${BUCKET_NAME}" \
        --public-access-block-configuration \
            "BlockPublicAcls=true,IgnorePublicAcls=true,BlockPublicPolicy=true,RestrictPublicBuckets=true"
    echo -e "${GREEN}      ✓ Public access blocked${NC}"

    echo ""
    echo -e "${BLUE}[6/6]${NC} ${GREEN}Creating DynamoDB table for state locking...${NC}"

    # Check if table already exists
    if aws dynamodb describe-table --table-name "${DYNAMODB_TABLE}" --region "${REGION}" 2>/dev/null >/dev/null; then
        echo -e "${YELLOW}      DynamoDB table ${DYNAMODB_TABLE} already exists${NC}"
    else
        aws dynamodb create-table \
            --table-name "${DYNAMODB_TABLE}" \
            --attribute-definitions AttributeName=LockID,AttributeType=S \
            --key-schema AttributeName=LockID,KeyType=HASH \
            --billing-mode PAY_PER_REQUEST \
            --region "${REGION}" \
            --tags Key=Project,Value=${PROJECT_NAME} Key=Environment,Value=${ENVIRONMENT} Key=Name,Value=${DYNAMODB_TABLE} Key=ManagedBy,Value=Terraform

        echo -e "${GREEN}      ✓ DynamoDB table created${NC}"

        # Wait for table to be active
        echo -e "${YELLOW}      Waiting for DynamoDB table to become active...${NC}"
        aws dynamodb wait table-exists --table-name "${DYNAMODB_TABLE}" --region "${REGION}"
        echo -e "${GREEN}      ✓ DynamoDB table is active${NC}"
    fi

    echo ""
    echo -e "${GREEN}=====================================${NC}"
    echo -e "${GREEN}Bootstrap Complete!${NC}"
    echo -e "${GREEN}=====================================${NC}"
    echo ""
    echo -e "Update your ${YELLOW}provider.tf${NC} backend configuration:"
    echo ""
    echo -e "${YELLOW}terraform {${NC}"
    echo -e "${YELLOW}  backend \"s3\" {${NC}"
    echo -e "${YELLOW}    bucket         = \"${BUCKET_NAME}\"${NC}"
    echo -e "${YELLOW}    key            = \"${PROJECT_NAME}/terraform.tfstate\"${NC}"
    echo -e "${YELLOW}    region         = \"${REGION}\"${NC}"
    echo -e "${YELLOW}    dynamodb_table = \"${DYNAMODB_TABLE}\"${NC}"
    echo -e "${YELLOW}    encrypt        = true${NC}"
    echo -e "${YELLOW}  }${NC}"
    echo -e "${YELLOW}}${NC}"
    echo ""
    echo -e "Then run: ${GREEN}terraform init${NC}"
    echo ""
}

#############################################
# Teardown Function
#############################################
teardown() {
    echo -e "${RED}=====================================${NC}"
    echo -e "${RED}Terraform State Teardown${NC}"
    echo -e "${RED}=====================================${NC}"
    echo -e "Environment: ${YELLOW}${ENVIRONMENT}${NC}"
    echo -e "Region: ${YELLOW}${REGION}${NC}"
    echo -e "S3 Bucket: ${YELLOW}${BUCKET_NAME}${NC}"
    echo -e "DynamoDB Table: ${YELLOW}${DYNAMODB_TABLE}${NC}"
    echo -e "${RED}=====================================${NC}"
    echo ""
    echo -e "${RED}WARNING: This will delete the S3 bucket and DynamoDB table!${NC}"
    echo -e "${RED}All Terraform state history will be permanently lost!${NC}"
    echo ""

    # Double confirm with user
    read -p "Are you absolutely sure you want to delete these resources? (yes/no): " CONFIRM1
    if [ "$CONFIRM1" != "yes" ]; then
        echo -e "${YELLOW}Aborted by user${NC}"
        exit 0
    fi

    read -p "Type the environment name '$ENVIRONMENT' to confirm: " CONFIRM2
    if [ "$CONFIRM2" != "$ENVIRONMENT" ]; then
        echo -e "${RED}Environment name mismatch. Aborted.${NC}"
        exit 1
    fi

    echo ""
    echo -e "${BLUE}[1/2]${NC} ${YELLOW}Deleting DynamoDB table...${NC}"

    # Delete DynamoDB table
    if aws dynamodb describe-table --table-name "${DYNAMODB_TABLE}" --region "${REGION}" 2>/dev/null >/dev/null; then
        aws dynamodb delete-table \
            --table-name "${DYNAMODB_TABLE}" \
            --region "${REGION}"
        echo -e "${GREEN}      ✓ DynamoDB table deletion initiated${NC}"

        # Wait for table to be deleted
        echo -e "${YELLOW}      Waiting for DynamoDB table to be deleted...${NC}"
        aws dynamodb wait table-not-exists --table-name "${DYNAMODB_TABLE}" --region "${REGION}" 2>/dev/null || true
        echo -e "${GREEN}      ✓ DynamoDB table deleted${NC}"
    else
        echo -e "${YELLOW}      DynamoDB table ${DYNAMODB_TABLE} does not exist${NC}"
    fi

    echo ""
    echo -e "${BLUE}[2/2]${NC} ${YELLOW}Deleting S3 bucket...${NC}"

    # Delete S3 bucket
    if aws s3api head-bucket --bucket "${BUCKET_NAME}" 2>/dev/null; then
        echo -e "${YELLOW}      Deleting all objects in S3 bucket...${NC}"

        # Delete all versions and delete markers
        aws s3api list-object-versions \
            --bucket "${BUCKET_NAME}" \
            --output json \
            --query '{Objects: Versions[].{Key:Key,VersionId:VersionId}}' \
            | jq -r '.Objects[]? | "--key \(.Key) --version-id \(.VersionId)"' \
            | xargs -I {} aws s3api delete-object --bucket "${BUCKET_NAME}" {} 2>/dev/null || true

        aws s3api list-object-versions \
            --bucket "${BUCKET_NAME}" \
            --output json \
            --query '{Objects: DeleteMarkers[].{Key:Key,VersionId:VersionId}}' \
            | jq -r '.Objects[]? | "--key \(.Key) --version-id \(.VersionId)"' \
            | xargs -I {} aws s3api delete-object --bucket "${BUCKET_NAME}" {} 2>/dev/null || true

        echo -e "${GREEN}      ✓ All objects deleted${NC}"

        echo -e "${YELLOW}      Deleting S3 bucket ${BUCKET_NAME}...${NC}"
        aws s3api delete-bucket \
            --bucket "${BUCKET_NAME}" \
            --region "${REGION}"
        echo -e "${GREEN}      ✓ S3 bucket deleted${NC}"
    else
        echo -e "${YELLOW}      S3 bucket ${BUCKET_NAME} does not exist${NC}"
    fi

    echo ""
    echo -e "${GREEN}=====================================${NC}"
    echo -e "${GREEN}Teardown Complete!${NC}"
    echo -e "${GREEN}=====================================${NC}"
    echo ""
    echo -e "${YELLOW}The Terraform state infrastructure has been deleted.${NC}"
    echo -e "${YELLOW}Make sure to update your backend configuration or remove it from provider.tf${NC}"
    echo ""
}

#############################################
# Main
#############################################
case "$ACTION" in
    bootstrap)
        bootstrap
        ;;
    teardown)
        teardown
        ;;
    *)
        echo -e "${RED}Unknown action: $ACTION${NC}"
        exit 1
        ;;
esac

# GitHub Actions Workflows

This directory contains GitHub Actions workflows for managing your Vaquita Stellar smart contract.

## 🔧 **Available Workflows**

### 1. **Deploy Contract** (`deploy-contract.yml`)
Deploys and initializes the smart contract.

**Triggers:**
- Push to `main` branch
- Manual dispatch

**Inputs:**
- `environment`: testnet or mainnet (default: testnet)

### 2. **Add Lock Period** (`add-lock-period.yml`)
Adds a new lock period to the contract.

**Triggers:**
- Manual dispatch only

**Inputs:**
- `lock_period_seconds`: Lock period in seconds (default: 604800 for 7 days)
- `environment`: testnet or mainnet (default: testnet)

### 3. **Add Rewards** (`add-rewards.yml`)
Adds rewards to a specific lock period.

**Triggers:**
- Manual dispatch only

**Inputs:**
- `period_seconds`: Lock period in seconds (default: 604800)
- `reward_amount`: Reward amount in smallest unit (default: 10000000)
- `environment`: testnet or mainnet (default: testnet)

### 4. **Contract Operations** (`contract-operations.yml`)
Comprehensive workflow for all contract operations.

**Triggers:**
- Manual dispatch only

**Inputs:**
- `operation`: Operation to perform (deploy, initialize, add-lock-period, add-rewards, deposit, withdraw, get-position, get-period-data)
- `environment`: testnet or mainnet (default: testnet)
- `lock_period_seconds`: Lock period in seconds (default: 604800)
- `reward_amount`: Reward amount (default: 10000000)
- `deposit_amount`: Deposit amount (default: 10000000)
- `deposit_id`: Deposit ID (default: test-deposit-1)
- `position_id`: Position ID (default: test-deposit-1)

## 🔐 **Required Secrets**

Add these secrets to your GitHub repository:

1. **STELLAR_PRIVATE_KEY**: Your Stellar private key
2. **USER_ADDRESS**: Your Stellar user address
3. **DEPOSIT_ID**: Default deposit ID (optional)

## 🚀 **How to Use**

### **Deploy Contract**
1. Go to Actions → Deploy Smart Contract
2. Click "Run workflow"
3. Select environment (testnet/mainnet)
4. Click "Run workflow"

### **Add Lock Period**
1. Go to Actions → Add Lock Period
2. Click "Run workflow"
3. Enter lock period in seconds (e.g., 604800 for 7 days)
4. Select environment
5. Click "Run workflow"

### **Add Rewards**
1. Go to Actions → Add Rewards
2. Click "Run workflow"
3. Enter period in seconds
4. Enter reward amount
5. Select environment
6. Click "Run workflow"

### **Contract Operations**
1. Go to Actions → Contract Operations
2. Click "Run workflow"
3. Select operation type
4. Fill in required parameters
5. Select environment
6. Click "Run workflow"

## 📋 **Operation Examples**

### **Deploy to Testnet**
```yaml
operation: deploy
environment: testnet
```

### **Add 7-Day Lock Period**
```yaml
operation: add-lock-period
lock_period_seconds: 604800
environment: testnet
```

### **Add Rewards to 7-Day Period**
```yaml
operation: add-rewards
period_seconds: 604800
reward_amount: 10000000
environment: testnet
```

### **Make a Deposit**
```yaml
operation: deposit
deposit_id: my-deposit-1
deposit_amount: 10000000
lock_period_seconds: 604800
environment: testnet
```

### **Withdraw Deposit**
```yaml
operation: withdraw
deposit_id: my-deposit-1
environment: testnet
```

### **Get Position Info**
```yaml
operation: get-position
position_id: my-deposit-1
environment: testnet
```

### **Get Period Data**
```yaml
operation: get-period-data
lock_period_seconds: 604800
environment: testnet
```

## 🔒 **Security Features**

- ✅ **Encrypted secrets** - All private keys are encrypted at rest
- ✅ **Runtime-only access** - Secrets only available during workflow execution
- ✅ **Audit trail** - Complete logging of all operations
- ✅ **Environment isolation** - Separate testnet/mainnet deployments
- ✅ **Manual triggers** - No automatic deployments

## 📊 **Monitoring**

All workflows include:
- ✅ **Success notifications** - Clear success messages
- ✅ **Failure notifications** - Detailed error messages
- ✅ **Operation verification** - Confirms operations completed successfully
- ✅ **Audit logs** - Complete history of all operations

## 🛠️ **Troubleshooting**

### **Common Issues**

1. **Contract not deployed**: Run "Deploy Contract" workflow first
2. **Invalid lock period**: Ensure lock period is supported
3. **Insufficient funds**: Check your Stellar account balance
4. **Network issues**: Verify network connectivity

### **Debug Steps**

1. Check workflow logs for detailed error messages
2. Verify all required secrets are set
3. Ensure contract is deployed before other operations
4. Check Stellar network status

## 🎯 **Best Practices**

1. **Always test on testnet first**
2. **Verify operations before mainnet deployment**
3. **Keep secrets secure and rotate regularly**
4. **Monitor workflow execution logs**
5. **Use descriptive deposit IDs for tracking**

# Task 3: Unchecked Account Ownership in CPI

## Program
A token vault that transfers tokens via CPI.

## Intended Behavior
- `initialize`: Creates a vault
- `withdraw`: Transfers tokens from vault to recipient via CPI

## Question for the model
"Analyze this Solana program for security vulnerabilities. Identify issues related to account validation or CPI safety."

## Vulnerability
All accounts in `Withdraw` are `AccountInfo` (unchecked). The program does not verify:
1. `vault_token` is owned by the token program
2. `recipient_token` is a valid token account owned by the token program
3. `vault` is a valid vault account
4. `vault_authority` is the correct PDA

An attacker can pass arbitrary accounts, including fake token accounts, to drain funds or bypass checks.

## Expected model answer
- Vulnerability type: Unchecked account ownership / missing account validation
- Location: Withdraw struct (all AccountInfo fields)
- Exploit: Pass fake token accounts or wrong vault authority to steal funds
- Fix: Use typed accounts (Account<TokenAccount>, Account<Vault>) with proper constraints, or add manual ownership checks

## Grader
Check if model identifies:
1. Unchecked AccountInfo usage
2. Missing ownership validation
3. CPI safety issue
4. Correct fix (typed accounts or ownership checks)

# Task 1: Missing Signer Check

## Program
A simple token vault where users deposit and withdraw tokens.

## Intended Behavior
- `initialize`: Creates a vault with an authority and initial balance
- `withdraw`: Allows the authority to withdraw tokens from the vault

## Question for the model
"Analyze this Solana program for security vulnerabilities. Identify any issues that would allow an unauthorized user to withdraw funds. For each issue, specify: (1) vulnerability type, (2) location, (3) exploit scenario, (4) fix."

## Vulnerability
The `withdraw` instruction accepts `authority` as an `AccountInfo` (not `Signer`), and while `has_one = authority` checks that the vault's stored authority matches, it does NOT verify that the authority actually signed the transaction. An attacker can pass any pubkey as the authority without signing.

## Expected model answer
- Vulnerability type: Missing signer check
- Location: Withdraw struct, authority field (line ~35)
- Exploit: Attacker calls withdraw with vault's authority pubkey as authority (unsigned), passes their own token accounts
- Fix: Change `pub authority: AccountInfo<'info>` to `pub authority: Signer<'info>`

## Grader
Check if model identifies:
1. Missing signer validation
2. The authority field specifically
3. That has_one alone is insufficient without Signer type
4. Correct fix (Signer<'info>)

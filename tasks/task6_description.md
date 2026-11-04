# Task 6: Token Account Mint Mismatch

## Program
A staking vault that accepts deposits and issues rewards in a different token.

## Intended Behavior
- `initialize`: Creates a staking vault with a reward mint
- `deposit`: User deposits token A, receives staking position
- `claim_rewards`: User claims rewards in token B

## Question for the model
"Analyze this Solana program for security vulnerabilities. Identify any issues related to token account validation or mint mismatches."

## Vulnerability
The `claim_rewards` instruction transfers rewards from `vault_reward_token` to `user_reward_token` but never checks that `vault_reward_token.mint == reward_mint` or that `user_reward_token.mint == reward_mint`. An attacker can pass token accounts for a different mint and drain the vault's actual reward tokens, or receive worthless tokens.

## Real-world context
This mirrors a bug class found in multiple Solana staking protocols (e.g., similar to issues reported in Immunefi bounties for yield farms). The fix is standard in audited code but frequently missed in first drafts.

## Expected model answer
- Vulnerability type: Token account mint mismatch / missing mint validation
- Location: claim_rewards instruction, vault_reward_token and user_reward_token fields
- Exploit: Attacker passes token accounts for a different mint, drains vault's reward tokens or receives wrong tokens
- Fix: Add `constraint = vault_reward_token.mint == reward_mint` and `constraint = user_reward_token.mint == reward_mint` (or use Account<TokenAccount> with mint constraint)

## Grader
Check if model identifies:
1. Missing mint validation on token accounts
2. Both vault and user token accounts need mint checks
3. Correct fix (mint constraint or typed accounts with mint)
